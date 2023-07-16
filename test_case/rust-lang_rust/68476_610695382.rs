
error[E0308]: mismatched types
 --> src/lib.rs:9:43
  |
6 | impl<T> Wrapper<T> {
  |      - expected type parameter
7 |     fn map<U>(self, f: impl FnOnce(T) -> U) -> Wrapper<U> {
  |            - found type parameter
8 |         match self {
9 |             Self::There(v) => Self::There(f(v)),
  |                                           ^^^^ expected type parameter `T`, found type parameter `U`
  |
  = note: expected type parameter `T`
             found type parameter `U`
  = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
  = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters

error[E0308]: mismatched types
 --> src/lib.rs:9:31
  |
6 | impl<T> Wrapper<T> {
  |      - found type parameter
7 |     fn map<U>(self, f: impl FnOnce(T) -> U) -> Wrapper<U> {
  |            - expected type parameter           ---------- expected `Wrapper<U>` because of return type
8 |         match self {
9 |             Self::There(v) => Self::There(f(v)),
  |                               ^^^^^^^^^^^^^^^^^ expected type parameter `U`, found type parameter `T`
  |
  = note: expected enum `Wrapper<U>`
             found enum `Wrapper<T>`
  = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
  = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters

error: aborting due to 2 previous errors
