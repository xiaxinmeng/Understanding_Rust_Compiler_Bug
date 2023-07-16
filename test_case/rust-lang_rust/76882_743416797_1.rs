
error[E0759]: `a` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
 --> src/lib.rs:5:5
  |
4 | fn get_trait<I>(a: &()) -> impl Trait {
  |                    --- this data with an anonymous lifetime `'_`...
5 |     a
  |     ^ ...is captured here...
  |
note: ...and is required to live as long as `'static` here
 --> src/lib.rs:4:28
  |
4 | fn get_trait<I>(a: &()) -> impl Trait {
  |                            ^^^^^^^^^^
help: to declare that the `impl Trait` captures data from argument `a`, you can add an explicit `'_` lifetime bound
  |
4 | fn get_trait<I>(a: &()) -> impl Trait + '_ {
  |                                       ^^^^
