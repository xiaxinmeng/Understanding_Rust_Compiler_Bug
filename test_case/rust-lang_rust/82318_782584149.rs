plain
.................................................................................................... 4000/11475
.................................................................................................... 4100/11475
.................................................................................................... 4200/11475
.................................................................................................... 4300/11475
.......................................................ii..............F.F.......................... 4400/11475
.................................................................................................... 4600/11475
.................................................................................................... 4700/11475
.................................................................................................... 4800/11475
.................................................................................................... 4900/11475
---
.................................................................................................... 9200/11475
.................................................................................................... 9300/11475
.................................................................................................... 9400/11475
.............................i......i............................................................... 9500/11475
......................................F.............................iiiiiii..iiiiii.i............... 9600/11475
.................................................................................................... 9800/11475
.................................................................................................... 9900/11475
.................................................................................................... 10000/11475
.................................................................................................... 10100/11475
---

---- [ui] ui/async-await/issue-61076.rs stdout ----
diff of stderr:

- error[E0277]: the `?` operator can only be applied to values that implement `Try2021`
+ error[E0277]: the `?` operator can only be applied to values that implement `Try`
3    |
4 LL |     foo()?;


5    |     ^^^^^^ the `?` operator cannot be applied to type `impl Future`
6    |
-    = help: the trait `Try2021` is not implemented for `impl Future`
+    = help: the trait `Try` is not implemented for `impl Future`
8    = note: required by `branch`
9 
- error[E0277]: the `?` operator can only be applied to values that implement `Try2021`
+ error[E0277]: the `?` operator can only be applied to values that implement `Try`
12    |
13 LL |     t?;

14    |     ^^ the `?` operator cannot be applied to type `T`
14    |     ^^ the `?` operator cannot be applied to type `T`
15    |
-    = help: the trait `Try2021` is not implemented for `T`
+    = help: the trait `Try` is not implemented for `T`
17    = note: required by `branch`
18 
19 error[E0609]: no field `0` on type `impl Future`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076/issue-61076.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-61076.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the `?` operator can only be applied to values that implement `Try`
   |
   |
LL |     foo()?; //~ ERROR the `?` operator can only be applied to values that implement `Try2021`
   |     ^^^^^^ the `?` operator cannot be applied to type `impl Future`
   |
   = help: the trait `Try` is not implemented for `impl Future`
   = note: required by `branch`

error[E0277]: the `?` operator can only be applied to values that implement `Try`
   |
   |
LL |     t?; //~ ERROR the `?` operator can only be applied to values that implement `Try2021`
   |     ^^ the `?` operator cannot be applied to type `T`
   |
   = help: the trait `Try` is not implemented for `T`
   = note: required by `branch`

error[E0609]: no field `0` on type `impl Future`
   |
   |
LL |     let _: i32 = tuple().0; //~ ERROR no field `0`
   |                          ^ field not available in `impl Future`, but it is available in its `Output`
   |
help: consider `await`ing on the `Future` and access the field of its `Output`
   |
LL |     let _: i32 = tuple().await.0; //~ ERROR no field `0`


error[E0609]: no field `a` on type `impl Future`
   |
   |
LL |     let _: i32 = struct_().a; //~ ERROR no field `a`
   |                            ^ field not available in `impl Future`, but it is available in its `Output`
   |
help: consider `await`ing on the `Future` and access the field of its `Output`
   |
LL |     let _: i32 = struct_().await.a; //~ ERROR no field `a`


error[E0599]: no method named `method` found for opaque type `impl Future` in the current scope
   |
   |
LL |     struct_().method(); //~ ERROR no method named
   |               ^^^^^^ method not found in `impl Future`
   |
help: consider `await`ing on the `Future` and calling the method on its `Output`
   |
LL |     struct_().await.method(); //~ ERROR no method named

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/issue-61076.rs:87:9
   |
   |
