 rust
union A {
    x: Box<i32>,
    y: Box<i64>,
}

union B {
    x: Box<i32>,
    y: Box<i64>,
    z: (),
}
