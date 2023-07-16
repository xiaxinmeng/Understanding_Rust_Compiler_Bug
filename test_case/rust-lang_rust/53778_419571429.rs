rust
macro_rules! gen_outer_inner {() => {
    macro_rules! m {() => ()} // Outer
    macro_rules! m {() => ()} // Inner
}}

fn main() {
    gen_outer_inner!();
    m!(); // Invoc
}
