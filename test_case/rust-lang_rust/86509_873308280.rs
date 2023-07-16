plain
   |
16 | mod tests;
   | ^^^^^^^^^^
   |
   = help: to create the module `tests`, create file "library/std/src/sys/unix/os_str/tests.rs" or "library/std/src/sys/unix/os_str/tests/mod.rs"
error: aborting due to previous error

For more information about this error, try `rustc --explain E0583`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:45
