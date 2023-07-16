
foo.rs:20:36: 20:62 error: the trait `for<'b> Process<'b>` is not implemented for the type `P` [E0277]
foo.rs:20     let _: Box<for<'b> Wrap<'b>> = Box::new(Wrapper(process));
                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~
foo.rs:20:36: 20:62 help: run `rustc --explain E0277` to see a detailed explanation
foo.rs:20:36: 20:62 error: the trait `for<'b> core::iter::Iterator` is not implemented for the type `<P as Process<'b>>::Item` [E0277]
foo.rs:20     let _: Box<for<'b> Wrap<'b>> = Box::new(Wrapper(process));
                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~
foo.rs:20:36: 20:62 help: run `rustc --explain E0277` to see a detailed explanation
foo.rs:20:36: 20:62 note: `<P as Process<'b>>::Item` is not an iterator; maybe try calling `.iter()` or a similar method
foo.rs:20     let _: Box<for<'b> Wrap<'b>> = Box::new(Wrapper(process));
                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~
foo.rs:20:36: 20:62 error: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
foo.rs:20     let _: Box<for<'b> Wrap<'b>> = Box::new(Wrapper(process));
                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~
note: first, the lifetime cannot outlive lifetime ReSkolemized(0, BrNamed(DefId { krate: 0, node: 64 }, "\'b"(61)))...
foo.rs:20:36: 20:62 note: ...so that trait type parameters matches those specified on the impl (expected `Wrap<'b>`, found `Wrap<'_>`)
foo.rs:20     let _: Box<for<'b> Wrap<'b>> = Box::new(Wrapper(process));
                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~
note: but, the lifetime must be valid for the static lifetime...
foo.rs:20:36: 20:62 note: ...so that trait type parameters matches those specified on the impl (expected `Process<'_>`, found `Process<'static>`)
foo.rs:20     let _: Box<for<'b> Wrap<'b>> = Box::new(Wrapper(process));
                                             ^~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 3 previous errors
