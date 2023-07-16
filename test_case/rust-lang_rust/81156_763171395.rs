plain
    Checking addr2line v0.14.0
error[E0432]: unresolved import `crate::io::Initializer`
 --> library/std/src/io/buffered/bufreader.rs:3:32
  |
3 | use crate::io::{self, BufRead, Initializer, IoSliceMut, Read, Seek, SeekFrom, DEFAULT_BUF_SIZE};
  |                                ^^^^^^^^^^^ no `Initializer` in `io`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:26
