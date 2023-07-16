 rust
#[inline(never)]
fn out<T>(x: T) {
    printfln!("%?", x);
}

#[inline(never)]
fn pass<T>(x: T) -> T {
    printfln!("%?", x);
    x
}

fn main() {
    for (x, y) in range(0, pass(10)).zip(range(0, 20)) {
        out(x);
        out(y);
    }
}
