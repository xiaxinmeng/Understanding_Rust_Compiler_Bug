plain
    Checking hashbrown v0.11.0
    Checking miniz_oxide v0.4.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
error: use of module `core::raw` that will be deprecated in future version 1.53.0: use pointer metadata APIs instead https://github.com/rust-lang/rust/issues/81513
    |
460 | pub use core::raw;
    |         ^^^^^^^^^
    |
    |
    = note: `-D deprecated-in-future` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `std`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:39
