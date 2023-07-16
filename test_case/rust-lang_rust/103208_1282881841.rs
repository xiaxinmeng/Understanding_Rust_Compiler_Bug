plain
---- [ui] src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs stdout ----
diff of 32bit.stderr:

132    |
133 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
134    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ help: skipping check that does not even have a feature gate
+    |
+    |
+ LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
135 
136 error: aborting due to 12 previous errors; 1 warning emitted
137 



The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/const_refers_to_static_cross_crate.32bit.stderr
To only update this specific test, also pass `--test-args consts/miri_unleashed/const_refers_to_static_cross_crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const SLICE_MUT: &[u8; 1] = { //~ ERROR undefined behavior to use this value
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
   | ^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to a static variable in a constant
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:34:9
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:34:9
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:15:1
   |
LL | const U8_MUT: &u8 = { //~ ERROR undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to a static variable in a constant
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:43:9
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:43:9
   |
LL |         U8_MUT => true,

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:22:15
   |
   |
LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:54:9
   |
   |
LL |         U8_MUT2 => true,

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:27:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:62:9
   |
   |
LL |         U8_MUT3 => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:34:9
   |
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:43:9
   |
LL |         U8_MUT => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:54:9
   |
   |
LL |         U8_MUT2 => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:62:9
   |
   |
LL |         U8_MUT3 => true,

warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
---
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:17:15
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:17:15
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:17:15
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:22:17
   |
   |
LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:27:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:27:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:27:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:27:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:27:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

error: aborting due to 12 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
