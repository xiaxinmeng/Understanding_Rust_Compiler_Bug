plain
...........ii...............................i....................................................... 11000/12247
.................................................................................................... 11100/12247
.................................................................................................... 11200/12247
.................................................................................................... 11300/12247
....................................................................F..........F.FF.FF.............. 11400/12247
.................................................................................................... 11600/12247
.................................................................................................... 11700/12247
.................................................................................................... 11800/12247
.................................................................................................... 11900/12247
---

---- [ui] ui/async-await/try-on-option-in-async.rs stdout ----
diff of stderr:

10 LL | |     }
11    | |_____- this function should return `Result` or `Option` to accept `?`
12    |
-    = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `{integer}`
+    = help: the trait `FromResidual<Option<!>>` is not implemented for `{integer}`
14 note: required by `from_residual`
15   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

29 LL | |     };
29 LL | |     };
30    | |_____- this function should return `Result` or `Option` to accept `?`
31    |
-    = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `u32`
+    = help: the trait `FromResidual<Option<!>>` is not implemented for `u32`
33 note: required by `from_residual`
34   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

48 LL | | }
48 LL | | }
49    | |_- this function should return `Result` or `Option` to accept `?`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
50    |
-    = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `u32`
+    = help: the trait `FromResidual<Option<!>>` is not implemented for `u32`
52 note: required by `from_residual`
53   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/try-on-option-in-async/try-on-option-in-async.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/try-on-option-in-async/try-on-option-in-async.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/try-on-option-in-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/try-on-option-in-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/try-on-option-in-async" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/try-on-option-in-async/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the `?` operator can only be used in an async block that returns `Result` or `Option` (or another type that implements `FromResidual`)
   |
LL |       async {
   |  ___________-
   |  ___________-
LL | |         let x: Option<u32> = None;
LL | |         x?; //~ ERROR the `?` operator
   | |          ^ cannot use the `?` operator in an async block that returns `{integer}`
LL | |         22
LL | |     }
   | |_____- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Option<!>>` is not implemented for `{integer}`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used in an async closure that returns `Result` or `Option` (or another type that implements `FromResidual`)
   |
   |
LL |       let async_closure = async || {
   |  __________________________________-
LL | |         let x: Option<u32> = None;
LL | |         x?; //~ ERROR the `?` operator
   | |          ^ cannot use the `?` operator in an async closure that returns `u32`
LL | |         22_u32
LL | |     };
   | |_____- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Option<!>>` is not implemented for `u32`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used in an async function that returns `Result` or `Option` (or another type that implements `FromResidual`)
   |
   |
LL |   async fn an_async_function() -> u32 {
   |  _____________________________________-
LL | |     let x: Option<u32> = None;
LL | |     x?; //~ ERROR the `?` operator
   | |      ^ cannot use the `?` operator in an async function that returns `u32`
LL | |     22
LL | | }
   | |_- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Option<!>>` is not implemented for `u32`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/issues/issue-32709.rs stdout ----
diff of stderr:

7    |           ^ the trait `From<{integer}>` is not implemented for `()`
8    |
9    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
-    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, {integer}>>` for `Result<i32, ()>`
+    = note: required because of the requirements on the impl of `FromResidual<Result<!, {integer}>>` for `Result<i32, ()>`
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
error[E0277]: `?` couldn't convert the error to `()`
   |
   |
