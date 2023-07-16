plain
   Compiling generic-array v0.14.4
   Compiling miniz_oxide v0.4.0
   Compiling unicode-normalization v0.1.13
   Compiling unic-emoji-char v0.9.0
error[E0277]: the `?` operator can only be used on `Option`s, not `Result`s, in a method that returns `Option`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.30/src/wrapper.rs:517:79
    |
514 | /     pub fn join(&self, other: Span) -> Option<Span> {
515 | |         let ret = match (self, other) {
516 | |             #[cfg(proc_macro_span)]
517 | |             (Span::Compiler(a), Span::Compiler(b)) => Span::Compiler(a.join(b)?),
    | |                                                                               ^ use `.ok()?` if you want to discard the `Result<Infallible, JoinError>` error information
521 | |         Some(ret)
522 | |     }
    | |_____- this function returns an `Option`
    |
    |
    = help: the trait `FromResidual<Result<Infallible, JoinError>>` is not implemented for `Option<imp::Span>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `proc-macro2` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
