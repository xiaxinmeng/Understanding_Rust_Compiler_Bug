plain

22 error: aborting due to 2 previous errors
23 
24 For more information about this error, try `rustc --explain E0080`.
+ Future incompatibility report: Future breakage diagnostic:
+ error: erroneous constant used
+    |
+    |
+ LL |     let _: &'static _ = &C;
+    |                         ^^ referenced constant has errors
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
25 



The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/invalid-union.32bit.stderr
To only update this specific test, also pass `--test-args consts/invalid-union.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/invalid-union.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | fn main() { //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^ type validation failed at .<deref>.y.<enum-variant(B)>.0: encountered `UnsafeCell` in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: erroneous constant used
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
  --> /checkout/src/test/ui/consts/invalid-union.rs:41:25
   |
LL |     let _: &'static _ = &C; //~ ERROR erroneous constant used
   |                         ^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: erroneous constant used
   |
   |
LL |     let _: &'static _ = &C; //~ ERROR erroneous constant used
   |                         ^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs stdout ----
diff of 32bit.stderr:

170 error: aborting due to 10 previous errors; 3 warnings emitted
171 
172 For more information about this error, try `rustc --explain E0080`.
+ Future incompatibility report: Future breakage diagnostic:
+ warning: any use of this value will cause an error
+    |
+    |
+ LL | / const U8_MUT2: &u8 = {
+ LL | |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
+    | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
+ LL | |
+ LL | |
+ LL | |
+ LL | | };
+    |
+ note: the lint level is defined here
+   --> $DIR/const_refers_to_static_cross_crate.rs:23:8
+    |
+    |
+ LL | #[warn(const_err)]
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
+ Future breakage diagnostic:
+ Future breakage diagnostic:
+ warning: any use of this value will cause an error
+   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
+    |
+ LL | / const U8_MUT3: &u8 = {
+ LL | |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
+    | |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
+ LL | |
+ LL | |
+ LL | |
+ LL | | };
+    |
+ note: the lint level is defined here
+   --> $DIR/const_refers_to_static_cross_crate.rs:30:8
+    |
+    |
+ LL | #[warn(const_err)]
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
173 
173 


The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/const_refers_to_static_cross_crate.32bit.stderr
To only update this specific test, also pass `--test-args consts/miri_unleashed/const_refers_to_static_cross_crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | / const SLICE_MUT: &[u8; 1] = { //~ ERROR undefined behavior to use this value
LL | | //~| encountered a reference pointing to a static variable
LL | |     unsafe { &static_cross_crate::ZERO }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:40:9
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:40:9
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:17:1
   |
LL | / const U8_MUT: &u8 = { //~ ERROR undefined behavior to use this value
LL | | //~| encountered a reference pointing to a static variable
LL | |     unsafe { &static_cross_crate::ZERO[0] }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:49:9
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:49:9
   |
LL |         U8_MUT => true,

warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:25:15
   |
   |
LL | / const U8_MUT2: &u8 = {
LL | |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
   | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
LL | |     //~^ WARN [const_err]
LL | |     //~| constant accesses static
LL | |     //~| WARN this was previously accepted by the compiler but is being phased out
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:23:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:60:9
   |
LL |         U8_MUT2 => true,

warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL | / const U8_MUT3: &u8 = {
LL | |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   | |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
LL | |     //~^ WARN [const_err]
LL | |     //~| constant accesses static
LL | |     //~| WARN this was previously accepted by the compiler but is being phased out
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:30:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:68:9
   |
LL |         U8_MUT3 => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:40:9
   |
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:49:9
   |
LL |         U8_MUT => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:60:9
   |
   |
LL |         U8_MUT2 => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:68:9
   |
   |
LL |         U8_MUT3 => true,

warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
---
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:19:15
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:19:15
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:19:15
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:25:17
   |
   |
LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

error: aborting due to 10 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:25:15
   |
   |
LL | / const U8_MUT2: &u8 = {
LL | |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
   | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
LL | |     //~^ WARN [const_err]
LL | |     //~| constant accesses static
LL | |     //~| WARN this was previously accepted by the compiler but is being phased out
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:23:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
Future breakage diagnostic:
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
LL | / const U8_MUT3: &u8 = {
LL | |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   | |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
LL | |     //~^ WARN [const_err]
LL | |     //~| constant accesses static
LL | |     //~| WARN this was previously accepted by the compiler but is being phased out
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:30:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------

