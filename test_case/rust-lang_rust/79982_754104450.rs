plain
    Checking hashbrown v0.9.0
    Checking object v0.22.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.14.0
error: expected one of `!`, `(`, `+`, `::`, `<`, `where`, or `{`, found `;`
   --> library/std/src/sys/unix/ext/process.rs:212:39
    |
212 | impl private::Sealed for ExitStatusExt;
    |                                       ^ expected one of 7 possible tokens

error[E0432]: unresolved imports `super::process::CommandExt`, `super::process::ExitStatusExt`
   |
   |
74 |     pub use super::process::{CommandExt, ExitStatusExt};
   |                              ^^^^^^^^^^  ^^^^^^^^^^^^^ no `ExitStatusExt` in `sys::unix::ext::process`
   |                              |
   |                              no `CommandExt` in `sys::unix::ext::process`

error[E0599]: no method named `as_raw_fd` found for reference `&ChildStdin` in the current scope
   --> library/std/src/sys/unix/kernel_copy.rs:364:44
    |
364 |         CopyParams(FdMeta::Pipe, Some(self.as_raw_fd()))
    |                                            ^^^^^^^^^ method not found in `&ChildStdin`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `sys::unix::ext::io::AsRawFd` defines an item `as_raw_fd`, perhaps you need to implement it
    |
    |
22  | pub trait AsRawFd {


error[E0599]: no method named `as_raw_fd` found for reference `&ChildStdout` in the current scope
   --> library/std/src/sys/unix/kernel_copy.rs:370:44
    |
370 |         CopyParams(FdMeta::Pipe, Some(self.as_raw_fd()))
    |                                            ^^^^^^^^^ method not found in `&ChildStdout`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `sys::unix::ext::io::AsRawFd` defines an item `as_raw_fd`, perhaps you need to implement it
    |
    |
22  | pub trait AsRawFd {


error[E0599]: no method named `as_raw_fd` found for reference `&ChildStderr` in the current scope
   --> library/std/src/sys/unix/kernel_copy.rs:376:44
    |
376 |         CopyParams(FdMeta::Pipe, Some(self.as_raw_fd()))
    |                                            ^^^^^^^^^ method not found in `&ChildStderr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `sys::unix::ext::io::AsRawFd` defines an item `as_raw_fd`, perhaps you need to implement it
    |
    |
22  | pub trait AsRawFd {

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0432, E0599.
Some errors have detailed explanations: E0432, E0599.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:39
