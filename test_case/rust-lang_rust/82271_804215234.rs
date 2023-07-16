plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0308]: mismatched types
   --> library/core/src/cell.rs:820:29
    |
820 |     pub fn borrow(&self) -> Ref<'_, T> {
    |            ------           ^^^^^^^^^^ expected struct `Ref`, found `()`
    |            |
    |            implicitly returns `()` as its body has no tail or `return` expression
821 |         self.try_borrow().expect("already mutably borrowed");
    |                                                             - help: consider removing this semicolon
    |
    = note: expected struct `Ref<'_, T>`

error[E0308]: mismatched types
   --> library/core/src/cell.rs:913:33
    |
    |
913 |     pub fn borrow_mut(&self) -> RefMut<'_, T> {
    |            ----------           ^^^^^^^^^^^^^ expected struct `RefMut`, found `()`
    |            |
    |            implicitly returns `()` as its body has no tail or `return` expression
914 |         self.try_borrow_mut().expect("already borrowed");
    |                                                         - help: consider removing this semicolon
    |
    = note: expected struct `RefMut<'_, T>`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:34
