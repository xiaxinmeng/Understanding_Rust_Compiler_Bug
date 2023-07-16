plain
........................................................................................ 616/648
................................
failures:

---- src/rc.rs - rc::Rc<dynAny>::downcast_unchecked (line 1282) stdout ----
error[E0412]: cannot find type `Rcl` in this scope
    |
    |
9   | let x: Rcl<dyn Any> = Rc::new(1_usize);
    |        ^^^ help: a struct with a similar name exists: `Rc`
   ::: /checkout/library/alloc/src/rc.rs:309:1
    |
    |
309 | pub struct Rc<T: ?Sized> {
    | ------------------------ similarly named struct `Rc` defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0412`.
Couldn't compile the test.
