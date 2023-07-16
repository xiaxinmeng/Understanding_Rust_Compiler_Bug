plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no method named `iter` found for struct `StableSet` in the current scope
  --> compiler/rustc_middle/src/middle/mod.rs:22:38
   |
22 |                 .chain(self.unstable.iter().map(|f| (*f, None)))
   |                                      ^^^^ method not found in `StableSet<rustc_span::Symbol>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
