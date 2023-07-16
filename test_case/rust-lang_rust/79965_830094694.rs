plain
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.11.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.14.0
error[E0599]: no variant or associated item named `NotSupported` found for enum `ErrorKind` in the current scope
   --> library/std/src/sys/unix/mod.rs:156:36
    |
156 |         libc::ENOSYS => ErrorKind::NotSupported,
    |                                    |
    |                                    variant or associated item not found in `ErrorKind`
    |                                    variant or associated item not found in `ErrorKind`
    |                                    help: there is a variant with a similar name: `Unsupported`
   ::: library/std/src/io/error.rs:95:1
    |
95  | pub enum ErrorKind {
95  | pub enum ErrorKind {
    | ------------------ variant or associated item `NotSupported` not found here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:58
