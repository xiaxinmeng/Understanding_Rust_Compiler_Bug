rust
enum E {
    A(i32, i32, i32, i32),
    B { x: i32, y: i32 },
}
fn foo(
    (a, .., b): (i32, i32, i32, i32, i32),
    (E::A {0: c, 1: d, 2: 1, 3: 1} | E::A(.., c, d) | E::B { x: c, y: d }): E,
    &([1, .., e, f] | [e, f, ..]): &[i32; 6],
) {
    // use a, b, c, d, e, f here
}
