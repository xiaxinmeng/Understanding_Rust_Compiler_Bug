 rust
fn main() {
    println!("{}", match 1i {
        n if n > 0  => "greater",
        n if n < 0  => "less",
        n if n == 0 => "equal",
    });
}
// error: non-exhaustive patterns: `_` not covered [E0004]
