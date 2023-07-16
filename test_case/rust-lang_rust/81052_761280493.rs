plain
    Checking addr2line v0.14.0
error: trait objects without an explicit `dyn` are deprecated
    --> library/std/src/io/mod.rs:2475:26
     |
2475 |         (&self.inner as &SizeHint).size_hint()
     |                          ^^^^^^^^ help: use `dyn`: `dyn SizeHint`
     |
     = note: `-D bare-trait-objects` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `std`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:23
