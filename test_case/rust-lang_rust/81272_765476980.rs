plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
error[E0624]: associated function `is_unique` is private
 --> library/alloc/src/rc/tests.rs:6:17
  |
6 |     assert!(Rc::is_unique(&x));
  |                 ^^^^^^^^^ private associated function

error[E0624]: associated function `is_unique` is private
 --> library/alloc/src/rc/tests.rs:8:18
  |
8 |     assert!(!Rc::is_unique(&x));
  |                  ^^^^^^^^^ private associated function

error[E0624]: associated function `is_unique` is private
  --> library/alloc/src/rc/tests.rs:10:17
   |
10 |     assert!(Rc::is_unique(&x));
   |                 ^^^^^^^^^ private associated function

error[E0624]: associated function `is_unique` is private
  --> library/alloc/src/rc/tests.rs:12:18
   |
12 |     assert!(!Rc::is_unique(&x));
   |                  ^^^^^^^^^ private associated function

error[E0624]: associated function `is_unique` is private
  --> library/alloc/src/rc/tests.rs:14:17
   |
14 |     assert!(Rc::is_unique(&x));
   |                 ^^^^^^^^^ private associated function
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0624`.
error: could not compile `alloc`
error: could not compile `alloc`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "proc_macro" "-p" "panic_unwind" "-p" "unwind" "-p" "alloc" "-p" "panic_abort" "-p" "core" "-p" "term" "-p" "std" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:51
