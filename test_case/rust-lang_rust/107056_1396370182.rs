plain
...............iii...................................................................... 14168/14206
......................................
failures:

---- [ui] checkout/tests/ui/async-await/in-trait/send-bound.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/send-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/send-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/send-bound/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0277]: `impl Future<Output = usize>` cannot be sent between threads safely
  --> /checkout/tests/ui/async-await/in-trait/send-bound.rs:15:17
   |
LL |     assert_send(x.foo())
   |     ----------- ^^^^^^^ `impl Future<Output = usize>` cannot be sent between threads safely
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Send` is not implemented for `impl Future<Output = usize>`
note: required by a bound in `assert_send`
  --> /checkout/tests/ui/async-await/in-trait/send-bound.rs:12:19
   |
LL | fn assert_send<T: Send>(_: T) {}
   |                   ^^^^ required by this bound in `assert_send`
   |
   |
LL | fn use_trait<T: MyTrait>(x: T) where impl Future<Output = usize>: Send {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