LL | fn a() -> Result<i32, ()> {
   |           --------------- expected `()` because of this
LL |     Err(5)?; //~ ERROR
   |           ^ the trait `From<{integer}>` is not implemented for `()`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = note: required because of the requirements on the impl of `FromResidual<Result<!, {integer}>>` for `Result<i32, ()>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
---- [ui] ui/try-block/try-block-bad-type.rs stdout ----
diff of stderr:

6    |
7    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
8    = help: the following implementations were found:
-              <TryFromSliceError as From<Infallible>>
-    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, &str>>` for `Result<u32, TryFromSliceError>`
+              <TryFromSliceError as From<!>>
+    = note: required because of the requirements on the impl of `FromResidual<Result<!, &str>>` for `Result<u32, TryFromSliceError>`
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
error[E0277]: `?` couldn't convert the error to `TryFromSliceError`
   |
   |
LL |         Err("")?; //~ ERROR `?` couldn't convert the error
   |                ^ the trait `From<&str>` is not implemented for `TryFromSliceError`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following implementations were found:
             <TryFromSliceError as From<!>>
   = note: required because of the requirements on the impl of `FromResidual<Result<!, &str>>` for `Result<u32, TryFromSliceError>`
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

10    = help: the following implementations were found:
11              <u8 as From<NonZeroU8>>
12              <u8 as From<bool>>
-    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, i32>>` for `Result<u64, u8>`
+    = note: required because of the requirements on the impl of `FromResidual<Result<!, i32>>` for `Result<u64, u8>`
14 note: required by `from_residual`
15   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL


17 LL |     fn from_residual(residual: R) -> Self;
19 
19 
- error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
+ error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
22    |
22    |
23 LL | / fn option_to_result() -> Result<u64, String> {
24 LL | |     Some(3)?;
24 LL | |     Some(3)?;
-    | |            ^ use `.ok_or(...)?` to provide an error compatible with `Result<u64, String>`
+    | |            ^ this `?` produces `Option<!>`, which is incompatible with `Result<u64, String>`
26 LL | |
27 LL | |     Ok(10)
28 LL | | }

29    | |_- this function returns a `Result`
30    |
-    = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `Result<u64, String>`
+    = help: the trait `FromResidual<Option<!>>` is not implemented for `Result<u64, String>`
32 note: required by `from_residual`
33   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

40    |
40    |
41 LL | / fn control_flow_to_result() -> Result<u64, String> {
42 LL | |     Ok(ControlFlow::Break(123)?)
-    | |                               ^ this `?` produces `ControlFlow<{integer}, Infallible>`, which is incompatible with `Result<u64, String>`
+    | |                               ^ this `?` produces `ControlFlow<{integer}, !>`, which is incompatible with `Result<u64, String>`
44 LL | |
45 LL | | }
46    | |_- this function returns a `Result`
47    |
47    |
-    = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Result<u64, String>`
+    = help: the trait `FromResidual<ControlFlow<{integer}, !>>` is not implemented for `Result<u64, String>`
49 note: required by `from_residual`
50   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

57    |
58 LL | / fn result_to_option() -> Option<u16> {
58 LL | / fn result_to_option() -> Option<u16> {
59 LL | |     Some(Err("hello")?)
-    | |                      ^ use `.ok()?` if you want to discard the `Result<Infallible, &str>` error information
+    | |                      ^ use `.ok()?` if you want to discard the `Result<!, &str>` error information
61 LL | |
62 LL | | }
63    | |_- this function returns an `Option`
64    |
64    |
-    = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `Option<u16>`
+    = help: the trait `FromResidual<Result<!, &str>>` is not implemented for `Option<u16>`
66 note: required by `from_residual`
67   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

74    |
75 LL | / fn control_flow_to_option() -> Option<u64> {
75 LL | / fn control_flow_to_option() -> Option<u64> {
76 LL | |     Some(ControlFlow::Break(123)?)
-    | |                                 ^ this `?` produces `ControlFlow<{integer}, Infallible>`, which is incompatible with `Option<u64>`
+    | |                                 ^ this `?` produces `ControlFlow<{integer}, !>`, which is incompatible with `Option<u64>`
78 LL | |
79 LL | | }
80    | |_- this function returns an `Option`
81    |
81    |
-    = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Option<u64>`
+    = help: the trait `FromResidual<ControlFlow<{integer}, !>>` is not implemented for `Option<u64>`
83 note: required by `from_residual`
84   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

91    |
91    |
92 LL | / fn result_to_control_flow() -> ControlFlow<String> {
93 LL | |     ControlFlow::Continue(Err("hello")?)
-    | |                                       ^ this `?` produces `Result<Infallible, &str>`, which is incompatible with `ControlFlow<String>`
+    | |                                       ^ this `?` produces `Result<!, &str>`, which is incompatible with `ControlFlow<String>`
95 LL | |
96 LL | | }
97    | |_- this function returns a `ControlFlow`
98    |
98    |
-    = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `ControlFlow<String>`
+    = help: the trait `FromResidual<Result<!, &str>>` is not implemented for `ControlFlow<String>`
100 note: required by `from_residual`
101   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

108    |
108    |
109 LL | / fn option_to_control_flow() -> ControlFlow<u64> {
110 LL | |     Some(3)?;
-    | |            ^ this `?` produces `Option<Infallible>`, which is incompatible with `ControlFlow<u64>`
+    | |            ^ this `?` produces `Option<!>`, which is incompatible with `ControlFlow<u64>`
112 LL | |
113 LL | |     ControlFlow::Break(10)
114 LL | | }

115    | |_- this function returns a `ControlFlow`
116    |
-    = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `ControlFlow<u64>`
+    = help: the trait `FromResidual<Option<!>>` is not implemented for `ControlFlow<u64>`
118 note: required by `from_residual`
119   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

126    |
126    |
127 LL | / fn control_flow_to_control_flow() -> ControlFlow<i64> {
128 LL | |     ControlFlow::Break(4_u8)?;
-    | |                             ^ this `?` produces `ControlFlow<u8, Infallible>`, which is incompatible with `ControlFlow<i64>`
+    | |                             ^ this `?` produces `ControlFlow<u8, !>`, which is incompatible with `ControlFlow<i64>`
130 LL | |
131 LL | |     ControlFlow::Continue(())
132 LL | | }

133    | |_- this function returns a `ControlFlow`
134    |
-    = help: the trait `FromResidual<ControlFlow<u8, Infallible>>` is not implemented for `ControlFlow<i64>`
+    = help: the trait `FromResidual<ControlFlow<u8, !>>` is not implemented for `ControlFlow<i64>`
136    = note: unlike `Result`, there's no `From`-conversion performed for `ControlFlow`
137 note: required by `from_residual`
138   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/bad-interconversion.stderr
To update references, rerun the tests and pass the `--bless` flag
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
error[E0277]: `?` couldn't convert the error to `u8`
   |
   |
LL | fn result_to_result() -> Result<u64, u8> {
   |                          --------------- expected `u8` because of this
LL |     Ok(Err(123_i32)?)
   |                    ^ the trait `From<i32>` is not implemented for `u8`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following implementations were found:
             <u8 as From<NonZeroU8>>
             <u8 as From<bool>>
   = note: required because of the requirements on the impl of `FromResidual<Result<!, i32>>` for `Result<u64, u8>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
   |
   |
LL | / fn option_to_result() -> Result<u64, String> {
LL | |     Some(3)?;
   | |            ^ this `?` produces `Option<!>`, which is incompatible with `Result<u64, String>`
LL | |     //~^ ERROR the `?` operator can only be used on `Result`s in a function that returns `Result`
LL | |     Ok(10)
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<Option<!>>` is not implemented for `Result<u64, String>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
   |
   |
LL | / fn control_flow_to_result() -> Result<u64, String> {
LL | |     Ok(ControlFlow::Break(123)?)
   | |                               ^ this `?` produces `ControlFlow<{integer}, !>`, which is incompatible with `Result<u64, String>`
LL | |     //~^ ERROR the `?` operator can only be used on `Result`s in a function that returns `Result`
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<ControlFlow<{integer}, !>>` is not implemented for `Result<u64, String>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
   |
LL | / fn result_to_option() -> Option<u16> {
LL | / fn result_to_option() -> Option<u16> {
LL | |     Some(Err("hello")?)
   | |                      ^ use `.ok()?` if you want to discard the `Result<!, &str>` error information
LL | |     //~^ ERROR the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<Result<!, &str>>` is not implemented for `Option<u16>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `Option`s in a function that returns `Option`
   |
LL | / fn control_flow_to_option() -> Option<u64> {
LL | / fn control_flow_to_option() -> Option<u64> {
LL | |     Some(ControlFlow::Break(123)?)
   | |                                 ^ this `?` produces `ControlFlow<{integer}, !>`, which is incompatible with `Option<u64>`
LL | |     //~^ ERROR the `?` operator can only be used on `Option`s in a function that returns `Option`
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<ControlFlow<{integer}, !>>` is not implemented for `Option<u64>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
   |
   |
LL | / fn result_to_control_flow() -> ControlFlow<String> {
LL | |     ControlFlow::Continue(Err("hello")?)
   | |                                       ^ this `?` produces `Result<!, &str>`, which is incompatible with `ControlFlow<String>`
LL | |     //~^ ERROR the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<Result<!, &str>>` is not implemented for `ControlFlow<String>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
   |
   |
LL | / fn option_to_control_flow() -> ControlFlow<u64> {
LL | |     Some(3)?;
   | |            ^ this `?` produces `Option<!>`, which is incompatible with `ControlFlow<u64>`
LL | |     //~^ ERROR the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
LL | |     ControlFlow::Break(10)
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<Option<!>>` is not implemented for `ControlFlow<u64>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator in a function that returns `ControlFlow<B, _>` can only be used on other `ControlFlow<B, _>`s (with the same Break type)
   |
   |
LL | / fn control_flow_to_control_flow() -> ControlFlow<i64> {
LL | |     ControlFlow::Break(4_u8)?;
   | |                             ^ this `?` produces `ControlFlow<u8, !>`, which is incompatible with `ControlFlow<i64>`
LL | |     //~^ ERROR the `?` operator in a function that returns `ControlFlow<B, _>` can only be used on other `ControlFlow<B, _>`s
LL | |     ControlFlow::Continue(())
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<ControlFlow<u8, !>>` is not implemented for `ControlFlow<i64>`
   = note: unlike `Result`, there's no `From`-conversion performed for `ControlFlow`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/try-trait/try-operator-on-main.rs stdout ----
diff of stderr:

11 LL | | }
12    | |_- this function should return `Result` or `Option` to accept `?`
13    |
-    = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
+    = help: the trait `FromResidual<Result<!, std::io::Error>>` is not implemented for `()`
15 note: required by `from_residual`
16   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/try-operator-on-main/try-operator-on-main.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/try-operator-on-main/try-operator-on-main.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-trait/try-operator-on-main.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-trait/try-operator-on-main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/try-operator-on-main" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/try-operator-on-main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
   |
LL | / fn main() {
LL | / fn main() {
LL | |     // error for a `Try` type on a non-`Try` fn
LL | |     std::fs::File::open("foo")?; //~ ERROR the `?` operator can only
   | |                               ^ cannot use the `?` operator in a function that returns `()`
LL | |
...  |
LL | |     try_trait_generic::<()>(); //~ ERROR the trait bound
LL | | }
   | |_- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Result<!, std::io::Error>>` is not implemented for `()`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;

error[E0277]: the `?` operator can only be applied to values that implement `Try`
  --> /checkout/src/test/ui/try-trait/try-operator-on-main.rs:10:5
   |
   |
LL |     ()?; //~ ERROR the `?` operator can only be applied to
   |     ^^^ the `?` operator cannot be applied to type `()`
   |
   = help: the trait `Try` is not implemented for `()`
note: required by `branch`
   |
   |
LL |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;


error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
   |
LL | / fn main() {
LL | / fn main() {
LL | |     // error for a `Try` type on a non-`Try` fn
LL | |     std::fs::File::open("foo")?; //~ ERROR the `?` operator can only
LL | |
LL | |     // a non-`Try` type on a non-`Try` fn
LL | |     ()?; //~ ERROR the `?` operator can only be applied to
   | |       ^ cannot use the `?` operator in a function that returns `()`
...  |
LL | |     try_trait_generic::<()>(); //~ ERROR the trait bound
LL | | }
   | |_- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<_>` is not implemented for `()`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the trait bound `(): Try` is not satisfied
   |
   |
LL |     try_trait_generic::<()>(); //~ ERROR the trait bound
   |                         ^^ the trait `Try` is not implemented for `()`
   |
note: required by a bound in `try_trait_generic`
   |
   |
LL | fn try_trait_generic<T: Try>() -> T {
   |                         ^^^ required by this bound in `try_trait_generic`
error[E0277]: the `?` operator can only be applied to values that implement `Try`
  --> /checkout/src/test/ui/try-trait/try-operator-on-main.rs:19:5
   |
   |
LL |     ()?; //~ ERROR the `?` operator can only be applied to values that implement `Try`
   |     ^^^ the `?` operator cannot be applied to type `()`
   |
   = help: the trait `Try` is not implemented for `()`
note: required by `branch`
   |
   |
LL |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/try-trait/try-on-option-diagnostics.rs stdout ----
diff of stderr:

9 LL | | }
10    | |_- this function should return `Result` or `Option` to accept `?`
11    |
-    = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `u32`
+    = help: the trait `FromResidual<Option<!>>` is not implemented for `u32`
13 note: required by `from_residual`
14   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

28 LL | |     };
28 LL | |     };
29    | |_____- this function should return `Result` or `Option` to accept `?`
30    |
-    = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `{integer}`
+    = help: the trait `FromResidual<Option<!>>` is not implemented for `{integer}`
32 note: required by `from_residual`
33   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

45 LL | |         }
45 LL | |         }
46    | |_________- this function should return `Result` or `Option` to accept `?`
47    |
-    = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `()`
+    = help: the trait `FromResidual<Option<!>>` is not implemented for `()`
49 note: required by `from_residual`
50   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL

62 LL | |         }
62 LL | |         }
63    | |_________- this function should return `Result` or `Option` to accept `?`
64    |
-    = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `()`
+    = help: the trait `FromResidual<Option<!>>` is not implemented for `()`
66 note: required by `from_residual`
67   --> $SRC_DIR/core/src/ops/try_trait.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/try-on-option-diagnostics/try-on-option-diagnostics.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/try-on-option-diagnostics/try-on-option-diagnostics.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-trait/try-on-option-diagnostics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-trait/try-on-option-diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/try-on-option-diagnostics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/try-on-option-diagnostics/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
   |
   |
LL | / fn a_function() -> u32 {
LL | |     let x: Option<u32> = None;
LL | |     x?; //~ ERROR the `?` operator
   | |      ^ cannot use the `?` operator in a function that returns `u32`
LL | |     22
LL | | }
   | |_- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Option<!>>` is not implemented for `u32`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used in a closure that returns `Result` or `Option` (or another type that implements `FromResidual`)
   |
   |
LL |       let a_closure = || {
   |  _____________________-
LL | |         let x: Option<u32> = None;
LL | |         x?; //~ ERROR the `?` operator
   | |          ^ cannot use the `?` operator in a closure that returns `{integer}`
LL | |         22
LL | |     };
   | |_____- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Option<!>>` is not implemented for `{integer}`
note: required by `from_residual`
   |
   |
---
test result: FAILED. 12124 passed; 8 failed; 115 ignored; 0 measured; 0 filtered out; finished in 126.14s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:51
