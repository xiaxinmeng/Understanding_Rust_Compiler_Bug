plain
   Compiling sha2 v0.9.1
   Compiling rustc-rayon v0.3.1
   Compiling matchers v0.0.1
   Compiling tempfile v3.1.0
error[E0309]: the parameter type `R` may not live long enough
    |
    |
313 |         fn with<'f, T, U, R>(
    |                           - help: consider adding an explicit lifetime bound...: `R: 'f`
...
316 |         ) -> impl FnMut(T) -> R + 'f {
    |              ^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `[closure@/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.3.1/src/iter/map_with.rs:317:13: 317:37]` will meet its required lifetime bounds

error[E0311]: the parameter type `T` may not live long enough
    |
    |
103 |         fn ok<T, E>(saved: &Mutex<Option<E>>) -> impl Fn(Result<T, E>) -> Option<T> + '_ {
    |
    |
note: the parameter type `T` must be valid for the anonymous lifetime defined on the function body at 103:28...
    |
    |
103 |         fn ok<T, E>(saved: &Mutex<Option<E>>) -> impl Fn(Result<T, E>) -> Option<T> + '_ {
    |                            ^^^^^^^^^^^^^^^^^
note: ...so that the type `[closure@/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.3.1/src/result.rs:104:13: 117:14]` will meet its required lifetime bounds
    |
    |
103 |         fn ok<T, E>(saved: &Mutex<Option<E>>) -> impl Fn(Result<T, E>) -> Option<T> + '_ {
help: consider introducing an explicit lifetime bound
    |
    |
103 |         fn ok<'a, T: 'a, E>(saved: &Mutex<Option<E>>) -> impl Fn(Result<T, E>) -> Option<T> + '_ + 'a {
    |               ^^^^^                                                                              ^^^^
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0309`.
error: could not compile `rustc-rayon`
