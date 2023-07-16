plain
........................................................................................ 12760/14878
........................................................................................ 12848/14878
........................................................................................ 12936/14878
........................................................................................ 13024/14878
...................................i..........i........iF..F.i......................i... 13112/14878
........................................................................................ 13288/14878
........................................................................................ 13376/14878
........................................................................................ 13464/14878
........................................................................................ 13552/14878
---

---- [ui] tests/ui/target-feature/gate.rs stdout ----
diff of stderr:

1 error[E0658]: the target feature `avx512bw` is currently unstable
-   --> $DIR/gate.rs:31:18
+   --> $DIR/gate.rs:32:18
3    |
4 LL | #[target_feature(enable = "avx512bw")]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/gate/gate.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/gate/gate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args target-feature/gate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/target-feature/gate.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/gate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/gate/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: the target feature `avx512bw` is currently unstable
  --> fake-test-src-base/target-feature/gate.rs:32:18
   |
LL | #[target_feature(enable = "avx512bw")]
   |
   = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information
   = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information
   = help: add `#![feature(avx512_target_feature)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
---
1 error: malformed `target_feature` attribute input
-   --> $DIR/invalid-attribute.rs:31:1
+   --> $DIR/invalid-attribute.rs:32:1
3    |
4 LL | #[target_feature = "+sse2"]
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[target_feature(enable = "name")]`
6 
7 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:16:1
+   --> $DIR/invalid-attribute.rs:17:1
+   --> $DIR/invalid-attribute.rs:17:1
9    |
10 LL | #[target_feature(enable = "sse2")]

14    | ------------------- not a function definition
15 
16 error: attribute should be applied to a function definition
16 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:21:1
+   --> $DIR/invalid-attribute.rs:22:1
18    |
19 LL | #[target_feature(enable = "sse2")]

23    | ------------------------ not a function definition
24 
25 error: attribute should be applied to a function definition
25 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:26:1
+   --> $DIR/invalid-attribute.rs:27:1
27    |
28 LL | #[target_feature(enable = "sse2")]

32    | ---------------- not a function definition
33 
34 error: attribute should be applied to a function definition
34 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:48:1
+   --> $DIR/invalid-attribute.rs:49:1
36    |
37 LL | #[target_feature(enable = "sse2")]

41    | -------------- not a function definition
42 
43 error: attribute should be applied to a function definition
43 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:53:1
+   --> $DIR/invalid-attribute.rs:54:1
45    |
46 LL | #[target_feature(enable = "sse2")]

50    | --------------------- not a function definition
51 
52 error: attribute should be applied to a function definition
52 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:58:1
+   --> $DIR/invalid-attribute.rs:59:1
54    |
55 LL | #[target_feature(enable = "sse2")]

59    | ----------- not a function definition
60 
61 error: attribute should be applied to a function definition
61 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:63:1
+   --> $DIR/invalid-attribute.rs:64:1
63    |
64 LL | #[target_feature(enable = "sse2")]

68    | ----------- not a function definition
69 
70 error: attribute should be applied to a function definition
70 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:68:1
+   --> $DIR/invalid-attribute.rs:69:1
72    |
73 LL |   #[target_feature(enable = "sse2")]

81    | |_- not a function definition
82 
83 error: attribute should be applied to a function definition
83 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:76:1
+   --> $DIR/invalid-attribute.rs:77:1
85    |
86 LL | #[target_feature(enable = "sse2")]

90    | -------------- not a function definition
91 
92 error: attribute should be applied to a function definition
92 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:81:1
+   --> $DIR/invalid-attribute.rs:82:1
94    |
95 LL | #[target_feature(enable = "sse2")]

99    | ------------ not a function definition
100 
101 error: attribute should be applied to a function definition
101 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:91:1
+   --> $DIR/invalid-attribute.rs:92:1
103    |
104 LL | #[target_feature(enable = "sse2")]

108    | ------------------ not a function definition
109 
110 error: attribute should be applied to a function definition
110 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:96:1
+   --> $DIR/invalid-attribute.rs:97:1
112    |
113 LL | #[target_feature(enable = "sse2")]

117    | ------------------- not a function definition
118 
119 error: attribute should be applied to a function definition
119 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:101:1
+   --> $DIR/invalid-attribute.rs:102:1
121    |
122 LL | #[target_feature(enable = "sse2")]

126    | ----------- not a function definition
127 
128 error: attribute should be applied to a function definition
128 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:119:5
+   --> $DIR/invalid-attribute.rs:120:5
130    |
131 LL |       #[target_feature(enable = "sse2")]

138    | |_____- not a function definition
139 
140 error: attribute should be applied to a function definition
140 error: attribute should be applied to a function definition
-   --> $DIR/invalid-attribute.rs:127:5
+   --> $DIR/invalid-attribute.rs:128:5
142    |
143 LL |     #[target_feature(enable = "sse2")]

