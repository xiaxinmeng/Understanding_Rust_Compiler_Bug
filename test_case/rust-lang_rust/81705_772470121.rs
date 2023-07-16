plain
    Checking addr2line v0.14.0
error: associated function has missing stability attribute
   --> library/std/src/error.rs:139:5
    |
139 | /     fn sources(&self) -> Chain<'_> {
140 | |         Chain { current: self.source() }
    | |_____^

error: aborting due to previous error


error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:31
