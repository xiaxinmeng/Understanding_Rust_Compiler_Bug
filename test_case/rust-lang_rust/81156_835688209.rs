plain
    Checking hashbrown v0.11.0
    Checking miniz_oxide v0.4.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.14.0
error[E0412]: cannot find type `ReadBuf` in this scope
   --> library/std/src/sys/unix/fd.rs:144:38
    |
144 |     pub fn read_buf(&self, buf: &mut ReadBuf<'_>) -> io::Result<()> {
    |
help: consider importing this struct
    |
6   | use crate::io::ReadBuf;
---
For more information about this error, try `rustc --explain E0412`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:13
