plain
   Compiling build_helper v0.1.0 (/checkout/src/build_helper)
   Compiling libc v0.2.108
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
error[E0432]: unresolved import `build_helper::maybe_static_clang_rt`
  |
  |
3 | use build_helper::maybe_static_clang_rt;
  |     |             |
  |     |             |
  |     |             help: a similar name exists in the module: `maybe_static_library`
  |     no `maybe_static_clang_rt` in the root
For more information about this error, try `rustc --explain E0432`.
error: could not compile `std` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
