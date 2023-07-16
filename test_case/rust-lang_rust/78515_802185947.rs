plain
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.11.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
error[E0599]: no function or associated item named `with_capacity` found for struct `SwitchWriter<_>` in the current scope
    |
    |
566 |                 *lock.borrow_mut() = SwitchWriter::with_capacity(0, stdout_raw());
    |                                                    ^^^^^^^^^^^^^ function or associated item not found in `SwitchWriter<_>`
    | 
   ::: library/std/src/io/buffered/switchwriter.rs:27:1
    |
27  | pub struct SwitchWriter<W: Write> {
    | --------------------------------- function or associated item `with_capacity` not found for this
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:37
