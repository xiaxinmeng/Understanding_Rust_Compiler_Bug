plain
+ error[E0390]: cannot define inherent `impl` for primitive types
2   --> $DIR/E0118.rs:1:6
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
3    |
4 LL | impl fn(u8) {
-    |      ^^^^^^ impl requires a nominal type
+    |      ^^^^^^
6    |
6    |
-    = note: either implement a trait on it or create a newtype to wrap it instead
8 
9 error: aborting due to previous error
10 

---
To only update this specific test, also pass `--test-args error-codes/E0118.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0118.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0118" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0118/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0390]: cannot define inherent `impl` for primitive types
   |
   |
LL | impl fn(u8) { //~ ERROR E0118
   |
   = help: consider using an extension trait instead

error: aborting due to previous error
---

---- [ui] src/test/ui/issues/issue-59488.rs stdout ----
diff of stderr:

96    = help: the trait `Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
97    = help: the following other types implement trait `Debug`:
98              extern "C" fn() -> Ret
-              extern "C" fn(A) -> Ret
-              extern "C" fn(A, ...) -> Ret
101              extern "C" fn(A, B) -> Ret
102              extern "C" fn(A, B, ...) -> Ret
103              extern "C" fn(A, B, C) -> Ret

104              extern "C" fn(A, B, C, ...) -> Ret
105              extern "C" fn(A, B, C, D) -> Ret
+              extern "C" fn(A, B, C, D, ...) -> Ret
+              extern "C" fn(A, B, C, D, E) -> Ret
106            and 68 others
107    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/issue-59488.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/issue-59488.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-59488.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59488.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
   |
LL |     foo > 12;
LL |     foo > 12;
   |     --- ^ -- {integer}
   |     fn() -> i32 {foo}
   |
   |
help: you might have forgotten to call this function
LL |     foo() > 12;
   |        ++

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-59488.rs:14:11
   |
LL |     foo > 12;
   |           ^^ expected fn item, found integer
   |
   = note: expected fn item `fn() -> i32 {foo}`


error[E0369]: binary operation `>` cannot be applied to type `fn(i64) -> i64 {bar}`
   |
LL |     bar > 13;
LL |     bar > 13;
   |     --- ^ -- {integer}
   |     |
   |     fn(i64) -> i64 {bar}
   |
help: you might have forgotten to call this function
   |
LL |     bar( /* arguments */ ) > 13;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-59488.rs:18:11
   |
   |
LL |     bar > 13;
   |           ^^ expected fn item, found integer
   |
   = note: expected fn item `fn(i64) -> i64 {bar}`


error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
   |
LL |     foo > foo;
LL |     foo > foo;
   |     --- ^ --- fn() -> i32 {foo}
   |     fn() -> i32 {foo}
   |
   |
help: you might have forgotten to call this function
LL |     foo() > foo;
   |        ++
   |        ++
help: you might have forgotten to call this function
   |
LL |     foo > foo();


error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
   |
   |
LL |     foo > bar;
   |     --- ^ --- fn(i64) -> i64 {bar}
   |     fn() -> i32 {foo}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
  --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
   |
LL |     foo > bar;
   |           ^^^ expected fn item, found a different fn item
   |
   = note: expected fn item `fn() -> i32 {foo}`
              found fn item `fn(i64) -> i64 {bar}`

error[E0369]: binary operation `==` cannot be applied to type `fn(usize) -> Foo {Foo::Bar}`
   |
   |
LL |     assert_eq!(Foo::Bar, i);
   |     |
   |     |
   |     fn(usize) -> Foo {Foo::Bar}
   |     fn(usize) -> Foo {Foo::Bar}
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `Debug`
   |
   |
LL |     assert_eq!(Foo::Bar, i);
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
   = help: the following other types implement trait `Debug`:
             extern "C" fn() -> Ret
             extern "C" fn(A, B) -> Ret
             extern "C" fn(A, B, ...) -> Ret
             extern "C" fn(A, B, C) -> Ret
             extern "C" fn(A, B, C, ...) -> Ret
             extern "C" fn(A, B, C, D) -> Ret
             extern "C" fn(A, B, C, D, ...) -> Ret
             extern "C" fn(A, B, C, D, E) -> Ret
           and 68 others
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0277, E0308, E0369.
For more information about an error, try `rustc --explain E0277`.
