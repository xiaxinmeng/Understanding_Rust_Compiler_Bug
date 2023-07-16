plain
.................................................................................................... 9500/11869
.................................................................................................... 9600/11869
........................................................................i......i.................... 9700/11869
.................................................................................................... 9800/11869
.................iiiiiii..iiiiii.i.................................................................. 9900/11869
.................................................................................................... 10100/11869
.................................................................................................... 10200/11869
.................................................................................................... 10300/11869
.................................................................................................... 10400/11869
---
 finished in 0.459 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.12s

 finished in 0.195 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.74s

 finished in 3.814 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 25 tests
iiiiiiiiiiiii............

 finished in 2.922 seconds
Build completed successfully in 0:31:11
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 11869 tests
......................i............................................................................. 100/11869
.....................................iiiii..iii.iiiiiiiii........................................... 200/11869
.................................................................................................... 400/11869
.................................................................................................... 500/11869
...............................................................................................i.... 600/11869
.................................................................................................... 700/11869
---
.................................................................................................... 2200/11869
.................i.................................................................................. 2300/11869
...ii............................................................................................... 2400/11869
........................................................................................i........... 2500/11869
........................................i.i.........................i............F.................. 2600/11869
....i......F.......................F................................................................ 2700/11869
.................................................................................................... 2900/11869
........................iiiii....................................................................... 3000/11869
.................................................................................................... 3100/11869
.................................................................................................... 3200/11869
---
.................................................................................................... 9400/11869
.................................................................................................... 9500/11869
.................................................................................................... 9600/11869
........................................................................i......i.................... 9700/11869
.................................iiiiiii............................................................ 9800/11869
.................iiiiiii..iiiiii.i.................................................................. 9900/11869
.................................................................................................... 10100/11869
.................................................................................................... 10200/11869
.................................................................................................... 10300/11869
.................................................................................................... 10400/11869
---

---- [ui] ui/consts/issue-83182.rs stdout ----
diff of stderr:

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer in `str` at .<deref>.0
6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
+    = note: the raw bytes of the constant (size: 8, align: 4) {
10            }
11 
12 error: aborting due to previous error

---
To only update this specific test, also pass `--test-args consts/issue-83182.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-83182.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-83182" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-83182/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/issue-83182.rs:3:1
   |
LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[&()]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer in `str` at .<deref>.0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error: aborting due to previous error

---
---- [ui] ui/consts/miri_unleashed/const_refers_to_static2.rs stdout ----
diff of 32bit.stderr:

3    |
4 LL | / const REF_INTERIOR_MUT: &usize = {
5 LL | |
- LL | |
- LL | |
8 LL | |     static FOO: AtomicUsize = AtomicUsize::new(0);
9 LL | |     unsafe { &*(&FOO as *const _ as *const usize) }
10 LL | | };
16            }
17 
18 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/const_refers_to_static2.rs:20:1
-   --> $DIR/const_refers_to_static2.rs:20:1
+   --> $DIR/const_refers_to_static2.rs:18:1
20    |
21 LL | / const READ_IMMUT: &usize = {
22 LL | |
- LL | |
- LL | |
25 LL | |     static FOO: usize = 0;
26 LL | |     &FOO
---
37 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static2.rs:16:18
+   --> $DIR/const_refers_to_static2.rs:14:18
39    |
40 LL |     unsafe { &*(&FOO as *const _ as *const usize) }

42 help: skipping check for `const_raw_ptr_deref` feature
-   --> $DIR/const_refers_to_static2.rs:16:14
+   --> $DIR/const_refers_to_static2.rs:14:14
+   --> $DIR/const_refers_to_static2.rs:14:14
44    |
45 LL |     unsafe { &*(&FOO as *const _ as *const usize) }

47 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static2.rs:25:6
+   --> $DIR/const_refers_to_static2.rs:21:6
+   --> $DIR/const_refers_to_static2.rs:21:6
49    |
50 LL |     &FOO
51    |      ^^^


The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2/const_refers_to_static2.32bit.stderr
To only update this specific test, also pass `--test-args consts/miri_unleashed/const_refers_to_static2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:11:1
   |
