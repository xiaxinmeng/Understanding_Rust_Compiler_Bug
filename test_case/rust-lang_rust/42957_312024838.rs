
Compiling rustc_typeck v0.0.0 (file:///home/aburka/rust/src/librustc_typeck)
warning: diagnostic code E0619 already used
    --> src/librustc_typeck/check/mod.rs:4719:21
     |
4719 | /                     type_error_struct!(self.tcx.sess, sp, ty, E0619,
4720 | |                                        "the type of this value must be known in this context")
     | |______________________________________________________________________________________________^
     |
note: previous invocation
    --> src/librustc_typeck/check/intrinsic.rs:40:13
     |
40   | /             struct_span_err!(tcx.sess, it.span, E0619,
41   | |                              "intrinsic must be a function")
     | |____________________________________________________________^
     = note: this error originates in a macro outside of the current crate
