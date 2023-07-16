
error[E0582]: binding for associated type `Output` references lifetime `'s`, which does not appear in the trait input types
 --> test.rs:3:76
  |
3 | fn does_not_compile<'b>(y: PhantomData<&'b ()>) -> Box<for<'s: 'b> Fn() -> PhantomData<&'s ()>> {
  |                                                                            ^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
