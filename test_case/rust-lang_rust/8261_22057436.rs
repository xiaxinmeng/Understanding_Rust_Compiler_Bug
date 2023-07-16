 rust
#[inline(never)]
fn out<T>(x: T) {
    printfln!("%?", x);
}

fn main() {
    for (x, y) in range(0, 10).zip(range(0, 20)) {
        out(x);
        out(y);
    }
}
