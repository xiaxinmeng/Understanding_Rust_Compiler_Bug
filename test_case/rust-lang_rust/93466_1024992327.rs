plain
.................................................................................................... 6400/12566
...................i..................................................i.iii........i...i............ 6500/12566
.................................................................................................... 6600/12566
........................................................i....i...................................... 6700/12566
..i.............F.Fi....F...F....i................................F...................i............. 6800/12566
.................................................................................................... 7000/12566
.................................................................................................... 7100/12566
...........ii................................ii..................................................... 7200/12566
....i............................................................................................... 7300/12566
---
- error: aborting due to previous error
+ error: associated constant is never used: `BAR`
+   --> $DIR/associated-const-dead-code.rs:6:5
+    |
+ LL |     const BAR: u32 = 1;
+ 
+ error: aborting due to 2 previous errors
14 
15 
---
To only update this specific test, also pass `--test-args associated-consts/associated-const-dead-code.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/associated-const-dead-code.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/associated-const-dead-code" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/associated-const-dead-code/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/consts/underscore_const_names.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/underscore_const_names.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/underscore_const_names" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/underscore_const_names/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: struct is never constructed: `ImplementsTrait`
  --> /checkout/src/test/ui/consts/underscore_const_names.rs:13:13
   |
LL |             struct ImplementsTrait<T: $trait>(PhantomData<T>);
...
...
LL | check_impl!(Str, Trt);
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/underscore_const_names.rs:3:9
   |
   |
LL | #![deny(unused)]
   |         ^^^^^^
   = note: `#[deny(dead_code)]` implied by `#[deny(unused)]`
   = note: this error originates in the macro `check_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: struct is never constructed: `ImplementsTrait`
  --> /checkout/src/test/ui/consts/underscore_const_names.rs:13:13
   |
   |
LL |             struct ImplementsTrait<T: $trait>(PhantomData<T>);
...
...
LL | check_impl!(Str, Trt);
   |
   = note: this error originates in the macro `check_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: struct is never constructed: `ImplementsTrait`
error: struct is never constructed: `ImplementsTrait`
  --> /checkout/src/test/ui/consts/underscore_const_names.rs:13:13
   |
LL |             struct ImplementsTrait<T: $trait>(PhantomData<T>);
...
...
LL |   check_impl!(Str, Trt);
   |
   = note: this error originates in the macro `check_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: struct is never constructed: `ImplementsTrait`
error: struct is never constructed: `ImplementsTrait`
  --> /checkout/src/test/ui/consts/underscore_const_names.rs:13:13
   |
LL |             struct ImplementsTrait<T: $trait>(PhantomData<T>);
...
...
LL |   check_impl!(Str, Trt);
   |
   = note: this error originates in the macro `check_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 4 previous errors
---

---- [ui] ui/lint/dead-code/issue-85255.rs stdout ----
diff of stderr:

