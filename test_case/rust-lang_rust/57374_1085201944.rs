
error: implementation of `Fn` is not general enough
 --> src/lib.rs:8:5
  |
8 |     bar()
  |     ^^^^^ implementation of `Fn` is not general enough
  |
  = note: `impl Fn(&'2 str)` must implement `Fn<(&'1 str,)>`, for any lifetime `'1`...
  = note: ...but it actually implements `Fn<(&'2 str,)>`, for some specific lifetime `'2`

error[E0308]: mismatched types
 --> src/lib.rs:8:5
  |
3 | fn bar<Input>() -> impl Fn(Input) {
  |                    --------------
  |                    |
  |                    the expected opaque type
  |                    the found opaque type
...
8 |     bar()
  |     ^^^^^ one type is more general than the other
  |
  = note: expected associated type `<impl Fn(&str) as FnOnce<(&str,)>>::Output`
             found associated type `<impl Fn(&str) as FnOnce<(&str,)>>::Output`
