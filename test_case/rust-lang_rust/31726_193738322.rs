 rust
fn f() {
    mod foo { pub fn f() {} }
    fn g() {
        mod foo { pub fn g() {} }
        foo::f();
    }
}
