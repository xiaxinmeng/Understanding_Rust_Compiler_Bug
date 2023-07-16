plain

---- [ui] tests/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs#mir stdout ----
diff of stderr:

4 LL |     sse2();
5    |     ^^^^^^ call to function with `#[target_feature]`
-    = note: can only be called if the required target features are available
-    = note: can only be called if the required target features are available
+    = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
+    = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`
8 
9 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block


12 LL |     avx_bmi2();
13    |     ^^^^^^^^^^ call to function with `#[target_feature]`
-    = note: can only be called if the required target features are available
-    = note: can only be called if the required target features are available
+    = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.
16 
17 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block


20 LL |     Quux.avx_bmi2();
21    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
-    = note: can only be called if the required target features are available
-    = note: can only be called if the required target features are available
+    = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.
24 
25 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block


28 LL |     avx_bmi2();
29    |     ^^^^^^^^^^ call to function with `#[target_feature]`
-    = note: can only be called if the required target features are available
-    = note: can only be called if the required target features are available
+    = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.
32 
33 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block


36 LL |     Quux.avx_bmi2();
37    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
-    = note: can only be called if the required target features are available
-    = note: can only be called if the required target features are available
+    = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.
40 
41 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block

44 LL |     sse2();
44 LL |     sse2();
45    |     ^^^^^^ call to function with `#[target_feature]`
-    = note: can only be called if the required target features are available
-    = note: can only be called if the required target features are available
+    = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
+    = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`
48 
49 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block


52 LL |     avx_bmi2();
53    |     ^^^^^^^^^^ call to function with `#[target_feature]`
-    = note: can only be called if the required target features are available
-    = note: can only be called if the required target features are available
+    = help: in order for the call to be safe, the context requires the following additional target feature: bmi2.
56 
57 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block


60 LL |     Quux.avx_bmi2();
61    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
-    = note: can only be called if the required target features are available
-    = note: can only be called if the required target features are available
+    = help: in order for the call to be safe, the context requires the following additional target feature: bmi2.
64 
65 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block

68 LL |     sse2();
68 LL |     sse2();
69    |     ^^^^^^ call to function with `#[target_feature]`
-    = note: can only be called if the required target features are available
-    = note: can only be called if the required target features are available
+    = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
+    = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`
72 
73 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block


76 LL | const name: () = sse2();
77    |                  ^^^^^^ call to function with `#[target_feature]`
-    = note: can only be called if the required target features are available
-    = note: can only be called if the required target features are available
+    = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
+    = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`
81 error: aborting due to 10 previous errors
82 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.mir/safe-calls.mir.stderr
To only update this specific test, also pass `--test-args rfcs/rfc-2396-target_feature-11/safe-calls.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:23:5
LL |     sse2();
LL |     sse2();
   |     ^^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
   = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:26:5
   |
LL |     avx_bmi2();
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:29:5
   |
LL |     Quux.avx_bmi2();
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:36:5
   |
LL |     avx_bmi2();
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:39:5
   |
LL |     Quux.avx_bmi2();
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:46:5
LL |     sse2();
LL |     sse2();
   |     ^^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
   = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:49:5
   |
LL |     avx_bmi2();
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target feature: bmi2.

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:52:5
   |
LL |     Quux.avx_bmi2();
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target feature: bmi2.

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:60:5
LL |     sse2();
LL |     sse2();
   |     ^^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
   = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`

error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:65:18
   |
LL | const name: () = sse2();
   |                  ^^^^^^ call to function with `#[target_feature]`
   |
   = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
   = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`
error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
