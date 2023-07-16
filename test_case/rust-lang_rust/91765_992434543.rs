plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.55
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0432]: unresolved import `build_helper::maybe_static_clang_rt`
  |
  |
3 | use build_helper::maybe_static_clang_rt;
  |     |             |
  |     |             |
  |     |             help: a similar name exists in the module: `maybe_static_library`
  |     no `maybe_static_clang_rt` in the root
For more information about this error, try `rustc --explain E0432`.
error: could not compile `unwind` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
