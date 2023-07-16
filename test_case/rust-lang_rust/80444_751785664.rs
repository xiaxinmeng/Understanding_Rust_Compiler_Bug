plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: associated function has missing stability attribute
   --> library/core/src/ops/range.rs:684:5
    |
684 | /     pub fn as_ref(&self) -> Bound<&T> {
685 | |         match *self {
686 | |             Included(ref x) => Included(x),
687 | |             Excluded(ref x) => Excluded(x),
688 | |             Unbounded => Unbounded,
690 | |     }
    | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/core/src/ops/range.rs:694:5
    |
694 | /     pub fn as_mut(&mut self) -> Bound<&mut T> {
695 | |         match *self {
696 | |             Included(ref mut x) => Included(x),
697 | |             Excluded(ref mut x) => Excluded(x),
698 | |             Unbounded => Unbounded,
700 | |     }
    | |_____^

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:18
