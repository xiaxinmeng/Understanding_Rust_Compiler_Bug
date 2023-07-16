plain

48 note: required by a bound in `const_eval_select`
49   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
50    |
- LL |     G: FnOnce<ARG, Output = RET> + ~const Drop + ~const Destruct,
+ LL |     G: FnOnce<ARG, Output = RET> + ~const Destruct,
52    |        ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
53 
54 error[E0271]: type mismatch resolving `<fn(i32) -> bool {bar} as FnOnce<(i32,)>>::Output == i32`
60 note: required by a bound in `const_eval_select`
61   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
62    |
62    |
- LL |     G: FnOnce<ARG, Output = RET> + ~const Drop + ~const Destruct,
+ LL |     G: FnOnce<ARG, Output = RET> + ~const Destruct,
64    |                    ^^^^^^^^^^^^ required by this bound in `const_eval_select`
66 error[E0631]: type mismatch in function arguments


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad/const-eval-select-bad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intrinsics/const-eval-select-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:6:27: 6:32]: ~const FnOnce<()>` is not satisfied
   |
   |
LL |     const_eval_select((), || {}, || {});
   |     -----------------     ^^^^^ expected an `FnOnce<()>` closure, found `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:6:27: 6:32]`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `~const FnOnce<()>` is not implemented for `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:6:27: 6:32]`
note: the trait `FnOnce<()>` is implemented for `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:6:27: 6:32]`, but that implementation is not `const`
   |
   |
LL |     const_eval_select((), || {}, || {});
   |                           ^^^^^
   = note: wrap the `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:6:27: 6:32]` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `const_eval_select`
   |
   |
LL |     F: ~const FnOnce<ARG, Output = RET>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`

error[E0277]: the trait bound `{integer}: ~const FnOnce<()>` is not satisfied
   |
   |
LL |     const_eval_select((), 42, 0xDEADBEEF);
   |     -----------------     ^^ expected an `FnOnce<()>` closure, found `{integer}`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `~const FnOnce<()>` is not implemented for `{integer}`
   = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `const_eval_select`
   |
   |
LL |     F: ~const FnOnce<ARG, Output = RET>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`

error[E0277]: expected a `FnOnce<()>` closure, found `{integer}`
   |
   |
LL |     const_eval_select((), 42, 0xDEADBEEF);
   |     -----------------         ^^^^^^^^^^ expected an `FnOnce<()>` closure, found `{integer}`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<()>` is not implemented for `{integer}`
   = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `const_eval_select`
   |
   |
LL |     G: FnOnce<ARG, Output = RET> + ~const Destruct,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`

error[E0271]: type mismatch resolving `<fn(i32) -> bool {bar} as FnOnce<(i32,)>>::Output == i32`
   |
   |
LL |     const_eval_select((1,), foo, bar);
   |     ^^^^^^^^^^^^^^^^^ expected `i32`, found `bool`
note: required by a bound in `const_eval_select`
  --> /checkout/library/core/src/intrinsics.rs:2358:20
   |
   |
LL |     G: FnOnce<ARG, Output = RET> + ~const Destruct,
   |                    ^^^^^^^^^^^^ required by this bound in `const_eval_select`
error[E0631]: type mismatch in function arguments
  --> /checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:33:32
   |
   |
LL | const fn foo(n: i32) -> i32 {
   | --------------------------- found signature of `fn(i32) -> _`
...
LL |     const_eval_select((true,), foo, baz);
   |     -----------------          ^^^ expected signature of `fn(bool) -> _`
   |     required by a bound introduced by this call
   |
note: required by a bound in `const_eval_select`
  --> /checkout/library/core/src/intrinsics.rs:2357:8
  --> /checkout/library/core/src/intrinsics.rs:2357:8
   |
LL |     F: ~const FnOnce<ARG, Output = RET>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0271, E0277, E0631.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.
------------------------------------------


---- [ui] ui/target-feature/tied-features.rs stdout ----
diff of stderr:

1 error: the target features paca, pacg must all be either enabled or disabled together
-   --> $DIR/tied-features.rs:13:5
+   --> $DIR/tied-features.rs:12:5
3    |
4 LL |     #[target_feature(enable = "pacg")]


7    = help: add the missing features in a `target_feature` attribute
8 
9 error: the target features paca, pacg must all be either enabled or disabled together
-   --> $DIR/tied-features.rs:25:1
+   --> $DIR/tied-features.rs:24:1
11    |
12 LL | #[target_feature(enable = "paca")]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features/tied-features.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features/tied-features.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args target-feature/tied-features.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature/tied-features.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "--target=aarch64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/tied-features/auxiliary"
stdout: none
--- stderr -------------------------------
error: the target features paca, pacg must all be either enabled or disabled together
   |
   |
LL |     #[target_feature(enable = "pacg")]
   |
   |
   = help: add the missing features in a `target_feature` attribute

error: the target features paca, pacg must all be either enabled or disabled together
   |
   |
LL | #[target_feature(enable = "paca")]
   |
   |
   = help: add the missing features in a `target_feature` attribute
error: aborting due to 2 previous errors
------------------------------------------


