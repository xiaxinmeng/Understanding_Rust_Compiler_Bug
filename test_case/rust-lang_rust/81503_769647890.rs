plain
.................................................................................................... 2100/11312
....................................................................................F............... 2200/11312
.................................................................................................... 2300/11312
.................................................................................................... 2400/11312
.............................................................................................F.F.F.. 2500/11312
.F..........................................i..i.................................................... 2600/11312
...............................................iiiii................................................ 2800/11312
.................................................................................................... 2900/11312
.................................................................................................... 3000/11312
.................................................................................................... 3100/11312
---

---- [ui] ui/const-generics/issues/issue-61336.rs#full stdout ----
diff of stderr:

13 LL |     [x; N]
14    |     ^^^^^^ the trait `Copy` is not implemented for `T`
15    |
-    = note: the `Copy` trait is required because the repeated element will be copied
17 help: consider restricting type parameter `T`
18    |
19 LL | fn g<T: Copy, const N: usize>(x: T) -> [T; N] {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336.full/issue-61336.full.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-61336.rs`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error in revision `full`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-61336.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `const_generics` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full, feature(const_generics))] //[full]~WARN the feature `const_generics` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information


error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     [x; N]
   |     ^^^^^^ the trait `Copy` is not implemented for `T`
help: consider restricting type parameter `T`
   |
   |
LL | fn g<T: Copy, const N: usize>(x: T) -> [T; N] {

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/const-generics/issues/issue-61336-2.rs#full stdout ----
diff of stderr:

13 LL |     [x; { N }]
14    |     ^^^^^^^^^^ the trait `Copy` is not implemented for `T`
15    |
-    = note: the `Copy` trait is required because the repeated element will be copied
17 help: consider restricting type parameter `T`
18    |
19 LL | fn g<T: Copy, const N: usize>(x: T) -> [T; N] {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336-2.full/issue-61336-2.full.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-61336-2.rs`

error in revision `full`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-61336-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336-2.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336-2.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `const_generics` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full, feature(const_generics))] //[full]~WARN the feature `const_generics` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information


error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     [x; { N }]
   |     ^^^^^^^^^^ the trait `Copy` is not implemented for `T`
help: consider restricting type parameter `T`
   |
   |
