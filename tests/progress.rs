use fire_rs::fire;
#[test]
fn no_args() {
    #[fire]
    fn foo() {}
    let app = foo_app();
    foo_slice(app, &["demo"])
}
#[test]
fn two_args() {
    #[fire]
    fn foo(a: i32, b: i64) {}
    let app = foo_app();
    foo_slice(app, &["demo", "2", "6"])
}

#[test]
fn with_name() {
    #[fire]
    fn foo(int: i32, long: i64) {}
    let app = foo_app();
    foo_slice(app, &["demo", "--int", "4", "--long", "8"])
}

#[test]
fn omit_rest_args() {
    #[fire]
    fn foo(int: i32, long: i64) {}
    let app = foo_app();
    foo_slice(app, &["demo", "2", "4", "9"])
}
