plain

---- [ui] src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs stdout ----
diff of stderr:

- error[E0277]: `main` has invalid return type `Result<f32, ParseFloatError>`
+ error[E0277]: `main` has invalid return type `f32`
3    |
4 LL |   #[test]

8 LL | | }
8 LL | | }
9    | |_^ `main` can only return types that implement `Termination`
10    |
-    = help: the trait `Termination` is not implemented for `Result<f32, ParseFloatError>`
-    = help: the following other types implement trait `Termination`:
-              Result<!, E>
-              Result<(), E>
-              Result<Infallible, E>
+    = help: the trait `Termination` is not implemented for `f32`
+    = note: required because of the requirements on the impl of `Termination` for `Result<f32, ParseFloatError>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
16 note: required by a bound in `assert_test_result`
17   --> $SRC_DIR/test/src/lib.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-test-wrong-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `main` has invalid return type `f32`
   |
LL |   #[test]
   |   ------- in this procedural macro expansion
   |   ------- in this procedural macro expansion
LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
LL | |     "0".parse()
LL | | }
   | |_^ `main` can only return types that implement `Termination`
   = help: the trait `Termination` is not implemented for `f32`
   = help: the trait `Termination` is not implemented for `f32`
   = note: required because of the requirements on the impl of `Termination` for `Result<f32, ParseFloatError>`
note: required by a bound in `assert_test_result`
   |
   |
LL | pub fn assert_test_result<T: Termination>(result: T) {
   |                              ^^^^^^^^^^^ required by this bound in `assert_test_result`
   = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
