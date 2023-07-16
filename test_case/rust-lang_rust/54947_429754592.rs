rust
const fn foo(i: usize) -> i32 {
    [0, 1, 2][(i % 10) + 3]
}
