plain
---- [ui] src/test/ui/hygiene/hir-res-hygiene.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/hir-res-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hir-res-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hir-res-hygiene/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/hygiene/hir-res-hygiene.rs:16:5
   |
LL |     Ok(())?;
LL |     Ok(())?;
   |     ^^ cannot infer type for type parameter `E` declared on the enum `Result`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-32709.rs stdout ----
diff of stderr:

7    |           ^ the trait `From<{integer}>` is not implemented for `()`
8    |
9    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
-    = help: the following other types implement trait `FromResidual<R>`:
-              <Result<T, F> as FromResidual<Result<Infallible, E>>>
-              <Result<T, F> as FromResidual<Yeet<E>>>
+    = help: the following other types implement trait `From<T>`:
+              <() as From<[T; 0]>>
+              <(T, T) as From<[T; 2]>>
+              <(T, T, T) as From<[T; 3]>>
+              <(T, T, T, T) as From<[T; 4]>>
+              <(T, T, T, T, T) as From<[T; 5]>>
+              <(T, T, T, T, T, T) as From<[T; 6]>>
+              <(T, T, T, T, T, T, T) as From<[T; 7]>>
+              <(T, T, T, T, T, T, T, T) as From<[T; 8]>>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
13    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, {integer}>>` for `Result<i32, ()>`
15 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/issue-32709.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-32709.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32709.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `?` couldn't convert the error to `()`
   |
   |
LL | fn a() -> Result<i32, ()> {
   |           --------------- expected `()` because of this
LL |     Err(5)?; //~ ERROR
   |           ^ the trait `From<{integer}>` is not implemented for `()`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `From<T>`:
             <() as From<[T; 0]>>
             <(T, T) as From<[T; 2]>>
             <(T, T, T) as From<[T; 3]>>
             <(T, T, T, T) as From<[T; 4]>>
             <(T, T, T, T, T) as From<[T; 5]>>
             <(T, T, T, T, T, T) as From<[T; 6]>>
             <(T, T, T, T, T, T, T) as From<[T; 7]>>
             <(T, T, T, T, T, T, T, T) as From<[T; 8]>>
           and 5 others
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, {integer}>>` for `Result<i32, ()>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---
- error: aborting due to previous error
+ error[E0282]: type annotations needed
+   --> $DIR/lint-qualification.rs:13:36
+    |
+ LL |     let _ = || -> Result<(), ()> { try!(Ok(())); Ok(()) }; // issue #37345
+    |
+    = note: this error originates in the macro `try` (in Nightly builds, run with -Z macro-backtrace for more info)
14 
+ error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args lint/lint-qualification.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-qualification.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-qualification" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-qualification/auxiliary"
stdout: none
--- stderr -------------------------------
error: unnecessary qualification
   |
   |
LL |     foo::bar(); //~ ERROR: unnecessary qualification
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-qualification.rs:1:9
   |
   |
LL | #![deny(unused_qualifications)]
   |         ^^^^^^^^^^^^^^^^^^^^^

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/lint/lint-qualification.rs:13:36
   |
LL |     let _ = || -> Result<(), ()> { try!(Ok(())); Ok(()) }; // issue #37345
   |
   = note: this error originates in the macro `try` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/macros/macro-comma-support-rpass.rs#core stdout ----

error in revision `core`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/macros/macro-comma-support-rpass.rs:271:9
   |
   |
LL |         try!(Ok(()));
   |
   = note: this error originates in the macro `try` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/macros/macro-comma-support-rpass.rs#std stdout ----

error in revision `std`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "std" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/macros/macro-comma-support-rpass.rs:271:9
   |
   |
LL |         try!(Ok(()));
   |
   = note: this error originates in the macro `try` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---

---- [ui] src/test/ui/suggestions/issue-71394-no-from-impl.rs stdout ----
diff of stderr:

5    |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
6    |
7    = help: the following other types implement trait `From<T>`:
-              <[T; LANES] as From<Simd<T, LANES>>>
-              <[bool; LANES] as From<Mask<T, LANES>>>
+              <[T; 0] as From<()>>
+              <[T; 10] as From<(T, T, T, T, T, T, T, T, T, T)>>
+              <[T; 11] as From<(T, T, T, T, T, T, T, T, T, T, T)>>
+              <[T; 12] as From<(T, T, T, T, T, T, T, T, T, T, T, T)>>
+              <[T; 1] as From<(T,)>>
+              <[T; 2] as From<(T, T)>>
+              <[T; 3] as From<(T, T, T)>>
+              <[T; 4] as From<(T, T, T, T)>>
+            and 7 others
10    = note: required because of the requirements on the impl of `Into<&[i8]>` for `&[u8]`
12 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl/issue-71394-no-from-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-71394-no-from-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-71394-no-from-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&[i8]: From<&[u8]>` is not satisfied
   |
   |
LL |     let _: &[i8] = data.into();
   |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
   |
   = help: the following other types implement trait `From<T>`:
             <[T; 0] as From<()>>
             <[T; 10] as From<(T, T, T, T, T, T, T, T, T, T)>>
             <[T; 11] as From<(T, T, T, T, T, T, T, T, T, T, T)>>
             <[T; 12] as From<(T, T, T, T, T, T, T, T, T, T, T, T)>>
             <[T; 1] as From<(T,)>>
             <[T; 2] as From<(T, T)>>
             <[T; 3] as From<(T, T, T)>>
             <[T; 4] as From<(T, T, T, T)>>
           and 7 others
   = note: required because of the requirements on the impl of `Into<&[i8]>` for `&[u8]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/type-ascription-instead-of-path-2.rs stdout ----

error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/type-ascription-instead-of-path-2.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/type-ascription-instead-of-path-2.fixed:3:18
   |
   |
LL |     let _ = vec![Ok(2)].into_iter().collect::<Result<Vec<_>,_>>()?;
   |             -----^^----------------
   |             |    |
   |             |    cannot infer type for type parameter `E` declared on the enum `Result`
   |             this method call resolves to `<Self as IntoIterator>::IntoIter`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
------------------------------------------
