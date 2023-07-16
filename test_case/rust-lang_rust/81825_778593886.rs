plain
    Checking hashbrown v0.9.0
    Checking object v0.22.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.14.0
error: unnecessary `unsafe` block
   --> library/std/src/sys/unix/process/process_unix.rs:119:25
    |
119 |         let _env_lock = unsafe { sys::os::env_read_lock() };
    |                         ^^^^^^ unnecessary `unsafe` block
    |
    = note: `-D unused-unsafe` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `std`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:25
