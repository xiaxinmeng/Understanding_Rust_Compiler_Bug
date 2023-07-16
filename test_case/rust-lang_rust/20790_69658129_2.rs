 rust
fn main() {
    let x = () + (); //~ ERROR binary operation

    // this shouldn't have a flow-on error:
    // japaric: and it doesn't with the either version of for loops
    for _ in x {}
}