147    |     ----- not a function definition
148 
149 error: the feature named `foo` is not valid for this target
149 error: the feature named `foo` is not valid for this target
-   --> $DIR/invalid-attribute.rs:33:18
+   --> $DIR/invalid-attribute.rs:34:18
151    |
152 LL | #[target_feature(enable = "foo")]
153    |                  ^^^^^^^^^^^^^^ `foo` is not valid for this target
154 
155 error: malformed `target_feature` attribute input
-   --> $DIR/invalid-attribute.rs:36:18
+   --> $DIR/invalid-attribute.rs:37:18
+   --> $DIR/invalid-attribute.rs:37:18
157    |
158 LL | #[target_feature(bar)]
159    |                  ^^^ help: must be of the form: `enable = ".."`
160 
161 error: malformed `target_feature` attribute input
-   --> $DIR/invalid-attribute.rs:38:18
+   --> $DIR/invalid-attribute.rs:39:18
+   --> $DIR/invalid-attribute.rs:39:18
163    |
164 LL | #[target_feature(disable = "baz")]
165    |                  ^^^^^^^^^^^^^^^ help: must be of the form: `enable = ".."`
166 
166 
167 error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
-   --> $DIR/invalid-attribute.rs:42:1
169    |
169    |
170 LL | #[target_feature(enable = "sse2")]


177    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
178 
179 error: cannot use `#[inline(always)]` with `#[target_feature]`
-   --> $DIR/invalid-attribute.rs:86:1
181    |
181    |
182 LL | #[inline(always)]

184 
184 
185 error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
-   --> $DIR/invalid-attribute.rs:111:5
187    |
187    |
188 LL |     #[target_feature(enable = "sse2")]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/invalid-attribute/invalid-attribute.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/invalid-attribute/invalid-attribute.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args target-feature/invalid-attribute.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/target-feature/invalid-attribute.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/invalid-attribute" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/invalid-attribute/auxiliary"
stdout: none
error: malformed `target_feature` attribute input
  --> fake-test-src-base/target-feature/invalid-attribute.rs:32:1
   |
   |
LL | #[target_feature = "+sse2"]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[target_feature(enable = "name")]`
error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:17:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | extern crate alloc;

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:22:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | use alloc::alloc::alloc;

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:27:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | extern "Rust" {}

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:49:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | mod another {}

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:54:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | const FOO: usize = 7;

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:59:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | struct Foo;

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:64:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | enum Bar {}

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:69:1
   |
   |
LL |   #[target_feature(enable = "sse2")]
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL |   //~^ ERROR attribute should be applied to a function
LL | / union Qux {
LL | | //~^ NOTE not a function
LL | |     f1: u16,
LL | |     f2: u16,
LL | | }
   | |_- not a function definition
error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:77:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | type Uwu = ();

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:82:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | trait Baz {}

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:92:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | static A: () = ();

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:97:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | impl Quux for u8 {}

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:102:1
   |
   |
LL | #[target_feature(enable = "sse2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | impl Foo {}

error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:120:5
   |
   |
LL |       #[target_feature(enable = "sse2")]
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL |       //~^ ERROR attribute should be applied to a function
LL | /     unsafe {
LL | |         foo();
LL | |         bar();
LL | |     }
   | |_____- not a function definition
error: attribute should be applied to a function definition
  --> fake-test-src-base/target-feature/invalid-attribute.rs:128:5
   |
   |
LL |     #[target_feature(enable = "sse2")]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL |     //~^ ERROR attribute should be applied to a function
LL |     || {};

error: the feature named `foo` is not valid for this target
  --> fake-test-src-base/target-feature/invalid-attribute.rs:34:18
   |
   |
LL | #[target_feature(enable = "foo")]
   |                  ^^^^^^^^^^^^^^ `foo` is not valid for this target
error: malformed `target_feature` attribute input
  --> fake-test-src-base/target-feature/invalid-attribute.rs:37:18
   |
   |
LL | #[target_feature(bar)]
   |                  ^^^ help: must be of the form: `enable = ".."`
error: malformed `target_feature` attribute input
  --> fake-test-src-base/target-feature/invalid-attribute.rs:39:18
   |
   |
LL | #[target_feature(disable = "baz")]
   |                  ^^^^^^^^^^^^^^^ help: must be of the form: `enable = ".."`

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
  --> fake-test-src-base/target-feature/invalid-attribute.rs:43:1
   |
LL | #[target_feature(enable = "sse2")]
...
LL | fn bar() {}
LL | fn bar() {}
   | -------- not an `unsafe` function
   = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
   = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
   = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error: cannot use `#[inline(always)]` with `#[target_feature]`
  --> fake-test-src-base/target-feature/invalid-attribute.rs:87:1
   |
LL | #[inline(always)]


error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
  --> fake-test-src-base/target-feature/invalid-attribute.rs:112:5
   |
LL |     #[target_feature(enable = "sse2")]
...
LL |     fn foo() {}
LL |     fn foo() {}
   |     -------- not an `unsafe` function
   = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
   = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
   = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
error: aborting due to 22 previous errors

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