76 LL |     pub fn b(&self) -> i32 { 6 }
78 
- warning: 12 warnings emitted
+ warning: associated function is never used: `a`
+   --> $DIR/issue-85255.rs:15:8
+   --> $DIR/issue-85255.rs:15:8
+    |
+ LL |     fn a(&self) -> i32 { 5 }
+ 
+ warning: associated function is never used: `b`
+   --> $DIR/issue-85255.rs:16:12
+    |
+    |
+ LL |     pub fn b(&self) -> i32 { 6 }
+ 
+ warning: associated function is never used: `a`
+   --> $DIR/issue-85255.rs:27:8
+    |
+    |
+ LL |     fn a(&self) -> i32 { 5 }
+ 
+ warning: associated function is never used: `b`
+   --> $DIR/issue-85255.rs:28:12
+    |
+    |
+ LL |     pub fn b(&self) -> i32 { 6 }
+ 
+ warning: associated function is never used: `a`
+   --> $DIR/issue-85255.rs:39:8
+    |
+    |
+ LL |     fn a(&self) -> i32 { 5 }
+ 
+ warning: associated function is never used: `b`
+   --> $DIR/issue-85255.rs:40:12
+    |
+    |
+ LL |     pub fn b(&self) -> i32 { 6 }
+ 
+ warning: 18 warnings emitted
80 
81 
---
To only update this specific test, also pass `--test-args lint/dead-code/issue-85255.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/dead-code/issue-85255.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/issue-85255" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/issue-85255/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: field is never read: `a`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:8:5
   |
LL |     a: i32, //~ WARNING: field is never read
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:4:9
   |
   |
LL | #![warn(dead_code)]
   |         ^^^^^^^^^

warning: field is never read: `b`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:9:5
   |
LL |     pub b: i32, //~ WARNING: field is never read

warning: associated function is never used: `a`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:15:8
   |
   |
LL |     fn a(&self) -> i32 { 5 } //~ WARNING: associated function is never used

warning: associated function is never used: `b`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:16:12
   |
   |
LL |     pub fn b(&self) -> i32 { 6 } //~ WARNING: associated function is never used

warning: field is never read: `a`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:20:5
   |
   |
LL |     a: i32, //~ WARNING: field is never read

warning: field is never read: `b`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:21:5
   |
   |
LL |     pub b: i32, //~ WARNING: field is never read

warning: associated function is never used: `a`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:27:8
   |
   |
LL |     fn a(&self) -> i32 { 5 } //~ WARNING: associated function is never used

warning: associated function is never used: `b`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:28:12
   |
   |
LL |     pub fn b(&self) -> i32 { 6 } //~ WARNING: associated function is never used

warning: field is never read: `a`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:32:5
   |
   |
LL |     a: i32, //~ WARNING: field is never read

warning: field is never read: `b`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:33:5
   |
   |
LL |     pub b: i32, //~ WARNING: field is never read

warning: associated function is never used: `a`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:39:8
   |
   |
LL |     fn a(&self) -> i32 { 5 } //~ WARNING: associated function is never used

warning: associated function is never used: `b`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:40:12
   |
   |
LL |     pub fn b(&self) -> i32 { 6 } //~ WARNING: associated function is never used

warning: associated function is never used: `a`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:15:8
   |
   |
LL |     fn a(&self) -> i32 { 5 } //~ WARNING: associated function is never used

warning: associated function is never used: `b`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:16:12
   |
   |
LL |     pub fn b(&self) -> i32 { 6 } //~ WARNING: associated function is never used

warning: associated function is never used: `a`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:27:8
   |
   |
LL |     fn a(&self) -> i32 { 5 } //~ WARNING: associated function is never used

warning: associated function is never used: `b`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:28:12
   |
   |
LL |     pub fn b(&self) -> i32 { 6 } //~ WARNING: associated function is never used

warning: associated function is never used: `a`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:39:8
   |
   |
LL |     fn a(&self) -> i32 { 5 } //~ WARNING: associated function is never used

warning: associated function is never used: `b`
  --> /checkout/src/test/ui/lint/dead-code/issue-85255.rs:40:12
   |
   |
LL |     pub fn b(&self) -> i32 { 6 } //~ WARNING: associated function is never used

warning: 18 warnings emitted



------------------------------------------


---- [ui] ui/lint/dead-code/lint-dead-code-1.rs stdout ----
diff of stderr:

64 LL | fn baz() -> impl Copy {
66 
- error: aborting due to 10 previous errors
+ error: struct is never constructed: `Bar`
+   --> $DIR/lint-dead-code-1.rs:12:16
---
69 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/lint-dead-code-1/lint-dead-code-1.stderr
To only update this specific test, also pass `--test-args lint/dead-code/lint-dead-code-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/dead-code/lint-dead-code-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/lint-dead-code-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/lint-dead-code-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: struct is never constructed: `Bar`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-1.rs:12:16
   |
LL |     pub struct Bar; //~ ERROR: struct is never constructed
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-1.rs:5:9
   |
   |
LL | #![deny(dead_code)]
   |         ^^^^^^^^^

error: static is never used: `priv_static`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-1.rs:20:1
   |
LL | static priv_static: isize = 0; //~ ERROR: static is never used

