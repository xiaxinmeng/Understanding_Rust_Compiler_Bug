rust
type Alias2<'b, T> = dyn FnMut() -> T + 'b;
