 rust
macro_rules! macro_ {
    ($t:ty) => {
        impl $t {
            fn bar() -> $t {
                unimplemented!();
            }
            fn foo() -> $t {
                <$t>::bar()
            }
        }
    }
}

struct Foo;
macro_!(Foo);
