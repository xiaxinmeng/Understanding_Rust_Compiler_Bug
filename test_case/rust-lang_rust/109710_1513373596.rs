plain

---- [ui] tests/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs#thir stdout ----
diff of stderr:

4 LL |     sse2();
5    |     ^^^^^^ call to function with `#[target_feature]`
6    |
-    = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
-    = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`
+    = note: can only be called if the required target features are available
9 
10 error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block


13 LL |     avx_bmi2();
14    |     ^^^^^^^^^^ call to function with `#[target_feature]`
15    |
-    = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.
+    = note: can only be called if the required target features are available
17 
18 error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block


21 LL |     Quux.avx_bmi2();
22    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
23    |
-    = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.
+    = note: can only be called if the required target features are available
25 
26 error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block


29 LL |     avx_bmi2();
30    |     ^^^^^^^^^^ call to function with `#[target_feature]`
31    |
-    = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.
+    = note: can only be called if the required target features are available
33 
34 error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block


37 LL |     Quux.avx_bmi2();
38    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
39    |
-    = help: in order for the call to be safe, the context requires the following additional target features: avx and bmi2.
+    = note: can only be called if the required target features are available
41 
42 error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block

45 LL |     sse2();
45 LL |     sse2();
46    |     ^^^^^^ call to function with `#[target_feature]`
47    |
-    = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
-    = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`
+    = note: can only be called if the required target features are available
50 
51 error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block


54 LL |     avx_bmi2();
55    |     ^^^^^^^^^^ call to function with `#[target_feature]`
56    |
-    = help: in order for the call to be safe, the context requires the following additional target feature: bmi2.
+    = note: can only be called if the required target features are available
58 
59 error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block


62 LL |     Quux.avx_bmi2();
63    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
64    |
-    = help: in order for the call to be safe, the context requires the following additional target feature: bmi2.
+    = note: can only be called if the required target features are available
66 
67 error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block

70 LL |     sse2();
70 LL |     sse2();
71    |     ^^^^^^ call to function with `#[target_feature]`
72    |
-    = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
-    = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`
+    = note: can only be called if the required target features are available
75 
76 error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block


79 LL | const name: () = sse2();
80    |                  ^^^^^^ call to function with `#[target_feature]`
81    |
-    = help: in order for the call to be safe, the context requires the following additional target feature: sse2.
-    = note: the sse2 target feature being enabled in the build configuration does not remove the requirement to list it in `#[target_feature]`
+    = note: can only be called if the required target features are available
85 error: aborting due to 10 previous errors
86 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.thir/safe-calls.thir.stderr
To only update this specific test, also pass `--test-args rfcs/rfc-2396-target_feature-11/safe-calls.rs`


error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.thir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.thir/auxiliary" "-Z" "thir-unsafeck"
stdout: none
--- stderr -------------------------------
error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:23:5
LL |     sse2();
LL |     sse2();
   |     ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:26:5
   |
LL |     avx_bmi2();
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:29:5
   |
LL |     Quux.avx_bmi2();
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:36:5
   |
LL |     avx_bmi2();
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:39:5
   |
LL |     Quux.avx_bmi2();
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:46:5
LL |     sse2();
LL |     sse2();
   |     ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:49:5
   |
LL |     avx_bmi2();
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:52:5
   |
LL |     Quux.avx_bmi2();
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:60:5
LL |     sse2();
LL |     sse2();
   |     ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/safe-calls.rs:65:18
   |
LL | const name: () = sse2();
   |                  ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error: aborting due to 10 previous errors

