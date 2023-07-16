plain
[00:13:40]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:13:40]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:40]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:40]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:13:41] error[E0432]: unresolved import `errors`
[00:13:41]   --> librustc_mir/borrow_check/nll/region_infer/error_reporting/mod.rs:25:5
[00:13:41] 25 | use errors::Applicability;
[00:13:41]    |     ^^^^^^ Did you mean `syntax::errors`?
[00:13:41] 
[00:13:53] error: aborting due to previous error
