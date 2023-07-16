rust
struct A {
    a: u8,
}

fn some_fn() {
    let mut widths = vec![A {a: 0}];
    let mut stack = vec![];

    let a = stack.pop().unwrap();
    let idx = 0usize;
    let b: String = widths[a]; // <-- (1)

    stack.push(idx); // <-- (2)
}

fn main() {}
