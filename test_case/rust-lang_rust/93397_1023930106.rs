plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.66
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0599]: no function or associated item named `total_cmp` found for slice `[f32]` in the current scope
     |
     |
4012 |         self.sort_unstable_by(Self::total_cmp);
     |                                     ^^^^^^^^^ function or associated item not found in `[f32]`

error[E0599]: no function or associated item named `total_cmp` found for slice `[f64]` in the current scope
     |
     |
4042 |         self.sort_unstable_by(Self::total_cmp);
     |                                     ^^^^^^^^^ function or associated item not found in `[f64]`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:03:46
