rust
// `dyn T` always has a hidden lifetime that the compiler need to guess
type Alias1<'a, T> = &'a mut dyn FnMut() -> T + 'a; // because it is behind a &'a ref, compiler guess the `dyn T` has `'a`
type Alias2<T> = dyn FnMut() -> T + 'static; // No other lifetime involved, the compiler can only guess `'static`
