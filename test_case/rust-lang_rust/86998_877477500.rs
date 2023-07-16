plain
Set({"library/std"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
 finished in 2.147 seconds
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling std v0.0.0 (/checkout/library/std)
error[E0152]: found duplicate lang item `begin_panic_fmt`
    |
    |
452 | pub fn begin_panic_fmt(msg: &fmt::Arguments<'_>) -> ! {
    |
    |
    = note: the lang item is first defined in crate `std` (which `std` depends on)
    = note: first definition in `std` loaded from /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-99269b396c53f0b1.so, /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-99269b396c53f0b1.rlib
    = note: second definition in the local crate (`std`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0152`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:17:55
