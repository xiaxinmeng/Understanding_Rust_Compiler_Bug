plain
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/const_refers_to_static_cross_crate.rs:12:1
+   --> $DIR/const_refers_to_static_cross_crate.rs:13:1
3    |
4 LL | const SLICE_MUT: &[u8; 1] = {
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to a static variable in a constant
10            }
11 
12 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:40:9
---
18 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/const_refers_to_static_cross_crate.rs:17:1
+   --> $DIR/const_refers_to_static_cross_crate.rs:18:1
20    |
21 LL | const U8_MUT: &u8 = {
22    | ^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to a static variable in a constant
27            }
28 
29 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:49:9
-   --> $DIR/const_refers_to_static_cross_crate.rs:49:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:50:9
31    |
32 LL |         U8_MUT => true,

34 
35 warning: any use of this value will cause an error
-   --> $DIR/const_refers_to_static_cross_crate.rs:25:15
-   --> $DIR/const_refers_to_static_cross_crate.rs:25:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:26:15
37    |
38 LL | const U8_MUT2: &u8 = {

43    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
44    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
45 note: the lint level is defined here
---
51 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:60:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:61:9
53    |
54 LL |         U8_MUT2 => true,

56 
57 warning: any use of this value will cause an error
-   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
-   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:33:20
59    |
60 LL | const U8_MUT3: &u8 = {

65    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
66    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
67 note: the lint level is defined here
---
73 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:68:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:69:9
75    |
76 LL |         U8_MUT3 => true,

78 
79 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:40:9
---
85 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:49:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:50:9
87    |
88 LL |         U8_MUT => true,

90 
91 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:60:9
-   --> $DIR/const_refers_to_static_cross_crate.rs:60:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:61:9
93    |
94 LL |         U8_MUT2 => true,

96 
97 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:68:9
-   --> $DIR/const_refers_to_static_cross_crate.rs:68:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:69:9
99    |
100 LL |         U8_MUT3 => true,

103 warning: skipping const checks
104    |
105 help: skipping check that does not even have a feature gate
---
115 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:19:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:20:15
117    |
118 LL |     unsafe { &static_cross_crate::ZERO[0] }

120 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:19:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:20:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:20:15
122    |
123 LL |     unsafe { &static_cross_crate::ZERO[0] }

125 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:19:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:20:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:20:15
127    |
128 LL |     unsafe { &static_cross_crate::ZERO[0] }

130 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:25:17
+   --> $DIR/const_refers_to_static_cross_crate.rs:26:17
+   --> $DIR/const_refers_to_static_cross_crate.rs:26:17
132    |
133 LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }

135 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:33:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:33:20
137    |
138 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

140 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
+   --> $DIR/const_refers_to_static_cross_crate.rs:33:20
142    |
143 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

145 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:33:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:33:20
147    |
148 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

150 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:33:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:33:20
152    |
153 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

158 For more information about this error, try `rustc --explain E0080`.
159 Future incompatibility report: Future breakage diagnostic:
160 warning: any use of this value will cause an error
160 warning: any use of this value will cause an error
-   --> $DIR/const_refers_to_static_cross_crate.rs:25:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:26:15
162    |
163 LL | const U8_MUT2: &u8 = {

168    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
169    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
170 note: the lint level is defined here
---
177 warning: any use of this value will cause an error
-   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:33:20
179    |
180 LL | const U8_MUT3: &u8 = {

185    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
186    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
187 note: the lint level is defined here
187 note: the lint level is defined here
-   --> $DIR/const_refers_to_static_cross_crate.rs:30:8
+   --> $DIR/const_refers_to_static_cross_crate.rs:31:8
189    |
190 LL | #[warn(const_err)]
191    |        ^^^^^^^^^


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
   | ^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to a static variable in a constant
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:41:9
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:41:9
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:18:1
   |
LL | const U8_MUT: &u8 = { //~ ERROR undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to a static variable in a constant
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:50:9
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:50:9
   |
LL |         U8_MUT => true,

warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:26:15
   |
   |
LL | const U8_MUT2: &u8 = {
   | ------------------
LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:24:8
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:24:8
   |
LL | #[warn(const_err)]
   |        ^^^^^^^^^

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:61:9
   |
LL |         U8_MUT2 => true,

warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:33:20
   |
   |
LL | const U8_MUT3: &u8 = {
   | ------------------
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:31:8
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:31:8
   |
LL | #[warn(const_err)]
   |        ^^^^^^^^^

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:69:9
   |
LL |         U8_MUT3 => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:41:9
   |
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:50:9
   |
LL |         U8_MUT => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:61:9
   |
   |
LL |         U8_MUT2 => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:69:9
   |
   |
LL |         U8_MUT3 => true,

warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
---
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:20:15
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:20:15
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:20:15
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:26:17
   |
   |
LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:33:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:33:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:33:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:33:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

error: aborting due to 10 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:26:15
   |
LL | const U8_MUT2: &u8 = {
   | ------------------
LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:24:8
---
Future breakage diagnostic:
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:33:20
   |
LL | const U8_MUT3: &u8 = {
   | ------------------
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:31:8
