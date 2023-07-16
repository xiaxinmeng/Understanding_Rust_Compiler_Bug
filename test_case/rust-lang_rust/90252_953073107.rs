rust
struct A {
    a: u8,
}

fn some_fn() {
    let mut widths = vec![A {a: 0}];
    let mut stack = vec![];

    let a = stack.pop().unwrap();
    let idx = 0usize;
    let b = &widths[a].a; // <-- (1)

    stack.push(idx); 
}
