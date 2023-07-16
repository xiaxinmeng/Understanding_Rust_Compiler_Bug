 rust
fn foo(_: (), x: int) -> int {
    x
}

fn main() {
    let mut a = 1;
    {
        a * foo(a = 3, a)
    };
}
