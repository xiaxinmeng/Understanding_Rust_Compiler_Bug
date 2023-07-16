plain
..................................iiiiii.................................i.......................... 1200/1205
.....
failures:

---- src/panicking.rs - panicking::update_hook (line 190) stdout ----
error[E0373]: closure may outlive the current function, but it borrows `prev`, which is owned by the current function
   |
   |
8  |     Box::new(|panic_info| {
   |              ^^^^^^^^^^^^ may outlive borrowed value `prev`
9  |         println!("Print custom message and execute panic handler as usual");
10 |         prev(panic_info);
   |         ---- `prev` is borrowed here
note: closure is returned here
  --> src/panicking.rs:195:5
   |
   |
8  | /     Box::new(|panic_info| {
9  | |         println!("Print custom message and execute panic handler as usual");
10 | |         prev(panic_info);
   | |______^
   | |______^
help: to force the closure to take ownership of `prev` (and any other referenced variables), use the `move` keyword
   |
8  |     Box::new(move |panic_info| {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0373`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:28:08
