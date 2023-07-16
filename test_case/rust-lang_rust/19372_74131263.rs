 rust
macro_rules! borked {
    () => {
        #[derive(Debug)] pub struct Good;
        #[cfg_attr(not(test), derive(Debug))] pub struct Bad;
    }
}

borked!();

fn main() {
    println!("{:?}", Good);
    println!("{:?}", Bad);
}
