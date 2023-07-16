rust
fn foo<const M: usize>() -> [(); 3 * M];
let _: [(); 3 * 2 + N] foo::<{ 2 + N }>();
