plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0493]: destructors cannot be evaluated at compile-time
  --> library/core/tests/task.rs:24:60
   |
24 |     const CONTEXT: Context<'static> = Context::from_waker(&WAKER);
   |                                                            ^^^^^- value is dropped here
   |                                                            constants cannot evaluate destructors

error[E0716]: temporary value dropped while borrowed
  --> library/core/tests/task.rs:24:60
  --> library/core/tests/task.rs:24:60
   |
24 |     const CONTEXT: Context<'static> = Context::from_waker(&WAKER);
   |                                       |                    |    |
   |                                       |                    |    temporary value is freed at the end of this statement
   |                                       |                    creates a temporary which is freed while still in use
   |                                       |                    creates a temporary which is freed while still in use
   |                                       using this value as a constant requires that borrow lasts for `'static`
Some errors have detailed explanations: E0493, E0716.
For more information about an error, try `rustc --explain E0493`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:01:38
