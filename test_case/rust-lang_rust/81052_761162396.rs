plain
    Checking object v0.22.0
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.9.0
    Checking addr2line v0.14.0
error: cannot specialize on trait `io::Read`
     |
     |
2480 | / impl<R: Read> Iterator for Bytes<BufReader<R>> {
2481 | |     fn size_hint(&self) -> (usize, Option<usize>) {
2482 | |         (self.inner.buffer().len(), None)
2484 | | }
     | |_^

error: aborting due to previous error
error: aborting due to previous error

error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:40
