plain
    Checking addr2line v0.14.0
error: trait objects without an explicit `dyn` are deprecated
   --> library/std/src/sys/unix/ext/process.rs:212:26
    |
212 | impl private::Sealed for ExitStatusExt { }
    |                          ^^^^^^^^^^^^^ help: use `dyn`: `dyn ExitStatusExt`
    |
    = note: `-D bare-trait-objects` implied by `-D warnings`

error[E0038]: the trait `ExitStatusExt` cannot be made into an object
   --> library/std/src/sys/unix/ext/process.rs:212:26
    |
212 | impl private::Sealed for ExitStatusExt { }
    |                          ^^^^^^^^^^^^^ `ExitStatusExt` cannot be made into an object
    |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   --> library/std/src/sys/unix/ext/process.rs:181:8
    |
177 | pub trait ExitStatusExt : private::Sealed {
    |           ------------- this trait cannot be made into an object...
...
181 |     fn from_raw(raw: i32) -> Self;
    |        ^^^^^^^^ ...because associated function `from_raw` has no `self` parameter
help: consider turning `from_raw` into a method by giving it a `&self` argument
    |
181 |     fn from_raw(&self, raw: i32) -> Self;
    |                 ^^^^^^
help: alternatively, consider constraining `from_raw` so it does not apply to trait objects
    |
181 |     fn from_raw(raw: i32) -> Self where Self: Sized;


error[E0277]: the trait bound `process::ExitStatus: Sealed` is not satisfied
   --> library/std/src/sys/unix/ext/process.rs:215:6
    |
177 | pub trait ExitStatusExt : private::Sealed {
    |                           --------------- required by this bound in `ExitStatusExt`
...
215 | impl ExitStatusExt for process::ExitStatus {
    |      ^^^^^^^^^^^^^ the trait `Sealed` is not implemented for `process::ExitStatus`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0038, E0277.
For more information about an error, try `rustc --explain E0038`.
For more information about an error, try `rustc --explain E0038`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:29