LL | fn g<T: Copy, const N: usize>(x: T) -> [T; N] {

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/const-generics/issues/issue-61336.rs#min stdout ----
diff of stderr:

4 LL |     [x; N]
5    |     ^^^^^^ the trait `Copy` is not implemented for `T`
6    |
-    = note: the `Copy` trait is required because the repeated element will be copied
8 help: consider restricting type parameter `T`
9    |
10 LL | fn g<T: Copy, const N: usize>(x: T) -> [T; N] {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336.min/issue-61336.min.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-61336.rs`

error in revision `min`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-61336.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     [x; N]
   |     ^^^^^^ the trait `Copy` is not implemented for `T`
help: consider restricting type parameter `T`
   |
   |
LL | fn g<T: Copy, const N: usize>(x: T) -> [T; N] {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/const-generics/issues/issue-61336-2.rs#min stdout ----
diff of stderr:

4 LL |     [x; { N }]
5    |     ^^^^^^^^^^ the trait `Copy` is not implemented for `T`
6    |
-    = note: the `Copy` trait is required because the repeated element will be copied
8 help: consider restricting type parameter `T`
9    |
10 LL | fn g<T: Copy, const N: usize>(x: T) -> [T; N] {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336-2.min/issue-61336-2.min.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-61336-2.rs`

error in revision `min`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-61336-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336-2.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336-2.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     [x; { N }]
   |     ^^^^^^^^^^ the trait `Copy` is not implemented for `T`
help: consider restricting type parameter `T`
   |
   |
LL | fn g<T: Copy, const N: usize>(x: T) -> [T; N] {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
1 error[E0277]: the trait bound `String: Copy` is not satisfied
-   --> $DIR/const-fn-in-vec.rs:3:32
+   --> $DIR/const-fn-in-vec.rs:4:32
3    |
4 LL |     let strings: [String; 5] = [String::new(); 5];
5    |                                ^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `String`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-in-vec/const-fn-in-vec.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-in-vec/const-fn-in-vec.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-fn-in-vec.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-fn-in-vec.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-in-vec" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-in-vec/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `String: Copy` is not satisfied
  --> /checkout/src/test/ui/consts/const-fn-in-vec.rs:4:32
   |
LL |     let strings: [String; 5] = [String::new(); 5];
   |                                ^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `String`
   |
   = help: creating a new `const` item will resolve this error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const.rs stdout ----
diff of stderr:

6    |
7    = help: the following implementations were found:
8              <Option<T> as Copy>
-    = note: the `Copy` trait is required because the repeated element will be copied
+    = help: creating a new `const` item will resolve this error
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const/fn-call-in-non-const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/fn-call-in-non-const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Option<Bar>: Copy` is not satisfied
   |
   |
LL |     let _: [Option<Bar>; 2] = [no_copy(); 2];
   |                               ^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `Option<Bar>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Option<T> as Copy>
   = help: creating a new `const` item will resolve this error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/consts/rfc-2203-const-array-repeat-exprs/migrate-fail.rs stdout ----
diff of stderr:

6    |
7    = help: the following implementations were found:
8              <Option<T> as Copy>
-    = note: the `Copy` trait is required because the repeated element will be copied
10 
11 error[E0277]: the trait bound `Option<Bar>: Copy` is not satisfied

16    |
17    = help: the following implementations were found:
17    = help: the following implementations were found:
18              <Option<T> as Copy>
-    = note: the `Copy` trait is required because the repeated element will be copied
21 error: aborting due to 2 previous errors
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/migrate-fail/migrate-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/rfc-2203-const-array-repeat-exprs/migrate-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/migrate-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/migrate-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=migrate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/migrate-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Option<Bar>: Copy` is not satisfied
   |
   |
LL |         let arr: [Option<Bar>; 2] = [x; 2];
   |                                     ^^^^^^ the trait `Copy` is not implemented for `Option<Bar>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Option<T> as Copy>

error[E0277]: the trait bound `Option<Bar>: Copy` is not satisfied
   |
   |
LL |         let arr: [Option<Bar>; 2] = [x; 2];
   |                                     ^^^^^^ the trait `Copy` is not implemented for `Option<Bar>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Option<T> as Copy>
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/consts/rfc-2203-const-array-repeat-exprs/nll-fail.rs stdout ----
diff of stderr:

6    |
7    = help: the following implementations were found:
8              <Option<T> as Copy>
-    = note: the `Copy` trait is required because the repeated element will be copied
10 
11 error[E0277]: the trait bound `Option<Bar>: Copy` is not satisfied
12   --> $DIR/nll-fail.rs:19:37
16    |
17    = help: the following implementations were found:
17    = help: the following implementations were found:
18              <Option<T> as Copy>
-    = note: the `Copy` trait is required because the repeated element will be copied
21 error: aborting due to 2 previous errors
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/nll-fail/nll-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/rfc-2203-const-array-repeat-exprs/nll-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/nll-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/nll-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/nll-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Option<Bar>: Copy` is not satisfied
   |
   |
LL |         let arr: [Option<Bar>; 2] = [x; 2];
   |                                     ^^^^^^ the trait `Copy` is not implemented for `Option<Bar>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Option<T> as Copy>

error[E0277]: the trait bound `Option<Bar>: Copy` is not satisfied
   |
   |
LL |         let arr: [Option<Bar>; 2] = [x; 2];
   |                                     ^^^^^^ the trait `Copy` is not implemented for `Option<Bar>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Option<T> as Copy>
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/consts/rfc-2203-const-array-repeat-exprs/trait-error.rs stdout ----
diff of stderr:

6    |
7    = help: the following implementations were found:
8              <Foo<T> as Copy>
-    = note: the `Copy` trait is required because the repeated element will be copied
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error/trait-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/rfc-2203-const-array-repeat-exprs/trait-error.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Foo<String>: Copy` is not satisfied
   |
   |
LL |     [Foo(String::new()); 4];
   |     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `Foo<String>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Foo<T> as Copy>
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-const_in_array_repeat_expressions.rs stdout ----
diff of stderr:

6    |
7    = help: the following implementations were found:
8              <Option<T> as Copy>
-    = note: the `Copy` trait is required because the repeated element will be copied
-    = note: this array initializer can be evaluated at compile-time, see issue #49147 <https://github.com/rust-lang/rust/issues/49147> for more information
12 
12 
13 error[E0277]: the trait bound `Option<String>: Copy` is not satisfied

18    |
19    = help: the following implementations were found:
19    = help: the following implementations were found:
20              <Option<T> as Copy>
-    = note: the `Copy` trait is required because the repeated element will be copied
23 error: aborting due to 2 previous errors
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions/feature-gate-const_in_array_repeat_expressions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-const_in_array_repeat_expressions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_in_array_repeat_expressions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Option<String>: Copy` is not satisfied
   |
   |
LL |     let arr: [Option<String>; 2] = [None::<String>; 2];
   |                                    ^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `Option<String>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Option<T> as Copy>

error[E0277]: the trait bound `Option<String>: Copy` is not satisfied
   |
   |
LL |     let arr: [Option<String>; 2] = [Some("foo".to_string()); 2];
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `Option<String>`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <Option<T> as Copy>
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/repeat-to-run-dtor-twice.rs stdout ----
diff of stderr:

3    |
4 LL |     let _ = [ a; 5 ];
5    |             ^^^^^^^^ the trait `Copy` is not implemented for `Foo`
-    |
-    = note: the `Copy` trait is required because the repeated element will be copied
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat-to-run-dtor-twice/repeat-to-run-dtor-twice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args repeat-to-run-dtor-twice.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repeat-to-run-dtor-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat-to-run-dtor-twice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat-to-run-dtor-twice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Foo: Copy` is not satisfied
   |
   |
LL |     let _ = [ a; 5 ];
   |             ^^^^^^^^ the trait `Copy` is not implemented for `Foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 11213 passed; 11 failed; 88 ignored; 0 measured; 0 filtered out; finished in 134.08s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:03
