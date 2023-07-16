plain
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.9.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
error[E0445]: private trait `SizeHint` in public interface
     |
     |
2459 | / impl<R: Read + SizeHint> Iterator for Bytes<R> {
2460 | |     type Item = Result<u8>;
2461 | |
2462 | |     fn next(&mut self) -> Option<Result<u8>> {
2476 | |     }
2477 | | }
2477 | | }
     | |_^ can't leak private trait
...
2480 |   trait SizeHint {
     |   -------------- `SizeHint` declared as private
error: aborting due to previous error

For more information about this error, try `rustc --explain E0445`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:35
