
error[E0277]: the trait bound `for<'b> P: Process<'b>` is not satisfied
  --> test.rs:20:36
   |
20 |     let _: Box<for<'b> Wrap<'b>> = Box::new(Wrapper(process));
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'b> Process<'b>` is not implemented for `P`
   |
   = help: consider adding a `where for<'b> P: Process<'b>` bound
   = note: required because of the requirements on the impl of `for<'b> Wrap<'b>` for `Wrapper<P>`
   = note: required for the cast to the object type `for<'b> Wrap<'b>`

error[E0277]: the trait bound `for<'b> <P as Process<'b>>::Item: std::iter::Iterator` is not satisfied
  --> test.rs:20:36
   |
20 |     let _: Box<for<'b> Wrap<'b>> = Box::new(Wrapper(process));
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'b> std::iter::Iterator` is not implemented for `<P as Process<'b>>::Item`
   |
   = note: `<P as Process<'b>>::Item` is not an iterator; maybe try calling `.iter()` or a similar method
   = note: required because of the requirements on the impl of `for<'b> Wrap<'b>` for `Wrapper<P>`
   = note: required for the cast to the object type `for<'b> Wrap<'b>`

error: aborting due to 2 previous errors
