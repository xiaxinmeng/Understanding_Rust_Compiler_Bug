 rust
fn foo(_: (), x: int) -> int { x }

fn main() {
    let mut a = 2i;
    println!("{}", a * foo(a = 5, a)); // 25
    a = 2i;
    println!("{}", foo((), a) * foo(a = 5, a)); // 10
}
