plain
........ii...............................i.......................................................... 11000/12243
.................................................................................................... 11100/12243
.................................................................................................... 11200/12243
.................................................................................................... 11300/12243
...............................................................F.....................F.............. 11400/12243
.................................................................................................... 11600/12243
.................................................................................................... 11700/12243
.................................................................................................... 11800/12243
.................................................................................................... 11900/12243
---

---- [ui] ui/inference/cannot-infer-closure.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for the closure `fn((), ()) -> Result<(), _>`
-   --> $DIR/cannot-infer-closure.rs:4:9
+ error[E0283]: type annotations needed
3    |
- LL |         Ok(b)
- LL |         Ok(b)
-    |         ^^ cannot infer type for type parameter `E` declared on the enum `Result`
+ LL |         Err(a)?;
+    |               ^ cannot infer type for enum `Result<(), _>`
6    |
- help: give this closure an explicit return type without `_` placeholders
+    = note: cannot satisfy `Result<(), _>: FromResidual<Result<Infallible, ()>>`
+ note: required by `from_residual`
+   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL
8    |
- LL |     let x = |a: (), b: ()| -> Result<(), _> {
-    |                            ++++++++++++++++
+ LL |     fn from_residual(residual: R) -> Self;
11 
12 error: aborting due to previous error
13 

---
To only update this specific test, also pass `--test-args inference/cannot-infer-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/cannot-infer-closure.rs:3:15
   |
LL |         Err(a)?;
   |               ^ cannot infer type for enum `Result<(), _>`
   |
   = note: cannot satisfy `Result<(), _>: FromResidual<Result<Infallible, ()>>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
---
+ error[E0283]: type annotations needed
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/cannot-infer-async.rs:11:20
3    |
- LL |     let fut = async {
-    |         --- consider giving `fut` a type
- LL |         Ok(())
- LL |         Ok(())
-    |         ^^ cannot infer type for type parameter `E` declared on the enum `Result`
+ LL |         make_unit()?;
+    |                    ^ cannot infer type for enum `Result<(), _>`
+    |
+    = note: cannot satisfy `Result<(), _>: FromResidual<Result<Infallible, std::io::Error>>`
+ note: required by `from_residual`
+   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL
+    |
+ LL |     fn from_residual(residual: R) -> Self;
9 
10 error: aborting due to previous error
11 

---
To only update this specific test, also pass `--test-args inference/cannot-infer-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-async" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-async/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/cannot-infer-async.rs:11:20
   |
LL |         make_unit()?;
   |                    ^ cannot infer type for enum `Result<(), _>`
   |
   = note: cannot satisfy `Result<(), _>: FromResidual<Result<Infallible, std::io::Error>>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
---
-   --> $DIR/cannot-infer-closure-circular.rs:7:14
+ error[E0283]: type annotations needed
+   --> $DIR/cannot-infer-closure-circular.rs:9:17
3    |
- LL |     let x = |r| {
-    |              ^ consider giving this closure parameter the explicit type `Result<(), E>`, where the type parameter `E` is specified
+ LL |         let v = r?;
+    |                 ^^ cannot infer type for enum `Result<(), _>`
+    |
+    = note: cannot satisfy `Result<(), _>: FromResidual<Result<Infallible, _>>`
+ note: required by `branch`
+   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL
+    |
+ LL |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args inference/cannot-infer-closure-circular.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-closure-circular.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure-circular" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure-circular/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/cannot-infer-closure-circular.rs:9:17
   |
LL |         let v = r?;
   |                 ^^ cannot infer type for enum `Result<(), _>`
   |
   = note: cannot satisfy `Result<(), _>: FromResidual<Result<Infallible, _>>`
note: required by `branch`
   |
   |
LL |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
For more information about this error, try `rustc --explain E0283`.

------------------------------------------


---- [ui] ui/issues/issue-32709.rs stdout ----
diff of stderr:

- error[E0277]: `?` couldn't convert the error to `()`
+ error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
3    |
3    |
- LL | fn a() -> Result<i32, ()> {
-    |           --------------- expected `()` because of this
- LL |     Err(5)?;
-    |           ^ the trait `From<{integer}>` is not implemented for `()`
+ LL | / fn a() -> Result<i32, ()> {
+ LL | |     Err(5)?;
+    | |           ^ this `?` produces `Result<Infallible, {integer}>`, which is incompatible with `Result<i32, ()>`
+ LL | |     Ok(1)
+ LL | | }
+    | |_- this function returns a `Result`
8    |
-    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
-    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, {integer}>>` for `Result<i32, ()>`
+    = help: the trait `FromResidual<Result<Infallible, {integer}>>` is not implemented for `Result<i32, ()>`
11 note: required by `from_residual`
12   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/issue-32709.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/issue-32709.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-32709.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32709.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
   |
   |
LL | / fn a() -> Result<i32, ()> {
LL | |     Err(5)?; //~ ERROR
   | |           ^ this `?` produces `Result<Infallible, {integer}>`, which is incompatible with `Result<i32, ()>`
LL | |     Ok(1)
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<Result<Infallible, {integer}>>` is not implemented for `Result<i32, ()>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/try-block/try-block-bad-type.rs stdout ----
diff of stderr:

- error[E0277]: `?` couldn't convert the error to `TryFromSliceError`
+ error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
2   --> $DIR/try-block-bad-type.rs:7:16
- LL |         Err("")?;
- LL |         Err("")?;
-    |                ^ the trait `From<&str>` is not implemented for `TryFromSliceError`
+ LL | / pub fn main() {
+ LL | |     let res: Result<u32, std::array::TryFromSliceError> = try {
+ LL | |         Err("")?;
+    | |                ^ this `?` produces `Result<Infallible, &str>`, which is incompatible with `Result<u32, TryFromSliceError>`
+ LL | |         5
+ ...  |
+ LL | |     let res: i32 = try { 5 };
+ LL | | }
+    | |_- this function returns a `Result`
6    |
-    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
-    = help: the following implementations were found:
-              <TryFromSliceError as From<Infallible>>
-    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, &str>>` for `Result<u32, TryFromSliceError>`
+    = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `Result<u32, TryFromSliceError>`
11 note: required by `from_residual`
12   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-block/try-block-bad-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-bad-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
   |
LL | / pub fn main() {
LL | / pub fn main() {
LL | |     let res: Result<u32, std::array::TryFromSliceError> = try {
LL | |         Err("")?; //~ ERROR `?` couldn't convert the error
   | |                ^ this `?` produces `Result<Infallible, &str>`, which is incompatible with `Result<u32, TryFromSliceError>`
LL | |         5
...  |
LL | |     let res: i32 = try { 5 }; //~ ERROR a `try` block must return `Result` or `Option`
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `Result<u32, TryFromSliceError>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0271]: type mismatch resolving `<Result<i32, i32> as Try>::Output == &str`
   |
   |
LL |         "" //~ ERROR type mismatch
   |         ^^ expected `i32`, found `&str`

error[E0271]: type mismatch resolving `<Result<i32, i32> as Try>::Output == ()`
   |
   |
LL |     let res: Result<i32, i32> = try { }; //~ ERROR type mismatch
   |                                       ^ expected `i32`, found `()`

error[E0277]: a `try` block must return `Result` or `Option` (or another type that implements `Try`)
   |
   |
LL |     let res: () = try { };
   |                         ^ could not wrap the final value of the block as `()` doesn't implement `Try`
   |
   = help: the trait `Try` is not implemented for `()`
note: required by `from_output`
   |
   |
LL |     fn from_output(output: Self::Output) -> Self;


error[E0277]: a `try` block must return `Result` or `Option` (or another type that implements `Try`)
   |
   |
LL |     let res: i32 = try { 5 }; //~ ERROR a `try` block must return `Result` or `Option`
   |                          ^ could not wrap the final value of the block as `i32` doesn't implement `Try`
   = help: the trait `Try` is not implemented for `i32`
note: required by `from_output`
  --> /checkout/library/core/src/ops/try_trait.rs:191:5
   |
   |
LL |     fn from_output(output: Self::Output) -> Self;

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0271, E0277.
---

---- [ui] ui/try-trait/bad-interconversion.rs stdout ----
diff of stderr:

- error[E0277]: `?` couldn't convert the error to `u8`
+ error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
3    |
3    |
- LL | fn result_to_result() -> Result<u64, u8> {
-    |                          --------------- expected `u8` because of this
- LL |     Ok(Err(123_i32)?)
-    |                    ^ the trait `From<i32>` is not implemented for `u8`
+ LL | / fn result_to_result() -> Result<u64, u8> {
+ LL | |     Ok(Err(123_i32)?)
+    | |                    ^ this `?` produces `Result<Infallible, i32>`, which is incompatible with `Result<u64, u8>`
+ LL | |
+ LL | | }
+    | |_- this function returns a `Result`
8    |
-    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
-    = help: the following implementations were found:
-              <u8 as From<NonZeroU8>>
-              <u8 as From<bool>>
-    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, i32>>` for `Result<u64, u8>`
+    = help: the trait `FromResidual<Result<Infallible, i32>>` is not implemented for `Result<u64, u8>`
14 note: required by `from_residual`
15   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/bad-interconversion.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/bad-interconversion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-trait/bad-interconversion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-trait/bad-interconversion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
   |
   |
LL | / fn result_to_result() -> Result<u64, u8> {
LL | |     Ok(Err(123_i32)?)
   | |                    ^ this `?` produces `Result<Infallible, i32>`, which is incompatible with `Result<u64, u8>`
LL | |     //~^ ERROR `?` couldn't convert the error to `u8`
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<Result<Infallible, i32>>` is not implemented for `Result<u64, u8>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


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
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


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
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


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
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


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
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


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
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


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
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


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
   = note: unlike `Result`, there's no `From`-conversion performed for `ControlFlow`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0277`.
---
test result: FAILED. 12122 passed; 6 failed; 115 ignored; 0 measured; 0 filtered out; finished in 131.35s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:59