error: constant is never used: `priv_const`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-1.rs:27:1
   |
   |
LL | const priv_const: isize = 0; //~ ERROR: constant is never used


error: struct is never constructed: `PrivStruct`
   |
   |
LL | struct PrivStruct; //~ ERROR: struct is never constructed


error: enum is never used: `priv_enum`
   |
   |
LL | enum priv_enum { foo2, bar2 } //~ ERROR: enum is never used

error: variant is never constructed: `bar3`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-1.rs:67:5
   |
   |
LL |     bar3 //~ ERROR variant is never constructed


error: function is never used: `priv_fn`
   |
   |
LL | fn priv_fn() { //~ ERROR: function is never used

error: function is never used: `foo`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-1.rs:93:4
   |
   |
LL | fn foo() { //~ ERROR: function is never used

error: function is never used: `bar`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-1.rs:98:4
   |
   |
LL | fn bar() { //~ ERROR: function is never used

error: function is never used: `baz`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-1.rs:102:4
   |
   |
LL | fn baz() -> impl Copy { //~ ERROR: function is never used

error: struct is never constructed: `Bar`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-1.rs:12:16
   |
   |
LL |     pub struct Bar; //~ ERROR: struct is never constructed

error: aborting due to 11 previous errors


---
24 
+ error: function is never used: `baz`
+   --> $DIR/lint-dead-code-3.rs:22:8
+    |
+ LL |     fn baz() {}
+ 
25 error: enum is never used: `c_void`
26   --> $DIR/lint-dead-code-3.rs:60:6
27    |
27    |

34 LL |     fn free(p: *const c_void);
36 
- error: aborting due to 5 previous errors
+ error: associated function is never used: `foo`
+   --> $DIR/lint-dead-code-3.rs:16:8
+   --> $DIR/lint-dead-code-3.rs:16:8
+    |
+ LL |     fn foo(&self) {
+    |        ^^^
+ 
+ error: function is never used: `free`
+   --> $DIR/lint-dead-code-3.rs:62:5
+    |
+ LL |     fn free(p: *const c_void);
+ 
+ error: aborting due to 8 previous errors
38 
39 
39 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/lint-dead-code-3/lint-dead-code-3.stderr
To only update this specific test, also pass `--test-args lint/dead-code/lint-dead-code-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/dead-code/lint-dead-code-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/lint-dead-code-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/lint-dead-code-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: struct is never constructed: `Foo`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-3.rs:14:8
   |
LL | struct Foo; //~ ERROR: struct is never constructed
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-3.rs:4:9
   |
   |
LL | #![deny(dead_code)]
   |         ^^^^^^^^^

error: associated function is never used: `foo`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-3.rs:16:8
   |
LL |     fn foo(&self) { //~ ERROR: associated function is never used

error: function is never used: `bar`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-3.rs:21:4
   |
   |
LL | fn bar() { //~ ERROR: function is never used

error: function is never used: `baz`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-3.rs:22:8
   |
   |
LL |     fn baz() {}

error: enum is never used: `c_void`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-3.rs:60:6
   |
   |
LL | enum c_void {} //~ ERROR: enum is never used

error: function is never used: `free`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-3.rs:62:5
   |
   |
LL |     fn free(p: *const c_void); //~ ERROR: function is never used

error: associated function is never used: `foo`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-3.rs:16:8
   |
   |
LL |     fn foo(&self) { //~ ERROR: associated function is never used

error: function is never used: `free`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-3.rs:62:5
   |
   |
LL |     fn free(p: *const c_void); //~ ERROR: function is never used

error: aborting due to 8 previous errors


---
28 LL |     fn unused_impl_fn_3(
29    |        ^^^^^^^^^^^^^^^^
30 
- error: aborting due to 4 previous errors
+ error: associated function is never used: `unused_impl_fn_1`
+    |
+    |
+ LL |     fn unused_impl_fn_1() {
+ 
+ 
+ error: associated function is never used: `unused_impl_fn_2`
+    |
+    |
+ LL |     fn unused_impl_fn_2(var: i32) {
+ 
+ 
+ error: associated function is never used: `unused_impl_fn_3`
+    |
+ LL |     fn unused_impl_fn_3(
+    |        ^^^^^^^^^^^^^^^^
+ 
+ 
+ error: aborting due to 7 previous errors
32 
33 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/lint-dead-code-6/lint-dead-code-6.stderr
To only update this specific test, also pass `--test-args lint/dead-code/lint-dead-code-6.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/dead-code/lint-dead-code-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/lint-dead-code-6" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/lint-dead-code-6/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: struct is never constructed: `UnusedStruct`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-6.rs:3:8
   |
LL | struct UnusedStruct; //~ ERROR struct is never constructed: `UnusedStruct`
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-6.rs:1:9
   |
   |
LL | #![deny(dead_code)]
   |         ^^^^^^^^^

error: associated function is never used: `unused_impl_fn_1`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-6.rs:5:8
   |
LL |     fn unused_impl_fn_1() { //~ ERROR associated function is never used: `unused_impl_fn_1`

error: associated function is never used: `unused_impl_fn_2`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-6.rs:9:8
   |
   |
LL |     fn unused_impl_fn_2(var: i32) { //~ ERROR associated function is never used: `unused_impl_fn_2`

error: associated function is never used: `unused_impl_fn_3`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-6.rs:13:8
   |
   |
LL |     fn unused_impl_fn_3( //~ ERROR associated function is never used: `unused_impl_fn_3`

error: associated function is never used: `unused_impl_fn_1`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-6.rs:5:8
   |
   |
LL |     fn unused_impl_fn_1() { //~ ERROR associated function is never used: `unused_impl_fn_1`

error: associated function is never used: `unused_impl_fn_2`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-6.rs:9:8
   |
   |
LL |     fn unused_impl_fn_2(var: i32) { //~ ERROR associated function is never used: `unused_impl_fn_2`

error: associated function is never used: `unused_impl_fn_3`
  --> /checkout/src/test/ui/lint/dead-code/lint-dead-code-6.rs:13:8
   |
   |
LL |     fn unused_impl_fn_3( //~ ERROR associated function is never used: `unused_impl_fn_3`

error: aborting due to 7 previous errors


---
To only update this specific test, also pass `--test-args lint/force-warn/warn-by-default-lint-two-modules.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/warn-by-default-lint-two-modules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/warn-by-default-lint-two-modules" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warn" "dead_code" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/warn-by-default-lint-two-modules/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/span/macro-span-replacement.rs stdout ----
diff of stderr:

15    = note: `#[warn(dead_code)]` implied by `#[warn(unused)]`
16    = note: this warning originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
- warning: 1 warning emitted
+ warning: struct is never constructed: `S`
+   --> $DIR/macro-span-replacement.rs:7:14
+    |
+    |
+ LL |         $b $a;
+ ...
+ ...
+ LL |     m!(S struct);
+    |
+    |
+    = note: this warning originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
+ warning: 2 warnings emitted
19 
20 

---
To only update this specific test, also pass `--test-args span/macro-span-replacement.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/macro-span-replacement.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/macro-span-replacement" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/macro-span-replacement/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: struct is never constructed: `S`
  --> /checkout/src/test/ui/span/macro-span-replacement.rs:7:14
   |
LL |         $b $a; //~ WARN struct is never constructed
...
...
LL |     m!(S struct);
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/span/macro-span-replacement.rs:3:9
   |
   |
LL | #![warn(unused)]
   |         ^^^^^^
   = note: `#[warn(dead_code)]` implied by `#[warn(unused)]`
   = note: this warning originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: struct is never constructed: `S`
  --> /checkout/src/test/ui/span/macro-span-replacement.rs:7:14
   |
   |
LL |         $b $a; //~ WARN struct is never constructed
...
...
LL |     m!(S struct);
   |
   |
   = note: this warning originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 2 warnings emitted


------------------------------------------
---
test result: FAILED. 12437 passed; 8 failed; 121 ignored; 0 measured; 0 filtered out; finished in 125.29s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:19
