plain
    Checking addr2line v0.14.0
error: unused attribute
    --> library/std/src/path.rs:1077:23
     |
1077 | #[cfg_attr(not(test), rustc_diagnostic_item = "PathBuf")]
     |
     |
     = note: `-D unused-attributes` implied by `-D warnings`
error: unused attribute
    --> library/std/src/path.rs:1732:23
     |
     |
1732 | #[cfg_attr(not(test), rustc_diagnostic_item = "Path")]

error: aborting due to 2 previous errors

error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:34
