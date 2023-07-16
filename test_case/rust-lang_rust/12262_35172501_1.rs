 rust
macro_rules! foo { ($e: expr) => { 'x: loop { $e } } }

fn main() {
    foo!(break 'x); //~ ERROR use of undeclared label `x`
}
