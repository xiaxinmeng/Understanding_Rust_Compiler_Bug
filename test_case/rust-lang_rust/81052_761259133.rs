plain
    Checking hashbrown v0.9.0
    Checking object v0.22.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.14.0
error[E0407]: method `lower_bound` is not a member of trait `SizeHint`
     |
     |
2492 | /     default fn lower_bound(&self) -> usize {
2494 | |     }
2494 | |     }
     | |_____^ not a member of trait `SizeHint`

error[E0407]: method `lower_bound` is not a member of trait `SizeHint`
     |
     |
2499 | /     fn lower_bound(&self) -> usize {
2500 | |         self.buffer().len()
2501 | |     }
     | |_____^ not a member of trait `SizeHint`

error[E0599]: no method named `lower_bound` found for reference `&Self` in the current scope
     |
     |
2486 |         (self.lower_bound(), self.upper_bound())
     |               ^^^^^^^^^^^ help: there is an associated function with a similar name: `upper_bound`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0407, E0599.
For more information about an error, try `rustc --explain E0407`.
For more information about an error, try `rustc --explain E0407`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:37
