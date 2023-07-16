 rust
macro_rules! foo { () => { break 'x; } } //~ ERROR use of undeclared label `x`

fn main() {
    'x: loop { foo!() }
}
