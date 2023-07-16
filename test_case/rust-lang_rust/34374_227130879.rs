 rust
fn main() {
    let x = 0;
    macro_rules! foo { () => {
        let x = 1;
        println!("{}", x); // prints `0` on stable, should print `1`
    } }
    m!();
}
