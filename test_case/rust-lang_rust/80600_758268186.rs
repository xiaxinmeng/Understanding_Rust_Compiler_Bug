plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0308]: mismatched types
   --> library/core/src/mem/maybe_uninit.rs:843:13
    |
242 | impl<T> MaybeUninit<T> {
    |      - this type parameter
...
835 |     pub unsafe fn array_assume_init<const N: usize>(array: [Self; N]) -> [T; N] {
    |                                                                          ------ expected `[T; N]` because of return type
...
843 |             (&array as *const _ as *const T).read()
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected array, found type parameter `T`
    |
    = note:       expected array `[T; N]`
            found type parameter `T`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:16
