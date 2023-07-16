
~/workspace/proc-macro-workshop (proc-macro-workshop:HEAD 0)$ RUSTC_BOOTSTRAP=1 RUST_BACKTRACE=1 cargo +1.30.0 build
   Compiling proc-macro2 v1.0.6
   Compiling syn v1.0.11
   Compiling bitfield-impl v0.0.0 (/Users/ekuber/workspace/proc-macro-workshop/bitfield/impl)
   Compiling sorted v0.0.0 (/Users/ekuber/workspace/proc-macro-workshop/sorted)
error[E0432]: unresolved import `proc_macro`
error[E0432]: unresolved import `proc_macro`
 --> bitfield/impl/src/lib.rs:3:5
  |
3 | use proc_macro::TokenStream;
  |     ^^^^^^^^^^ Did you mean `self::proc_macro`?

 --> sorted/src/lib.rs:3:5
  |
3 | use proc_macro::TokenStream;
  |     ^^^^^^^^^^ Did you mean `self::proc_macro`?

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.

For more information about this error, try `rustc --explain E0432`.
error: Could not compile `sorted`.
warning: build failed, waiting for other jobs to finish...
error: Could not compile `bitfield-impl`.
warning: build failed, waiting for other jobs to finish...
error: build failed
