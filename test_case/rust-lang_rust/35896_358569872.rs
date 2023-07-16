rust
#![feature(decl_macro)]

mod foo {
    pub fn f() {} // (1)

    pub macro m($arg:expr) {
        f(); // This resolves to (1)
        mod bar {
            fn f() { $arg }
        }
    }
}

fn main() {
    fn f() {} // (2) (note -- no conflict error with (1))
    foo::m!(f()); // The `f` argument resolves to (2) even though $arg is in a weird place
}
