 rust
fn foo(_: (), x: isize) -> isize { x }

fn main() {
    let mut a = 2isize;
    println!("{}", a * foo(a = 5, a)); // 10
    a = 2isize;
    println!("{}", foo((), a) * foo(a = 5, a)); // 10
}
