plain
    Checking object v0.22.0
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.11.0
    Checking addr2line v0.14.0
error[E0599]: no method named `set_buffer_mode` found for struct `ReentrantMutexGuard<'_, RefCell<SwitchWriter<StdoutRaw>>>` in the current scope
    |
    |
561 |                 lock.set_buffer_mode(BufferMode::Immediate)
    |                      ^^^^^^^^^^^^^^^ method not found in `ReentrantMutexGuard<'_, RefCell<SwitchWriter<StdoutRaw>>>`
   ::: library/std/src/sys_common/remutex.rs:40:1
    |
    |
40  | pub struct ReentrantMutexGuard<'a, T: 'a> {
    | ----------------------------------------- method `set_buffer_mode` not found for this
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:45
