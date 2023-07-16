
error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
 --> <source>:2:24
  |
2 | type BlahResult<I, E = impl ExampleTrait<I>> = Result<I, E>;
  |                        ^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
