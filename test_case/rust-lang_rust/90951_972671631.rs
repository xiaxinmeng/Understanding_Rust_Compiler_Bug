plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0061]: this function takes 1 argument but 2 arguments were supplied
   --> compiler/rustc_middle/src/ty/context.rs:937:44
    |
937 |         let mk_const = |ty, val| interners.intern_const(ty, val);
    |                                            |
    |                                            expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> compiler/rustc_middle/src/ty/context.rs:179:8
    |
179 |     fn intern_const(&self, const_: Const<'tcx>) -> &'tcx Const<'tcx> {

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
