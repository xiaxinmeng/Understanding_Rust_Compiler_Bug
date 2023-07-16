 rust
fn main() {
    let a = Test { n: 1 };
    {
        let a = Test { n: 2};
        println!("Hello, world!");
    }
    // Nothing here, should the first a have already been dropped?
}
