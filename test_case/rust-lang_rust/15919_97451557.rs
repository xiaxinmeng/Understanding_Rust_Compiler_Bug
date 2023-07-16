 rust
fn main() {
    let x = [0usize; 9223372036854775808];
//~^ error: the type `[usize; 9223372036854775808]` is too big for the current architecture
}
