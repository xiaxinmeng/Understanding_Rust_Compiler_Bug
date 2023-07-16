plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:e83cf548fe2db927e33b135adbd2e291ec48d14d)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 12936/14577
........................................................................................ 13024/14577
........................................................................................ 13112/14577
........................................................................................ 13200/14577
......................................................F..........F...................... 13288/14577
........................................................................................ 13464/14577
........................................................................................ 13552/14577
.....................................................................................i.. 13640/14577
........................................................................................ 13728/14577
---
...............................iii...................................................... 14520/14577
.........................................................
failures:

---- [ui] tests/ui/traits/new-solver/int-var-alias-eq.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/int-var-alias-eq.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/int-var-alias-eq" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/int-var-alias-eq/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
error[E0308]: mismatched types
  --> fake-test-src-base/traits/new-solver/int-var-alias-eq.rs:10:13
LL |     let x = 1 + 2;
   |             ^^^^^ expected integer, found associated type
   |
   = note:         expected type `{integer}`
   = note:         expected type `{integer}`
           found associated type `<{integer} as Add<{integer}>>::Output`
   = help: consider constraining the associated type `<{integer} as Add<_>>::Output` to `{integer}`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
   = help: consider constraining the associated type `<{integer} as Add<{integer}>>::Output` to `{integer}`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/traits/new-solver/int-var-alias-eq.rs:15:5
LL |     1 + 2
   |     ^^^^^ expected integer, found associated type
   |
   = note:         expected type `{integer}`
   = note:         expected type `{integer}`
           found associated type `<{integer} as Add<{integer}>>::Output`
   = help: consider constraining the associated type `<{integer} as Add<_>>::Output` to `{integer}`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
   = help: consider constraining the associated type `<{integer} as Add<{integer}>>::Output` to `{integer}`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/traits/new-solver/int-var-alias-eq.rs:15:5
LL | fn test2() -> u32 {
   |               --- expected `u32` because of return type
LL |     // expectation from return ty
LL |     1 + 2
LL |     1 + 2
   |     ^^^^^ expected `u32`, found associated type
   |
   = note:         expected type `u32`
           found associated type `<{integer} as Add<{integer}>>::Output`
   = help: consider constraining the associated type `<{integer} as Add<{integer}>>::Output` to `u32`
help: consider dereferencing the type
   |
LL |     *1 + 2
   |     +
---
---- [ui] tests/ui/traits/new-solver/try-example.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/try-example.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/try-example" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/try-example/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
  --> fake-test-src-base/traits/new-solver/try-example.rs:7:18
LL | fn main() -> Result<(), Box<dyn Error>> {
   | --------------------------------------- this function returns a `Result`
LL |     let x: i32 = parse()?;
LL |     let x: i32 = parse()?;
   |                  ^^^^^^^^ this `?` produces `<Result<_, ParseError> as Try>::Residual`, which is incompatible with `Result<_, ParseError>`
   |
   = help: the trait `FromResidual` is not implemented for `Result<_, ParseError>`
   = help: the following other types implement trait `FromResidual<R>`:
             <Result<T, F> as FromResidual<Result<Infallible, E>>>
             <Result<T, F> as FromResidual<Yeet<E>>>

error: the type `ControlFlow<<_ as Try>::Residual, <_ as Try>::Output>` is not well-formed
  --> fake-test-src-base/traits/new-solver/try-example.rs:7:18
LL |     let x: i32 = parse()?;
   |                  ^^^^^^^^

error: aborting due to 2 previous errors
---

---- [ui] tests/ui/typeck/lazy-norm/cast-checks-handling-projections.rs stdout ----
diff of stderr:

- error[E0271]: type mismatch resolving `char == <u8 as Add>::Output`
+ error[E0308]: mismatched types
3    |
3    |
4 LL |     (0u8 + 0u8) as char;
-    |     ^^^^^^^^^^^ types differ
+    |     ^^^^^^^^^^^ expected `u8`, found associated type
+    |
+    = note:         expected type `u8`
+    = note:         expected type `u8`
+            found associated type `<u8 as Add>::Output`
+    = help: consider constraining the associated type `<u8 as Add<_>>::Output` to `u8`
+    = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
+    = help: consider constraining the associated type `<u8 as Add>::Output` to `u8`
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0605]: non-primitive cast: `<u8 as Add>::Output` as `char`
+    |
+    |
+ LL |     (0u8 + 0u8) as char;
+    |     ^^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
- For more information about this error, try `rustc --explain E0271`.
+ error: aborting due to 2 previous errors
+ 
+ Some errors have detailed explanations: E0308, E0605.
---
To only update this specific test, also pass `--test-args typeck/lazy-norm/cast-checks-handling-projections.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/lazy-norm/cast-checks-handling-projections.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/lazy-norm/cast-checks-handling-projections" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/lazy-norm/cast-checks-handling-projections/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/typeck/lazy-norm/cast-checks-handling-projections.rs:5:5
   |
   |
LL |     (0u8 + 0u8) as char;
   |     ^^^^^^^^^^^ expected `u8`, found associated type
   = note:         expected type `u8`
   = note:         expected type `u8`
           found associated type `<u8 as Add>::Output`
   = help: consider constraining the associated type `<u8 as Add<_>>::Output` to `u8`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
   = help: consider constraining the associated type `<u8 as Add>::Output` to `u8`


error[E0605]: non-primitive cast: `<u8 as Add>::Output` as `char`
  --> fake-test-src-base/typeck/lazy-norm/cast-checks-handling-projections.rs:5:5
   |
LL |     (0u8 + 0u8) as char;
   |     ^^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0605.
For more information about an error, try `rustc --explain E0308`.
