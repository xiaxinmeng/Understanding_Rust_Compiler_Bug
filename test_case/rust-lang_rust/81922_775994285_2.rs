rust
macro_rules! foo {
    ($i:ident) => {
        let $i = 42;
    };
}

#[allow(unused_variables)]
fn main() {
    foo!(a);
}
