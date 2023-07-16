plain

---- [ui] src/test/ui/issues/issue-32709.rs stdout ----
diff of stderr:

7    |           ^ the trait `From<{integer}>` is not implemented for `()`
8    |
9    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
-    = help: the following other types implement trait `FromResidual<R>`:
-              <Result<T, F> as FromResidual<Result<Infallible, E>>>
-              <Result<T, F> as FromResidual<Yeet<E>>>
+    = help: the following other types implement trait `From<T>`:
+              <(&'a A, &'a Z, &'a Y, &'a X, &'a W, &'a V, &'a U, &'a T) as From<&'a (A, Z, Y, X, W, V, U, T)>>
+              <(&'a B, &'a A, &'a Z, &'a Y, &'a X, &'a W, &'a V, &'a U, &'a T) as From<&'a (B, A, Z, Y, X, W, V, U, T)>>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+              <(&'a C, &'a B, &'a A, &'a Z, &'a Y, &'a X, &'a W, &'a V, &'a U, &'a T) as From<&'a (C, B, A, Z, Y, X, W, V, U, T)>>
+              <(&'a D, &'a C, &'a B, &'a A, &'a Z, &'a Y, &'a X, &'a W, &'a V, &'a U, &'a T) as From<&'a (D, C, B, A, Z, Y, X, W, V, U, T)>>
+              <(&'a E, &'a D, &'a C, &'a B, &'a A, &'a Z, &'a Y, &'a X, &'a W, &'a V, &'a U, &'a T) as From<&'a (E, D, C, B, A, Z, Y, X, W, V, U, T)>>
+              <(&'a T,) as From<&'a (T,)>>
+              <(&'a U, &'a T) as From<&'a (U, T)>>
+              <(&'a V, &'a U, &'a T) as From<&'a (V, U, T)>>
+            and 16 others
13    = note: required for `Result<i32, ()>` to implement `FromResidual<Result<Infallible, {integer}>>`
15 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/issue-32709.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-32709.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32709.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32709/auxiliary"
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
             <(&'a A, &'a Z, &'a Y, &'a X, &'a W, &'a V, &'a U, &'a T) as From<&'a (A, Z, Y, X, W, V, U, T)>>
             <(&'a B, &'a A, &'a Z, &'a Y, &'a X, &'a W, &'a V, &'a U, &'a T) as From<&'a (B, A, Z, Y, X, W, V, U, T)>>
             <(&'a C, &'a B, &'a A, &'a Z, &'a Y, &'a X, &'a W, &'a V, &'a U, &'a T) as From<&'a (C, B, A, Z, Y, X, W, V, U, T)>>
             <(&'a D, &'a C, &'a B, &'a A, &'a Z, &'a Y, &'a X, &'a W, &'a V, &'a U, &'a T) as From<&'a (D, C, B, A, Z, Y, X, W, V, U, T)>>
             <(&'a E, &'a D, &'a C, &'a B, &'a A, &'a Z, &'a Y, &'a X, &'a W, &'a V, &'a U, &'a T) as From<&'a (E, D, C, B, A, Z, Y, X, W, V, U, T)>>
             <(&'a T,) as From<&'a (T,)>>
             <(&'a U, &'a T) as From<&'a (U, T)>>
             <(&'a V, &'a U, &'a T) as From<&'a (V, U, T)>>
           and 16 others
   = note: required for `Result<i32, ()>` to implement `FromResidual<Result<Infallible, {integer}>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
