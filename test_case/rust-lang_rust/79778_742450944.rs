rust
struct Bar<T: Cake>(T);

fn foo<N: Unsigned>(val: Val) -> Bar<N> { .. }
