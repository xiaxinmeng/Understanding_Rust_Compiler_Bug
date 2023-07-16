rust
macro_rules! foo {
    ($i:ident) => {
        let $i = 42;
    };
}

fn main() {
    #[allow(unused_variables)]
    foo!(a);
}
