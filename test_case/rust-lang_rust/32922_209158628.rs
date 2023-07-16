 rust
macro_rules! foo { () => {
    let x = 1;
    macro_rules! bar { () => {x} }
    println!("{}", bar!());
}}

fn main() {
    foo! {};
}