LL | async fn tuple() -> Tuple {
   |                     ----- the `Output` of this `async fn`'s expected opaque type
...
LL |         Tuple(_) => {} //~ ERROR mismatched types
   |         ^^^^^^^^ expected opaque type, found struct `Tuple`
   |
   = note: expected opaque type `impl Future`
                   found struct `Tuple`
help: consider `await`ing on the `Future`
   |
LL |     match tuple().await { //~ HELP consider `await`ing on the `Future`

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0277, E0308, E0599, E0609.
---
diff of stderr:

2   --> $DIR/cannot-infer-closure-circular.rs:7:14
3    |
4 LL |     let x = |r| {
-    |              ^ consider giving this closure parameter the explicit type `Result<(), E>`, with the type parameters specified
+    |              ^ consider giving this closure parameter the explicit type `Result<(), E>`, where the type parameter `E` is specified
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure-circular/cannot-infer-closure-circular.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/cannot-infer-closure-circular.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-closure-circular.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure-circular" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure-circular/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed for `Result<(), E>`
   |
   |
LL |     let x = |r| {
   |              ^ consider giving this closure parameter the explicit type `Result<(), E>`, where the type parameter `E` is specified
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/inference/cannot-infer-closure.rs stdout ----
diff of stderr:

- <<<<<<< HEAD
2 error[E0282]: type annotations needed for the closure `fn((), ()) -> Result<(), _>`
-   --> $DIR/cannot-infer-closure.rs:3:15
- ||||||| parent of f4e89f77bb1 (PoC: A new hybrid design for Try)
- error[E0282]: type annotations needed for the closure `fn((), ()) -> std::result::Result<(), _>`
-   --> $DIR/cannot-infer-closure.rs:3:15
- =======
- error[E0282]: type annotations needed for the closure `fn((), ()) -> std::result::Result<(), _>`
9   --> $DIR/cannot-infer-closure.rs:4:9
- >>>>>>> f4e89f77bb1 (PoC: A new hybrid design for Try)
12 LL |         Ok(b)
12 LL |         Ok(b)
13    |         ^^ cannot infer type for type parameter `E` declared on the enum `Result`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure/cannot-infer-closure.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure/cannot-infer-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/cannot-infer-closure.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed for the closure `fn((), ()) -> Result<(), _>`
   |
   |
LL |         Ok(b) //~ ERROR type annotations needed for the closure
   |         ^^ cannot infer type for type parameter `E` declared on the enum `Result`
   |
help: give this closure an explicit return type without `_` placeholders
   |
LL |     let x = |a: (), b: ()| -> Result<(), _> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.

------------------------------------------


---- [ui] ui/issues/issue-32709.rs stdout ----
diff of stderr:

7    |     ^^^^^^^ the trait `From<{integer}>` is not implemented for `()`
8    |
9    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
-    = note: required because of the requirements on the impl of `FromResidual<std::result::Result<!, {integer}>>` for `std::result::Result<i32, ()>`
+    = note: required because of the requirements on the impl of `FromResidual<Result<!, {integer}>>` for `Result<i32, ()>`
11    = note: required by `from_residual`
13 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/issue-32709.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-32709.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32709.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/auxiliary"
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
   |     ^^^^^^^ the trait `From<{integer}>` is not implemented for `()`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = note: required because of the requirements on the impl of `FromResidual<Result<!, {integer}>>` for `Result<i32, ()>`
   = note: required by `from_residual`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/option-to-result.rs stdout ----
diff of stderr:

8    |     ^^ the trait `From<result::sadness::PleaseCallTheOkOrMethodToUseQuestionMarkOnOptionsInAMethodThatReturnsResult>` is not implemented for `()`
9    |
10    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
-    = note: required because of the requirements on the impl of `FromResidual<Option<!>>` for `std::result::Result<(), ()>`
+    = note: required because of the requirements on the impl of `FromResidual<Option<!>>` for `Result<(), ()>`
12    = note: required by `from_residual`
13 
14 error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
22 LL | | }
22 LL | | }
23    | |_- this function should return `Result` or `Option` to accept `?`
24    |
-    = help: the trait `FromResidual<std::result::Result<!, i32>>` is not implemented for `Option<i32>`
+    = help: the trait `FromResidual<Result<!, i32>>` is not implemented for `Option<i32>`
26    = note: required by `from_residual`
28 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/option-to-result/option-to-result.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args option-to-result.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/option-to-result.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/option-to-result" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/option-to-result/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `?` couldn't convert the error to `()`
  --> /checkout/src/test/ui/option-to-result.rs:5:5
   |
LL | fn test_result() -> Result<(),()> {
   |                     ------------- expected `()` because of this
LL |     let a:Option<()> = Some(());
LL |     a?;//~ ERROR `?` couldn't convert the error to `()`
   |     ^^ the trait `From<result::sadness::PleaseCallTheOkOrMethodToUseQuestionMarkOnOptionsInAMethodThatReturnsResult>` is not implemented for `()`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = note: required because of the requirements on the impl of `FromResidual<Option<!>>` for `Result<(), ()>`
   = note: required by `from_residual`

error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
  --> /checkout/src/test/ui/option-to-result.rs:11:5
   |
LL | / fn test_option() -> Option<i32>{
LL | |     let a:Result<i32, i32> = Ok(5);
LL | |     a?;//~ ERROR the `?` operator can only be used in a function that returns `Result` or `Option`
   | |     ^^ cannot use the `?` operator in a function that returns `Option<i32>`
LL | |     Some(5)
LL | | }
   | |_- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Result<!, i32>>` is not implemented for `Option<i32>`
   = note: required by `from_residual`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs stdout ----
diff of stderr:

551 LL |     if -let 0 = 0 {}
552    |        ^^^^^^^^^^ cannot apply unary operator `-`
553 
- error[E0277]: the `?` operator can only be applied to values that implement `Try2021`
+ error[E0277]: the `?` operator can only be applied to values that implement `Try`
556    |
556    |
557 LL |     if (let 0 = 0)? {}
558    |        ^^^^^^^^^^^^ the `?` operator cannot be applied to type `bool`
559    |
559    |
-    = help: the trait `Try2021` is not implemented for `bool`
+    = help: the trait `Try` is not implemented for `bool`
561    = note: required by `branch`
562 
563 error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
709    = note: expected type `bool`
710             found struct `std::ops::Range<bool>`
711 
711 
- error[E0277]: the `?` operator can only be applied to values that implement `Try2021`
+ error[E0277]: the `?` operator can only be applied to values that implement `Try`
714    |
714    |
715 LL |         if let 0 = 0? {}

716    |                    ^^ the `?` operator cannot be applied to type `{integer}`
717    |
-    = help: the trait `Try2021` is not implemented for `{integer}`
+    = help: the trait `Try` is not implemented for `{integer}`
719    = note: required by `branch`
721 error[E0308]: mismatched types


739 LL |     while -let 0 = 0 {}
740    |           ^^^^^^^^^^ cannot apply unary operator `-`
741 
- error[E0277]: the `?` operator can only be applied to values that implement `Try2021`
+ error[E0277]: the `?` operator can only be applied to values that implement `Try`
744    |
744    |
745 LL |     while (let 0 = 0)? {}
746    |           ^^^^^^^^^^^^ the `?` operator cannot be applied to type `bool`
747    |
747    |
-    = help: the trait `Try2021` is not implemented for `bool`
+    = help: the trait `Try` is not implemented for `bool`
749    = note: required by `branch`
750 
751 error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
897    = note: expected type `bool`
898             found struct `std::ops::Range<bool>`
899 
899 
- error[E0277]: the `?` operator can only be applied to values that implement `Try2021`
+ error[E0277]: the `?` operator can only be applied to values that implement `Try`
902    |
902    |
903 LL |         while let 0 = 0? {}

904    |                       ^^ the `?` operator cannot be applied to type `{integer}`
905    |
-    = help: the trait `Try2021` is not implemented for `{integer}`
+    = help: the trait `Try` is not implemented for `{integer}`
907    = note: required by `branch`
908 
909 error[E0614]: type `bool` cannot be dereferenced

918 LL |     -let 0 = 0;
919    |     ^^^^^^^^^^ cannot apply unary operator `-`
920 
- error[E0277]: the `?` operator can only be applied to values that implement `Try2021`
+ error[E0277]: the `?` operator can only be applied to values that implement `Try`
923    |
923    |
924 LL |     (let 0 = 0)?;
925    |     ^^^^^^^^^^^^ the `?` operator cannot be applied to type `bool`
926    |
926    |
-    = help: the trait `Try2021` is not implemented for `bool`
+    = help: the trait `Try` is not implemented for `bool`
928    = note: required by `branch`
929 
930 error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)

965 LL |     &let 0 = 0
966    |     ^^^^^^^^^^ expected `()`, found `&bool`
967 
- error[E0277]: the `?` operator can only be applied to values that implement `Try2021`
+ error[E0277]: the `?` operator can only be applied to values that implement `Try`
970    |
970    |
971 LL |         let 0 = 0?;

972    |                 ^^ the `?` operator cannot be applied to type `{integer}`
973    |
-    = help: the trait `Try2021` is not implemented for `{integer}`
+    = help: the trait `Try` is not implemented for `{integer}`
975    = note: required by `branch`
977 error: aborting due to 104 previous errors; 2 warnings emitted


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/disallowed-positions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expressions must be enclosed in braces to be used as const generic arguments
   |
   |
LL |         true && let 1 = 1
   |
   |
help: enclose the `const` expression in braces
   |
LL |         { true && let 1 = 1 }
   |         ^                   ^

error: `let` expressions are not supported here
   |
   |
LL |     if &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: _, end: _ } = true..true && false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: _, end: _ } = true..true || false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: F, end } = F..|| true {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: true, end } = t..&&false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
---
test result: FAILED. 11375 passed; 7 failed; 93 ignored; 0 measured; 0 filtered out; finished in 140.91s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:27
