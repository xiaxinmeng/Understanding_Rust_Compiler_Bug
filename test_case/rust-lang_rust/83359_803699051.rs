plain
    Checking panic_abort v0.0.0 (/checkout/library/panic_abort)
error[E0739]: attribute should be applied to function
    --> library/alloc/src/vec/mod.rs:2631:1
     |
2631 |   #[track_caller]
     |   ^^^^^^^^^^^^^^^
2632 |   #[stable(feature = "extend_ref", since = "1.2.0")]
2633 | / impl<'a, T: Copy + 'a, A: Allocator + 'a> Extend<&'a T> for Vec<T, A> {
2634 | |     fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
2635 | |         self.spec_extend(iter.into_iter())
...    |
2648 | |     }
2649 | | }
     | |_- not a function
     | |_- not a function

error: aborting due to previous error

For more information about this error, try `rustc --explain E0739`.
error: could not compile `alloc`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:36
