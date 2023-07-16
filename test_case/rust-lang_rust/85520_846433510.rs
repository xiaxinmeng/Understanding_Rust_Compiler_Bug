rust
fn foo<const N: usize>(_: [i32; N], _: impl Copy) {}

struct Boo<const N: i32>{ ... }
fn bar<const N: i32>(_: Boo<i32>, _: impl Debug) {}
