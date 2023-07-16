RUST
#[rustfmt::skip]
macro_rules! f {
    (
        /// doc
    ) => { println!("/// doc"); };
    (#[$m:meta]) => { println!("#[$m:meta]"); };
    () => { println!("()"); };
}

fn main() {
    f!(
        /// doc
    );
    f!();
}
