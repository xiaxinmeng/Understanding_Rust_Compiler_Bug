plain
    Checking miniz_oxide v0.4.0
    Checking object v0.22.0
    Checking hashbrown v0.9.0
    Checking addr2line v0.14.0
error[E0412]: cannot find type `Arc` in this scope
    |
    |
492 | impl<T: Error> Error for Arc<T> {
    |
help: consider importing one of these items
    |
19  | use alloc::sync::Arc;
19  | use alloc::sync::Arc;
    |
19  | use crate::sync::Arc;
    |

error[E0545]: `issue` must be a non-zero numeric string or "none"
    |
    |
491 | #[unstable(feature = "arc_error", issue = "")]
    |                                           |
    |                                           |
    |                                           cannot parse integer from empty string
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0412`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:28
