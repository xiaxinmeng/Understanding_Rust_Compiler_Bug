
error[E0277]: `&mut ()` is not a tuple
   --> /home/gh-compiler-errors/test.rs:6:41
    |
6   |     handlers.unwrap().as_mut().call_mut(&mut ());
    |                                -------- -^^^^^^
    |                                |        |
    |                                |        `&mut ()` is not a tuple
    |                                |        help: consider removing the leading `&`-reference
    |                                required by a bound introduced by this call
    |
    = help: the trait `Tuple` is not implemented for `&mut ()`
note: required by a bound in `call_mut`
   --> /home/gh-compiler-errors/rust/library/core/src/ops/function.rs:334:23
    |
334 | pub trait FnMut<Args: Tuple>: FnOnce<Args> {
    |                       ^^^^^ required by this bound in `call_mut`

error: aborting due to previous error
