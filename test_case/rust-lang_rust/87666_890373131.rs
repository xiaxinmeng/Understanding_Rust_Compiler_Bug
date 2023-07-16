plain
Set({"library/std"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
 finished in 2.511 seconds
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling std v0.0.0 (/checkout/library/std)
error[E0425]: cannot find function `init` in module `sys`
  --> library/std/src/sys_common/rt.rs:16:14
   |
16 |         sys::init(argc, argv);
   |              ^^^^ not found in `sys`
help: consider importing one of these items
   |
4  | use crate::sys::args::init;
   |
   |
4  | use crate::sys::net::init;
   |
4  | use crate::sys::stack_overflow::init;
   |
4  | use crate::sys::thread::guard::init;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
error: could not compile `std`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:19:25
