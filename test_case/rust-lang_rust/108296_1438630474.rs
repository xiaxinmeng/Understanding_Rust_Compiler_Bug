rs
#[deny(dead_code)]
#[allow(unused_must_use)]
pub fn foo() {
    || {
        fn a() {}
    };
}
