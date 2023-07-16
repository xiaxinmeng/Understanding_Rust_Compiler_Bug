 rust
macro_rules! foo {
    ($v:ident) => (
        let $v = 3;
        println!("{}", $v); // If you comment this then it's fine.
    );
}

fn main() {
    foo!(x);
    println!("{}", x);
}
