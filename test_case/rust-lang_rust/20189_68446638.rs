
/home/cmr/rust/src/test/run-pass/traits-multidispatch-infer-convert-source-and-target.rs:35:5: 35:9 error: unable to infer enough type information about `_`; type annotations required
/home/cmr/rust/src/test/run-pass/traits-multidispatch-infer-convert-source-and-target.rs:35     test(Default::default(), Default::default(),  2, 4);
                                                                                                ^~~~
error: aborting due to previous error
/home/cmr/rust/src/test/run-pass/traits-multidispatch-infer-convert-target.rs:42:5: 42:9 error: unable to infer enough type information about `_`; type annotations required
/home/cmr/rust/src/test/run-pass/traits-multidispatch-infer-convert-target.rs:42     test(Default::default(), 44_u32, 2, 4);
                                                                                     ^~~~
error: aborting due to previous error

