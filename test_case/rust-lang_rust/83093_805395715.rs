plain
    Checking object v0.22.0
    Checking hashbrown v0.11.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.14.0
error[E0432]: unresolved import `crate::detect::arch::is_x86_feature_detected`
   |
   |
30 |             use crate::detect::arch::is_x86_feature_detected;
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `is_x86_feature_detected` in `std_detect::detect::arch`
   |
   = note: this could be because a macro annotated with `#[macro_export]` will be exported at the root of the crate instead of the module where it is defined
help: a macro with this name exists at the root of the crate
   |
30 |             use ::is_x86_feature_detected;


error[E0603]: module `arch` is private
   |
   |
30 |             use crate::detect::arch::is_x86_feature_detected;
   |                                ^^^^ private module
   |
note: the module `arch` is defined here
  --> library/std/src/../../stdarch/crates/std_detect/src/detect/mod.rs:32:9
32 |         mod arch;
   |         ^^^^^^^^^

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0432, E0603.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:37
