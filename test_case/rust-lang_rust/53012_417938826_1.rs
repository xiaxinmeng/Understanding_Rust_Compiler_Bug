rust
macro_rules! not_noop {
    ( pub fn $ident:ident ( $($args:tt)* ) {} ) => {
        pub fn $ident(_: usize, $($args)*) {}
    }
}

pub struct Foo;

impl Foo {
    not_noop! {
        pub fn method() {}
    }
}

pub fn call_foo_method() {
    // I can no longer call Foo::method(). I need to pass in a usize.
    Foo::method(0usize);
}
