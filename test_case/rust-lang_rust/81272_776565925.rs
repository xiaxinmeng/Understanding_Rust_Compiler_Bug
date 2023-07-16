plain
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.18
    Checking panic_abort v0.0.0 (/checkout/library/panic_abort)
error[E0432]: unresolved import `crate::rc::is_dangling`
   |
   |
30 | use crate::rc::is_dangling;
   |     ^^^^^^^^^^^^^^^^^^^^^^ no `is_dangling` in `rc`

error[E0405]: cannot find trait `MarkerEq` in module `crate::rc`
     |
     |
2065 | impl<T: ?Sized + crate::rc::MarkerEq> ArcEqIdent<T> for Arc<T> {
     |                             ^^^^^^^^ not found in `crate::rc`
help: consider importing one of these items
     |
     |
7    | use crate::rc::utils::MarkerEq;
     |
7    | use crate::vec::slice::cmp::MarkerEq;

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0405, E0432.
Some errors have detailed explanations: E0405, E0432.
For more information about an error, try `rustc --explain E0405`.
error: could not compile `alloc`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:25
