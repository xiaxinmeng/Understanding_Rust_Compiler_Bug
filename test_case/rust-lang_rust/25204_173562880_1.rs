
wf-15.rs:5:1: 5:49 warning: the trait `core::marker::Sized` is not implemented for the type `Self` [E0277]
wf-15.rs:5 trait OptSelfSized where Option<Self>: Sized { }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
wf-15.rs:5:1: 5:49 help: run `rustc --explain E0277` to see a detailed explanation
wf-15.rs:5:1: 5:49 note: `Self` does not have a constant size known at compile-time
wf-15.rs:5:1: 5:49 note: this warning results from recent bug fixes and clarifications; it will become a HARD ERROR in the next release. See RFC 1214 for details.
wf-15.rs:5 trait OptSelfSized where Option<Self>: Sized { }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
wf-15.rs:5:1: 5:49 note: required by `core::option::Option`
