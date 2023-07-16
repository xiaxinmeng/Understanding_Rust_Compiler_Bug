 rust
const N: usize = 2;
const ARR: [i32; N] = [42; N];
const X: bool = (ARR[0] == 99) && (ARR[99] == 1337);

fn main() {
    let _ = X;
}
