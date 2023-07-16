rust
struct Foo;

macro_rules! test {
    (inner, $tr: path) => {
        impl $tr for Foo {}
    };
    ($cr: path) => { test!(inner, $cr::Buf); }
}

test!(bytes);
