
error[E0308]: mismatched types
 --> src/lib.rs:4:1
  |
4 | / pub struct Request<'a, S: Scope<'a> = StaticScope> {
5 | |     scope: S,
6 | |     phantom: PhantomData<Cell<&'a ()>>,
7 | | }
  | |_^ lifetime mismatch
  |
  = note: expected type `Scope<'a>`
             found type `Scope<'static>`
note: the lifetime 'a as defined on the struct at 4:1...
 --> src/lib.rs:4:1
  |
4 | pub struct Request<'a, S: Scope<'a> = StaticScope> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: ...does not necessarily outlive the static lifetime

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: Could not compile `rsmpi-request-lifetime-issue`.

To learn more, run the command again with --verbose.
