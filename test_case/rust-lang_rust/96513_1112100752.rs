plain
Successfully built 1460b1f46d50
Successfully tagged rust-ci:latest
Built container sha256:1460b1f46d5061665e092ad44ebf8b2b206876d7ba22db053341374550097af1
Uploading finished image to https://ci-caches.rust-lang.org/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775
upload failed: - to s3://rust-lang-ci-sccache2/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---

---- [ui] src/test/ui/binop/issue-77910-1.rs stdout ----
diff of stderr:

25              extern "C" fn(A, B, C) -> Ret
26              extern "C" fn(A, B, C, ...) -> Ret
27              extern "C" fn(A, B, C, D) -> Ret
+            and 94 others
+            and 94 others
29    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
31 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/issue-77910-1.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binop/issue-77910-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-77910-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {foo}`
   |
   |
LL |     assert_eq!(foo, y);
   |     |
   |     |
   |     for<'r> fn(&'r i32) -> &'r i32 {foo}
   |
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `for<'r> fn(&'r i32) -> &'r i32 {foo}` doesn't implement `Debug`
   |
   |
LL |     assert_eq!(foo, y);
   |     ^^^^^^^^^^^^^^^^^^ `for<'r> fn(&'r i32) -> &'r i32 {foo}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `for<'r> fn(&'r i32) -> &'r i32 {foo}`
   = help: the following other types implement trait `Debug`:
             extern "C" fn() -> Ret
             extern "C" fn(A) -> Ret
             extern "C" fn(A, ...) -> Ret
             extern "C" fn(A, B) -> Ret
             extern "C" fn(A, B, ...) -> Ret
             extern "C" fn(A, B, C) -> Ret
             extern "C" fn(A, B, C, ...) -> Ret
             extern "C" fn(A, B, C, D) -> Ret
           and 94 others
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0369.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-59488.rs stdout ----
diff of stderr:

103              extern "C" fn(A, B, C) -> Ret
104              extern "C" fn(A, B, C, ...) -> Ret
105              extern "C" fn(A, B, C, D) -> Ret
+            and 94 others
+            and 94 others
107    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
109 error: aborting due to 9 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
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
             extern "C" fn(A) -> Ret
             extern "C" fn(A, ...) -> Ret
             extern "C" fn(A, B) -> Ret
             extern "C" fn(A, B, ...) -> Ret
             extern "C" fn(A, B, C) -> Ret
             extern "C" fn(A, B, C, ...) -> Ret
             extern "C" fn(A, B, C, D) -> Ret
           and 94 others
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0277, E0308, E0369.
For more information about an error, try `rustc --explain E0277`.
