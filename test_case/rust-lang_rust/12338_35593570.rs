 rust
macro_rules! foo {
    () => { break 'x; }
}

pub fn main() {
    'x: for _ in range(0,1) { foo!() } //~ ERROR use of undeclared label `x`
}
