rust
macro_rules! wha {
    ($lhs:expr ; $rhs:expr) => ($lhs $rhs)
    //~^ ERROR macro expansion ignores token `*3` and any following
}
fn main() {
    let z = wha!( 4 ; * 3 );
    println!("{}", z);
}
