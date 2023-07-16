rust
struct S {
    y: usize,
}

fn foo(x: u32) {
    let _ = bar(S { y: x as usize });
}

fn bar(x: S) -> usize {
    x.y
}

fn main() {
    foo(42);
}
