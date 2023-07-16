plain
   Compiling rustc-std-workspace-std v1.99.0 (/checkout/library/rustc-std-workspace-std)
   Compiling proc_macro v0.0.0 (/checkout/library/proc_macro)
   Compiling unicode-width v0.1.8
   Compiling getopts v0.2.21
error[E0277]: expected a `FnOnce<()>` closure, found `Result<String, VarError>`
    --> library/proc_macro/src/lib.rs:1293:48
     |
1293 |         let value = injected_value.map_or_else(env_value, Ok);
     |                                    ----------- ^^^^^^^^^ expected an `FnOnce<()>` closure, found `Result<String, VarError>`
     |                                    required by a bound introduced by this call
     |
     |
     = help: the trait `FnOnce<()>` is not implemented for `Result<String, VarError>`
     = note: wrap the `Result<String, VarError>` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `Option::<T>::map_or_else`
     |
     |
996  |         D: ~const FnOnce() -> U,
     |            ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map_or_else`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `proc_macro` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:35