LL | / const REF_INTERIOR_MUT: &usize = { //~ ERROR undefined behavior to use this value
LL | | //~| encountered a reference pointing to a static variable
LL | |     static FOO: AtomicUsize = AtomicUsize::new(0);
LL | |     unsafe { &*(&FOO as *const _ as *const usize) }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:18:1
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:18:1
   |
LL | / const READ_IMMUT: &usize = { //~ ERROR it is undefined behavior to use this value
LL | | //~| encountered a reference pointing to a static variable
LL | |     static FOO: usize = 0;
LL | |     &FOO
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:14:18
   |
LL |     unsafe { &*(&FOO as *const _ as *const usize) }
help: skipping check for `const_raw_ptr_deref` feature
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:14:14
   |
   |
LL |     unsafe { &*(&FOO as *const _ as *const usize) }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:21:6
   |
LL |     &FOO
---
---- [ui] ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs stdout ----
diff of 32bit.stderr:

3    |
4 LL | / const SLICE_MUT: &[u8; 1] = {
5 LL | |
- LL | |
- LL | |
8 LL | |     unsafe { &static_cross_crate::ZERO }
9 LL | | };
10    | |__^ type validation failed: encountered a reference pointing to a static variable
15            }
16 
17 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:47:9
---
23 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/const_refers_to_static_cross_crate.rs:19:1
+   --> $DIR/const_refers_to_static_cross_crate.rs:17:1
25    |
26 LL | / const U8_MUT: &u8 = {
27 LL | |
- LL | |
- LL | |
- LL | |
30 LL | |     unsafe { &static_cross_crate::ZERO[0] }
31 LL | | };
32    | |__^ type validation failed: encountered a reference pointing to a static variable
37            }
38 
39 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:56:9
-   --> $DIR/const_refers_to_static_cross_crate.rs:56:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:49:9
41    |
42 LL |         U8_MUT => true,

44 
44 
45 warning: any use of this value will cause an error
-   --> $DIR/const_refers_to_static_cross_crate.rs:29:15
47    |
47    |
48 LL | / const U8_MUT2: &u8 = {
49 LL | |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
51 LL | |
52 LL | |
53 LL | |
- LL | |
---
58 note: the lint level is defined here
-   --> $DIR/const_refers_to_static_cross_crate.rs:27:8
+   --> $DIR/const_refers_to_static_cross_crate.rs:23:8
60    |
61 LL | #[warn(const_err)]

64    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
65 
66 error: could not evaluate constant pattern
66 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:67:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:60:9
68    |
69 LL |         U8_MUT2 => true,

71 
71 
72 warning: any use of this value will cause an error
-   --> $DIR/const_refers_to_static_cross_crate.rs:37:51
74    |
74    |
75 LL | / const U8_MUT3: &u8 = {
76 LL | |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
77    | |                                                   ^^^^^^^^^^^ constant accesses static
78 LL | |
79 LL | |
- ...  |
---
85 note: the lint level is defined here
-   --> $DIR/const_refers_to_static_cross_crate.rs:35:8
+   --> $DIR/const_refers_to_static_cross_crate.rs:30:8
87    |
88 LL | #[warn(const_err)]

91    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
92 
93 error: could not evaluate constant pattern
93 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:75:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:68:9
95    |
96 LL |         U8_MUT3 => true,

98 
99 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:47:9
---
105 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:56:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:49:9
107    |
108 LL |         U8_MUT => true,

110 
111 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:67:9
-   --> $DIR/const_refers_to_static_cross_crate.rs:67:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:60:9
113    |
114 LL |         U8_MUT2 => true,

116 
117 error: could not evaluate constant pattern
-   --> $DIR/const_refers_to_static_cross_crate.rs:75:9
-   --> $DIR/const_refers_to_static_cross_crate.rs:75:9
+   --> $DIR/const_refers_to_static_cross_crate.rs:68:9
119    |
120 LL |         U8_MUT3 => true,

123 warning: skipping const checks
124    |
125 help: skipping check that does not even have a feature gate
---
135 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:23:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:19:15
137    |
138 LL |     unsafe { &static_cross_crate::ZERO[0] }

140 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:23:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:19:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:19:15
142    |
143 LL |     unsafe { &static_cross_crate::ZERO[0] }

145 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:23:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:19:15
+   --> $DIR/const_refers_to_static_cross_crate.rs:19:15
147    |
148 LL |     unsafe { &static_cross_crate::ZERO[0] }

150 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:29:17
+   --> $DIR/const_refers_to_static_cross_crate.rs:25:17
+   --> $DIR/const_refers_to_static_cross_crate.rs:25:17
152    |
153 LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }

155 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:37:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
157    |
158 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

160 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:37:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
162    |
163 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

165 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:37:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
167    |
168 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }


170 help: skipping check for `const_panic` feature
-   --> $DIR/const_refers_to_static_cross_crate.rs:37:77
172    |
172    |
173 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

175 help: skipping check that does not even have a feature gate
-   --> $DIR/const_refers_to_static_cross_crate.rs:37:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
+   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
177    |
178 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }


The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/const_refers_to_static_cross_crate.32bit.stderr
To only update this specific test, also pass `--test-args consts/miri_unleashed/const_refers_to_static_cross_crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:12:1
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
   |
   |
LL | / const U8_MUT3: &u8 = {
LL | |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   | |                                                   ^^^^^^^^^^^ constant accesses static
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
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check for `const_panic` feature
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   = note: this warning originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors; 3 warnings emitted

---
test result: FAILED. 11733 passed; 3 failed; 133 ignored; 0 measured; 0 filtered out; finished in 70.53s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/ui --pass=check --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:13
