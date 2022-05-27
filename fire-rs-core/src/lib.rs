//! Core crate for proc-macro
extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::ops::Deref;
use syn::{FnArg, Ident, Item, Pat};

/// This macro will generate 4 functions to create cli.
///
/// Suppose the fired function is called `foo`, then the 4 functions are:
/// - `foo_app`: parse the inputs of `foo` function, return a `clap::App`
///
/// - `foo_input`: receive the `clap::App` as input, match with stdin and call the `foo` function
///
/// - `foo_slice`: similar to `foo_input`, but match with the given slice
///
/// - `foo_fire`: simply combine the `foo_app` and `foo_input` for convenience
#[proc_macro_attribute]
pub fn fire(_head: TokenStream, body: TokenStream) -> TokenStream {
    match syn::parse::<Item>(body).unwrap() {
        Item::Fn(func) => {
            let ident = &func.sig.ident;
            let inputs = &func.sig.inputs;
            let args = inputs
                .iter()
                .map(|fnc| match fnc {
                    FnArg::Typed(pt) => match pt.pat.deref() {
                        Pat::Ident(pat_ident) => &pat_ident.ident,
                        _ => panic!("complex pattern is not supported!"),
                    },
                    _ => panic!("associated function is not supported!"), // 排除self参数
                })
                .collect::<Vec<_>>();
            let app_fn = gen_app(ident, &args);
            let match_fn = gen_match(ident, &args);
            let fire_ident = format_ident!("{}_fire", ident);
            let stdin_ident = format_ident!("{}_stdin", ident);
            let func_ident = format_ident!("{}_app", ident);

            let gen = quote! {
                use fire_rs::{App, Arg};
                use std::ffi::OsString;
                #func
                #app_fn
                #match_fn
                fn #fire_ident() {
                    let app = #func_ident();
                    #stdin_ident(app);
                }
            };
            gen.into()
        }
        _ => panic!("gg"),
    }
}
// app部分
fn gen_app(ident: &Ident, args: &[&Ident]) -> TokenStream2 {
    let func_ident = format_ident!("{}_app", ident);
    quote! {
        fn #func_ident<'a, 'b>() -> App<'a, 'b> {
            let mut app = App::new("demo")
                .arg(Arg::with_name("args")
                    .takes_value(true)
                    .multiple(true));
                #(
                    let args = Arg::with_name(stringify!(#args))
                        .takes_value(true)
                        .long(stringify!(#args)); // 利用stringify将ident转化为字符串
                    app = app.arg(args);
                )*
            app
        }
    }
}
// 对输入的处理逻辑
fn gen_match(ident: &Ident, args: &[&Ident]) -> TokenStream2 {
    // 通用处理逻辑
    let common = quote! {
        let mut ifs = false; // 是否出现命名参数
        #(
            ifs = ifs || matches.is_present(stringify!(#args));
        )*
        if ifs {
            #ident(#(matches.value_of(stringify!(#args)).unwrap().parse().unwrap()),*);
        }
        else {
            let mut v = matches.values_of("args").unwrap_or_default();
            #ident(#(
                {
                    let #args = 0; // 为了能迭代，让args随便出现一下
                    v.next().unwrap().parse().unwrap()//块表达式的值
                }
            ),*);
        }
    };
    let stdin_ident = format_ident!("{}_stdin", ident);
    let slice_ident = format_ident!("{}_slice", ident);
    quote! {
        fn #stdin_ident(app: App) {
            let matches = app.get_matches();
            #common
        }
        fn #slice_ident<I, T>(app: App, itr: I)  where
            I: IntoIterator<Item = T>,
            T: Into<OsString> + Clone
        {
            let matches = app.get_matches_from(itr);
            #common
        }
    }
}
