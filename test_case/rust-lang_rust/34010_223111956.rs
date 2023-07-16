 rust
struct Foo {
    x: i32,
    mac!(), //< say `mac!()` expands to `y: i32`
}
