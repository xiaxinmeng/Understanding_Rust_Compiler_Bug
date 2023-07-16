plain
........................................................................................ 11880/13197
........................................................................................ 11968/13197
........................................................................................ 12056/13197
........................................................................................ 12144/13197
................................................................................F..F.... 12232/13197
..........................i............................................................. 12408/13197
........................................................................................ 12496/13197
........................................................................................ 12584/13197
........................................................................................ 12672/13197
---
---- [ui] src/test/ui/try-trait/option-to-result.rs stdout ----
diff of stderr:

27    |
28    = help: the trait `FromResidual<Result<Infallible, i32>>` is not implemented for `Option<i32>`
29    = help: the following other types implement trait `FromResidual<R>`:
+              <Option<Result<T, F>> as FromResidual<Result<Infallible, E>>>
30              <Option<T> as FromResidual<Yeet<()>>>
31              <Option<T> as FromResidual>


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/option-to-result/option-to-result.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/option-to-result/option-to-result.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-trait/option-to-result.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-trait/option-to-result.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/option-to-result" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/option-to-result/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
   |
   |
LL | / fn test_result() -> Result<(),()> {
LL | |     let a:Option<()> = Some(());
LL | |     a?;//~ ERROR the `?` operator can only be used
   | |      ^ use `.ok_or(...)?` to provide an error compatible with `Result<(), ()>`
LL | |     Ok(())
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `Result<(), ()>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Result<T, F> as FromResidual<Result<Infallible, E>>>
             <Result<T, F> as FromResidual<Yeet<E>>>

error[E0277]: the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
   |
   |
LL | / fn test_option() -> Option<i32>{
LL | |     let a:Result<i32, i32> = Ok(5);
LL | |     a?;//~ ERROR the `?` operator can only be used
   | |      ^ use `.ok()?` if you want to discard the `Result<Infallible, i32>` error information
LL | |     Some(5)
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<Result<Infallible, i32>>` is not implemented for `Option<i32>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Option<Result<T, F>> as FromResidual<Result<Infallible, E>>>
             <Option<T> as FromResidual<Yeet<()>>>
             <Option<T> as FromResidual>
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/try-trait/bad-interconversion.rs stdout ----
diff of stderr:

62    |
63    = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `Option<u16>`
64    = help: the following other types implement trait `FromResidual<R>`:
+              <Option<Result<T, F>> as FromResidual<Result<Infallible, E>>>
65              <Option<T> as FromResidual<Yeet<()>>>
66              <Option<T> as FromResidual>

77    |
77    |
78    = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Option<u64>`
79    = help: the following other types implement trait `FromResidual<R>`:
+              <Option<Result<T, F>> as FromResidual<Result<Infallible, E>>>
80              <Option<T> as FromResidual<Yeet<()>>>
81              <Option<T> as FromResidual>


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/bad-interconversion.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/bad-interconversion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-trait/bad-interconversion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-trait/bad-interconversion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `?` couldn't convert the error to `u8`
   |
   |
LL | fn result_to_result() -> Result<u64, u8> {
   |                          --------------- expected `u8` because of this
LL |     Ok(Err(123_i32)?)
   |                    ^ the trait `From<i32>` is not implemented for `u8`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `From<T>`:
             <f32 as From<i16>>
             <f32 as From<i8>>
             <f32 as From<u16>>
             <f32 as From<u8>>
             <f64 as From<f32>>
             <f64 as From<i16>>
             <f64 as From<i32>>
             <f64 as From<i8>>
           and 67 others
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, i32>>` for `Result<u64, u8>`

error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
   |
   |
LL | / fn option_to_result() -> Result<u64, String> {
LL | |     Some(3)?;
   | |            ^ use `.ok_or(...)?` to provide an error compatible with `Result<u64, String>`
LL | |     //~^ ERROR the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
LL | |     Ok(10)
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `Result<u64, String>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Result<T, F> as FromResidual<Result<Infallible, E>>>
             <Result<T, F> as FromResidual<Yeet<E>>>

error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
   |
   |
LL | / fn control_flow_to_result() -> Result<u64, String> {
LL | |     Ok(ControlFlow::Break(123)?)
   | |                               ^ this `?` produces `ControlFlow<{integer}, Infallible>`, which is incompatible with `Result<u64, String>`
LL | |     //~^ ERROR the `?` operator can only be used on `Result`s in a function that returns `Result`
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Result<u64, String>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Result<T, F> as FromResidual<Result<Infallible, E>>>
             <Result<T, F> as FromResidual<Yeet<E>>>

error[E0277]: the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
   |
LL | / fn result_to_option() -> Option<u16> {
LL | / fn result_to_option() -> Option<u16> {
LL | |     Some(Err("hello")?)
   | |                      ^ use `.ok()?` if you want to discard the `Result<Infallible, &str>` error information
LL | |     //~^ ERROR the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `Option<u16>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Option<Result<T, F>> as FromResidual<Result<Infallible, E>>>
             <Option<T> as FromResidual<Yeet<()>>>
             <Option<T> as FromResidual>

error[E0277]: the `?` operator can only be used on `Option`s in a function that returns `Option`
   |
LL | / fn control_flow_to_option() -> Option<u64> {
LL | / fn control_flow_to_option() -> Option<u64> {
LL | |     Some(ControlFlow::Break(123)?)
   | |                                 ^ this `?` produces `ControlFlow<{integer}, Infallible>`, which is incompatible with `Option<u64>`
LL | |     //~^ ERROR the `?` operator can only be used on `Option`s in a function that returns `Option`
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Option<u64>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Option<Result<T, F>> as FromResidual<Result<Infallible, E>>>
             <Option<T> as FromResidual<Yeet<()>>>
             <Option<T> as FromResidual>

error[E0277]: the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
   |
   |
LL | / fn result_to_control_flow() -> ControlFlow<String> {
LL | |     ControlFlow::Continue(Err("hello")?)
   | |                                       ^ this `?` produces `Result<Infallible, &str>`, which is incompatible with `ControlFlow<String>`
LL | |     //~^ ERROR the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `ControlFlow<String>`
   = help: the trait `FromResidual` is implemented for `ControlFlow<B, C>`

error[E0277]: the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
   |
   |
LL | / fn option_to_control_flow() -> ControlFlow<u64> {
LL | |     Some(3)?;
   | |            ^ this `?` produces `Option<Infallible>`, which is incompatible with `ControlFlow<u64>`
LL | |     //~^ ERROR the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
LL | |     ControlFlow::Break(10)
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `ControlFlow<u64>`
   = help: the trait `FromResidual` is implemented for `ControlFlow<B, C>`

error[E0277]: the `?` operator in a function that returns `ControlFlow<B, _>` can only be used on other `ControlFlow<B, _>`s (with the same Break type)
   |
   |
LL | / fn control_flow_to_control_flow() -> ControlFlow<i64> {
LL | |     ControlFlow::Break(4_u8)?;
   | |                             ^ this `?` produces `ControlFlow<u8, Infallible>`, which is incompatible with `ControlFlow<i64>`
LL | |     //~^ ERROR the `?` operator in a function that returns `ControlFlow<B, _>` can only be used on other `ControlFlow<B, _>`s
LL | |     ControlFlow::Continue(())
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<ControlFlow<u8, Infallible>>` is not implemented for `ControlFlow<i64>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: unlike `Result`, there's no `From`-conversion performed for `ControlFlow`
   = help: the trait `FromResidual` is implemented for `ControlFlow<B, C>`
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
