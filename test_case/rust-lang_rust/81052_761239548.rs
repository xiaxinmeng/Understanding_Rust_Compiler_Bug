plain
    Checking miniz_oxide v0.4.0
    Checking object v0.22.0
    Checking hashbrown v0.9.0
    Checking addr2line v0.14.0
error[E0520]: `lower_bound` specializes an item from a parent `impl`, but that item is not marked `default`
     |
     |
2495 |   impl<T> SizeHint for T {}
     |   ------------------------- parent `impl` is here
...
2499 | /     fn lower_bound(&self) -> usize {
2500 | |         self.buffer().len()
2501 | |     }
     | |_____^ cannot specialize default item `lower_bound`
     |
     = note: to specialize, `lower_bound` in the parent `impl` must be marked `default`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0520`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:34
