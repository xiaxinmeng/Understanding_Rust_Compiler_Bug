plain
.................................F.................................................................. 4000/12097
.................................................................................................... 4100/12097
.................................................................................................... 4200/12097
.................................................................................................... 4300/12097
.....F..........FF.................................................................................. 4400/12097
.......................................................................ii........................... 4600/12097
.....................................................................i.............................. 4700/12097
.................................................................................................... 4800/12097
.................................................................................................... 4900/12097
---
.................................................................................................... 9700/12097
.................................................................................................... 9800/12097
.....................iiii........i.................................................................. 9900/12097
.................................................................................................... 10000/12097
iiiiii.i..iiiiii.i....F.....F....................................................................... 10100/12097
...................................................................F................................ 10300/12097
.................................................................................................... 10400/12097
.................................................................................................... 10400/12097
........F...F....................................................................................... 10500/12097
.................................................................................................... 10700/12097
.................................................................................................... 10800/12097
...........ii..............................i.........................F.............................. 10900/12097
.................................................................................................... 11000/12097
.................................................................................................... 11000/12097
.................................................................................................... 11100/12097
.......................................................................F............................ 11200/12097
.................................................................................................... 11300/12097
.................................................i.................................................. 11400/12097
.................................................................................................... 11500/12097
.................................................................................................... 11600/12097
.................................................................................................... 11700/12097
........F........................................................................F............F....F 11800/12097
...........F...........F.....F............F......................................................... 11900/12097
.................................................................................................
failures:

---- [ui] ui/async-await/async-unsafe-fn-call-in-safe.rs#mir stdout ----
---- [ui] ui/async-await/async-unsafe-fn-call-in-safe.rs#mir stdout ----
diff of stderr:

14    |
15    = note: consult the function's documentation for information on how to avoid undefined behavior
16 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/async-unsafe-fn-call-in-safe.rs:19:5
-    |
- LL |     S::f();
-    |     ^^^^^^ call to unsafe function
-    |
-    = note: consult the function's documentation for information on how to avoid undefined behavior
- 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/async-unsafe-fn-call-in-safe.rs:20:5
- LL |     f();
-    |     ^^^ call to unsafe function
-    |
-    |
-    = note: consult the function's documentation for information on how to avoid undefined behavior
- error: aborting due to 4 previous errors
+ error: aborting due to 2 previous errors
34 
35 For more information about this error, try `rustc --explain E0133`.
35 For more information about this error, try `rustc --explain E0133`.
36 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.mir/async-unsafe-fn-call-in-safe.mir.stderr
To only update this specific test, also pass `--test-args async-await/async-unsafe-fn-call-in-safe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     S::f(); //~ ERROR call to unsafe function is unsafe
   |     ^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     f(); //~ ERROR call to unsafe function is unsafe
   |     ^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/binding/issue-53114-safety-checks.rs stdout ----
diff of stderr:

- warning: reference to packed field is unaligned
-   --> $DIR/issue-53114-safety-checks.rs:23:13
+ error[E0133]: access to union field is unsafe and requires unsafe function or block
3    |
- LL |     let _ = &p.b;
-    |             ^^^^
-    |             ^^^^
+ LL |     let _ = u1.a;  // #53114: should eventually signal error as well
+    |             ^^^^ access to union field
-    = note: `#[warn(unaligned_references)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
- warning: reference to packed field is unaligned
-   --> $DIR/issue-53114-safety-checks.rs:29:17
-    |
-    |
- LL |     let (_,) = (&p.b,);
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
- warning: reference to packed field is unaligned
-   --> $DIR/issue-53114-safety-checks.rs:39:11
-    |
-    |
- LL |     match &p.b  { _ => { } }
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
- warning: reference to packed field is unaligned
-   --> $DIR/issue-53114-safety-checks.rs:45:12
-    |
-    |
- LL |     match (&p.b,)  { (_,) => { } }
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
- 
42 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/issue-53114-safety-checks.rs:26:13
44    |
45 LL |     let _ = &u2.a;
-    |             ^^^^^ access to union field
+    |              ^^^^ access to union field
+    |              ^^^^ access to union field
47    |
48    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


56    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
57 
58 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/issue-53114-safety-checks.rs:32:17
60    |
60    |
61 LL |     let (_,) = (&u2.a,);
-    |                 ^^^^^ access to union field
+    |                  ^^^^ access to union field
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
64    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

+ warning: reference to packed field is unaligned
+   --> $DIR/issue-53114-safety-checks.rs:23:13
+    |
+    |
+ LL |     let _ = &p.b;
+    |             ^^^^
+    |
+    = note: `#[warn(unaligned_references)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+ warning: reference to packed field is unaligned
+   --> $DIR/issue-53114-safety-checks.rs:29:17
+    |
+    |
+ LL |     let (_,) = (&p.b,);
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+ 
66 error[E0133]: access to union field is unsafe and requires unsafe function or block
68    |


72    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
73 
74 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/issue-53114-safety-checks.rs:42:11
76    |
76    |
77 LL |     match &u2.a { _ => { } }
-    |           ^^^^^ access to union field
+    |            ^^^^ access to union field
79    |
80    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


88    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
89 
90 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/issue-53114-safety-checks.rs:48:12
92    |
92    |
93 LL |     match (&u2.a,) { (_,) => { } }
-    |            ^^^^^ access to union field
+    |             ^^^^ access to union field
95    |
96    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

- error: aborting due to 7 previous errors; 4 warnings emitted
+ warning: reference to packed field is unaligned
+   --> $DIR/issue-53114-safety-checks.rs:39:11
+   --> $DIR/issue-53114-safety-checks.rs:39:11
+    |
+ LL |     match &p.b  { _ => { } }
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+ warning: reference to packed field is unaligned
+   --> $DIR/issue-53114-safety-checks.rs:45:12
+    |
+    |
+ LL |     match (&p.b,)  { (_,) => { } }
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+ error: aborting due to 8 previous errors; 4 warnings emitted
99 
100 For more information about this error, try `rustc --explain E0133`.
101 
---
To only update this specific test, also pass `--test-args binding/issue-53114-safety-checks.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/issue-53114-safety-checks.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     let _ = u1.a;  // #53114: should eventually signal error as well
   |             ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     let _ = &u2.a; //~ ERROR  [E0133]
   |              ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     let (_,) = (u1.a,);  //~ ERROR   [E0133]
   |                 ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     let (_,) = (&u2.a,); //~ ERROR   [E0133]
   |                  ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:23:13
   |
   |
LL |     let _ = &p.b;  //~ WARN    reference to packed field
   |
   = note: `#[warn(unaligned_references)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:29:17
   |
   |
LL |     let (_,) = (&p.b,);  //~ WARN     reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     match u1.a  { _ => { } } //~ ERROR   [E0133]
   |           ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     match &u2.a { _ => { } } //~ ERROR   [E0133]
   |            ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     match (u1.a,)  { (_,) => { } } //~ ERROR   [E0133]
   |            ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     match (&u2.a,) { (_,) => { } } //~ ERROR   [E0133]
   |             ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:39:11
   |
   |
LL |     match &p.b  { _ => { } } //~ WARN     reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:45:12
   |
   |
LL |     match (&p.b,)  { (_,) => { } } //~ WARN     reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: aborting due to 8 previous errors; 4 warnings emitted

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs#mir stdout ----
diff of stderr:

1 error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/const-extern-fn-requires-unsafe.rs:11:5
- LL |     foo();
-    |     ^^^^^ call to unsafe function
-    |
-    |
-    = note: consult the function's documentation for information on how to avoid undefined behavior
- 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
11    |
11    |
12 LL |     let a: [u8; foo()];
14    |
14    |
15    = note: consult the function's documentation for information on how to avoid undefined behavior
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
18 
19 For more information about this error, try `rustc --explain E0133`.
19 For more information about this error, try `rustc --explain E0133`.
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.mir/const-extern-fn-requires-unsafe.mir.stderr
To only update this specific test, also pass `--test-args consts/const-extern-fn/const-extern-fn-requires-unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     let a: [u8; foo()];
   |                 ^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
diff of stderr:

2   --> $DIR/issue-45729-unsafe-in-generator.rs:8:9
3    |
4 LL |         *(1 as *mut u32) = 42;
-    |         ^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer
+    |         ^^^^^^^^^^^^^^^^ dereference of raw pointer
6    |
7    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-45729-unsafe-in-generator.mir/issue-45729-unsafe-in-generator.mir.stderr
To only update this specific test, also pass `--test-args generator/issue-45729-unsafe-in-generator.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/issue-45729-unsafe-in-generator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-45729-unsafe-in-generator.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-45729-unsafe-in-generator.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
   |
   |
LL |         *(1 as *mut u32) = 42;
   |         ^^^^^^^^^^^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/impl-trait/auto-trait-leak.rs stdout ----
diff of stderr:

19    |
20 LL | fn cycle1() -> impl Clone {
21    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires unsafety-checking `cycle1`...
-   --> $DIR/auto-trait-leak.rs:12:1
-    |
- LL | fn cycle1() -> impl Clone {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^
27 note: ...which requires building MIR for `cycle1`...
29    |


51 LL | fn cycle2() -> impl Clone {
52    | ^^^^^^^^^^^^^^^^^^^^^^^^^
53 note: ...which requires processing MIR for `cycle2`...
-   --> $DIR/auto-trait-leak.rs:19:1
-    |
- LL | fn cycle2() -> impl Clone {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires unsafety-checking `cycle2`...
60    |
60    |
61 LL | fn cycle2() -> impl Clone {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auto-trait-leak.stderr
To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
   |
   |
LL | fn cycle1() -> impl Clone {
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
note: ...which requires computing type of `cycle2::{opaque#0}`...
   |
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
   = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / use std::cell::Cell;
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | | fn send<T: Send>(_: T) {}
...  |
LL | |     Rc::new(String::from("foo"))
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.

------------------------------------------


---- [ui] ui/impl-trait/closure-in-impl-trait-arg.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/closure-in-impl-trait-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/closure-in-impl-trait-arg/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/closure-in-impl-trait-arg/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing normalized predicates of `bug`
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |
   |
note: ...which requires simplifying constant for the type system `bug::{opaque#0}::{constant#0}`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                     ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires simplifying constant for the type system `bug::{opaque#0}::{constant#0}`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                     ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `bug::{opaque#0}::{constant#0}`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                     ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires caching mir of `bug::{opaque#0}::{constant#0}` for CTFE...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                     ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires elaborating drops for `bug::{opaque#0}::{constant#0}`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                     ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires borrow-checking `bug::{opaque#0}::{constant#0}`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                     ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires borrow-checking `bug::{opaque#0}::{constant#0}::{closure#0}`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                       ^^^^^^^^
note: ...which requires processing `bug::{opaque#0}::{constant#0}::{closure#0}`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                       ^^^^^^^^
note: ...which requires processing MIR for `bug::{opaque#0}::{constant#0}::{closure#0}`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                       ^^^^^^^^
note: ...which requires building MIR for `bug::{opaque#0}::{constant#0}::{closure#0}`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                       ^^^^^^^^
note: ...which requires unsafety-checking `bug::{opaque#0}::{constant#0}::{closure#0}`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   |                                       ^^^^^^^^
note: ...which requires unsafety-checking `bug`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `bug`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `bug`...
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires computing normalized predicates of `bug`, completing the cycle
note: cycle used when checking that `bug` is well-formed
   |
   |
LL | fn bug(_: impl Iterator<Item = [(); { |x: u32| { x }; 4 }]>) {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.

------------------------------------------


---- [ui] ui/impl-trait/closure-in-impl-trait.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/closure-in-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/closure-in-impl-trait/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/closure-in-impl-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `ok::{constant#0}`
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |
   |
note: ...which requires simplifying constant for the type system `ok::{constant#0}`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |                                            ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `ok::{constant#0}`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |                                            ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires caching mir of `ok::{constant#0}` for CTFE...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |                                            ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires elaborating drops for `ok::{constant#0}`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |                                            ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires borrow-checking `ok::{constant#0}`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |                                            ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires borrow-checking `ok::{constant#0}::{closure#0}`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |                                              ^^^^^^^^
note: ...which requires processing `ok::{constant#0}::{closure#0}`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |                                              ^^^^^^^^
note: ...which requires processing MIR for `ok::{constant#0}::{closure#0}`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |                                              ^^^^^^^^
note: ...which requires building MIR for `ok::{constant#0}::{closure#0}`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |                                              ^^^^^^^^
note: ...which requires unsafety-checking `ok::{constant#0}::{closure#0}`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   |                                              ^^^^^^^^
note: ...which requires unsafety-checking `ok`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `ok`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `ok`...
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires simplifying constant for the type system `ok::{constant#0}`, completing the cycle
note: cycle used when checking that `ok` is well-formed
   |
   |
LL | fn ok<T>() -> Box<dyn Iterator<Item = [(); { |x: u32| { x }; 4 }]>> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.

------------------------------------------


---- [ui] ui/issues/issue-16538.rs#mir stdout ----
diff of stderr:

+ error[E0133]: use of extern static is unsafe and requires unsafe function or block
+    |
+    |
+ LL | static foo: *const Y::X = Y::foo(Y::x as *const Y::X);
+    |                                  ^^^^ use of extern static
+    |
+    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
+ 
1 error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
3    |

12    |
12    |
13    = help: the trait `Sync` is not implemented for `*const usize`
14    = note: shared static variables must have a type that implements `Sync`
- 
- error[E0133]: use of extern static is unsafe and requires unsafe function or block
-   --> $DIR/issue-16538.rs:14:34
-    |
- LL | static foo: *const Y::X = Y::foo(Y::x as *const Y::X);
-    |                                  ^^^^ use of extern static
-    |
-    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
24 error: aborting due to 3 previous errors
25 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16538.mir/issue-16538.mir.stderr
To only update this specific test, also pass `--test-args issues/issue-16538.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16538.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16538.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16538.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of extern static is unsafe and requires unsafe function or block
   |
   |
LL | static foo: *const Y::X = Y::foo(Y::x as *const Y::X);
   |                                  ^^^^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior

error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
   |
   |
LL | static foo: *const Y::X = Y::foo(Y::x as *const Y::X);


error[E0277]: `*const usize` cannot be shared between threads safely
   |
   |
LL | static foo: *const Y::X = Y::foo(Y::x as *const Y::X);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `*const usize` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `*const usize`
   = note: shared static variables must have a type that implements `Sync`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0015, E0133, E0277.
For more information about an error, try `rustc --explain E0015`.
For more information about an error, try `rustc --explain E0015`.

------------------------------------------


---- [ui] ui/issues/issue-28324.rs#mir stdout ----
diff of stderr:

1 error[E0133]: use of extern static is unsafe and requires unsafe function or block
-   --> $DIR/issue-28324.rs:8:24
3    |
3    |
4 LL | pub static BAZ: u32 = *&error_message_count;
-    |                        ^^^^^^^^^^^^^^^^^^^^ use of extern static
+    |                         ^^^^^^^^^^^^^^^^^^^ use of extern static
6    |
7    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28324.mir/issue-28324.mir.stderr
To only update this specific test, also pass `--test-args issues/issue-28324.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28324.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28324.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28324.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of extern static is unsafe and requires unsafe function or block
   |
   |
LL | pub static BAZ: u32 = *&error_message_count;
   |                         ^^^^^^^^^^^^^^^^^^^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/pattern/usefulness/issue-57472.rs stdout ----
diff of stderr:

16 LL |             Punned { bar: [_] } => println!("bar"),
18 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0133]: access to union field is unsafe and requires unsafe function or block
+    |
+    |
+ LL |             Punned { foo: [_] } => println!("foo"),
+    |                           ^^^ access to union field
+    |
+    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
20 
+ error[E0133]: access to union field is unsafe and requires unsafe function or block
+    |
---
To only update this specific test, also pass `--test-args pattern/usefulness/issue-57472.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/issue-57472.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/issue-57472" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/issue-57472/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/issue-57472.rs:15:13
   |
LL |             Punned { bar: [_], .. } => println!("bar"),
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/pattern/usefulness/issue-57472.rs:2:9
   |
   |
LL | #![deny(unreachable_patterns)]

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/issue-57472.rs:31:13
   |
   |
LL |             Punned { bar: [_] } => println!("bar"),


error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |             Punned { foo: [_] } => println!("foo"),
   |                           ^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |             Punned { bar: [_] } => println!("bar"),
   |                           ^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/safe-extern-statics-mut.rs#mir stdout ----
diff of stderr:

7    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
8 
9 error[E0133]: use of mutable static is unsafe and requires unsafe function or block
-   --> $DIR/safe-extern-statics-mut.rs:14:14
11    |
11    |
12 LL |     let rb = &B;
-    |              ^^ use of mutable static
+    |               ^ use of mutable static
14    |
15    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior


23    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
24 
25 error[E0133]: use of mutable static is unsafe and requires unsafe function or block
-   --> $DIR/safe-extern-statics-mut.rs:16:15
27    |
27    |
28 LL |     let xrb = &XB;
-    |               ^^^ use of mutable static
+    |                ^^ use of mutable static
30    |
31    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics-mut.mir/safe-extern-statics-mut.mir.stderr
To only update this specific test, also pass `--test-args safe-extern-statics-mut.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/safe-extern-statics-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics-mut.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics-mut.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     let b = B; //~ ERROR use of mutable static is unsafe
   |             ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     let rb = &B; //~ ERROR use of mutable static is unsafe
   |               ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     let xb = XB; //~ ERROR use of mutable static is unsafe
   |              ^^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     let xrb = &XB; //~ ERROR use of mutable static is unsafe
   |                ^^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/safe-extern-statics.rs#mir stdout ----
diff of stderr:

7    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
8 
9 error[E0133]: use of extern static is unsafe and requires unsafe function or block
-   --> $DIR/safe-extern-statics.rs:14:14
11    |
11    |
12 LL |     let ra = &A;
-    |              ^^ use of extern static
+    |               ^ use of extern static
14    |
15    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior


23    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
24 
25 error[E0133]: use of extern static is unsafe and requires unsafe function or block
-   --> $DIR/safe-extern-statics.rs:16:15
27    |
27    |
28 LL |     let xra = &XA;
-    |               ^^^ use of extern static
+    |                ^^ use of extern static
30    |
31    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics.mir/safe-extern-statics.mir.stderr
To only update this specific test, also pass `--test-args safe-extern-statics.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/safe-extern-statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/safe-extern-statics.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of extern static is unsafe and requires unsafe function or block
   |
   |
LL |     let a = A; //~ ERROR use of extern static is unsafe
   |             ^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior

error[E0133]: use of extern static is unsafe and requires unsafe function or block
   |
   |
LL |     let ra = &A; //~ ERROR use of extern static is unsafe
   |               ^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior

error[E0133]: use of extern static is unsafe and requires unsafe function or block
   |
   |
LL |     let xa = XA; //~ ERROR use of extern static is unsafe
   |              ^^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior

error[E0133]: use of extern static is unsafe and requires unsafe function or block
   |
   |
LL |     let xra = &XA; //~ ERROR use of extern static is unsafe
   |                ^^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/span/lint-unused-unsafe.rs#mir stdout ----
diff of stderr:

47    |         ^^^^^^ unnecessary `unsafe` block
48 
49 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:32:5
51    |
51    |
- LL | unsafe fn bad7() {
-    | ---------------- because it's nested under this `unsafe` fn
54 LL |     unsafe {
-    |     ^^^^^^ unnecessary `unsafe` block
+    |     ------ because it's nested under this `unsafe` block
+ LL |         unsafe {
+    |         ^^^^^^ unnecessary `unsafe` block
56 
57 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:33:9
59    |
59    |
60 LL | unsafe fn bad7() {
61    | ---------------- because it's nested under this `unsafe` fn
62 LL |     unsafe {
- LL |         unsafe {
- LL |         unsafe {
-    |         ^^^^^^ unnecessary `unsafe` block
+    |     ^^^^^^ unnecessary `unsafe` block
66 error: aborting due to 8 previous errors
67 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/lint-unused-unsafe.mir/lint-unused-unsafe.mir.stderr
To only update this specific test, also pass `--test-args span/lint-unused-unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/lint-unused-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/lint-unused-unsafe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/lint-unused-unsafe.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unnecessary `unsafe` block
   |
   |
LL | fn bad1() { unsafe {} }                  //~ ERROR: unnecessary `unsafe` block
   |             ^^^^^^ unnecessary `unsafe` block
note: the lint level is defined here
  --> /checkout/src/test/ui/span/lint-unused-unsafe.rs:7:9
   |
   |
LL | #![deny(unused_unsafe)]


error: unnecessary `unsafe` block
   |
   |
LL | fn bad2() { unsafe { bad1() } }          //~ ERROR: unnecessary `unsafe` block
   |             ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL | unsafe fn bad3() { unsafe {} }           //~ ERROR: unnecessary `unsafe` block
   | ----------------   ^^^^^^ unnecessary `unsafe` block
   | |
   | because it's nested under this `unsafe` fn

error: unnecessary `unsafe` block
   |
   |
LL | fn bad4() { unsafe { callback(||{}) } }  //~ ERROR: unnecessary `unsafe` block
   |             ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL | unsafe fn bad5() { unsafe { unsf() } }   //~ ERROR: unnecessary `unsafe` block
   | ----------------   ^^^^^^ unnecessary `unsafe` block
   | |
   | because it's nested under this `unsafe` fn

error: unnecessary `unsafe` block
   |
   |
LL |     unsafe {                             // don't put the warning here
   |     ------ because it's nested under this `unsafe` block
LL |         unsafe {                         //~ ERROR: unnecessary `unsafe` block
   |         ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL |     unsafe {                             //~ ERROR: unnecessary `unsafe` block
   |     ------ because it's nested under this `unsafe` block
LL |         unsafe {                         //~ ERROR: unnecessary `unsafe` block
   |         ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL | unsafe fn bad7() {
   | ---------------- because it's nested under this `unsafe` fn
LL |     unsafe {                             //~ ERROR: unnecessary `unsafe` block
   |     ^^^^^^ unnecessary `unsafe` block
error: aborting due to 8 previous errors


------------------------------------------
---
4 LL |     a += 3;
-    |     ^^^^^^ use of mutable static
+    |     ^ use of mutable static
6    |
7    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

10   --> $DIR/static-mut-foreign-requires-unsafe.rs:10:5
11    |
12 LL |     a = 4;
12 LL |     a = 4;
-    |     ^^^^^ use of mutable static
+    |     ^ use of mutable static
14    |
15    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-foreign-requires-unsafe.mir/static-mut-foreign-requires-unsafe.mir.stderr
To only update this specific test, also pass `--test-args static/static-mut-foreign-requires-unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-mut-foreign-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-foreign-requires-unsafe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-foreign-requires-unsafe.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     a += 3; //~ ERROR: requires unsafe
   |     ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     a = 4; //~ ERROR: requires unsafe
   |     ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     let _b = a; //~ ERROR: requires unsafe
   |              ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0133`.

---
4 LL |     a += 3;
-    |     ^^^^^^ use of mutable static
+    |     ^ use of mutable static
6    |
7    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

10   --> $DIR/static-mut-requires-unsafe.rs:8:5
11    |
12 LL |     a = 4;
12 LL |     a = 4;
-    |     ^^^^^ use of mutable static
+    |     ^ use of mutable static
14    |
15    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-requires-unsafe.mir/static-mut-requires-unsafe.mir.stderr
To only update this specific test, also pass `--test-args static/static-mut-requires-unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-mut-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-requires-unsafe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-requires-unsafe.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     a += 3;         //~ ERROR: requires unsafe
   |     ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     a = 4;          //~ ERROR: requires unsafe
   |     ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     let _b = a;     //~ ERROR: requires unsafe
   |              ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/thread-local-static.rs stdout ----
diff of stderr:

+ error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
+    |
+    |
+ LL |     std::mem::swap(x, &mut STATIC_VAR_2)
+    |                            ^^^^^^^^^^^^ dereference of raw pointer
+    |
+    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
1 error[E0658]: mutable references are not allowed in constant functions
2   --> $DIR/thread-local-static.rs:7:12
3    |


29    |
30    = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
31    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
- 
- error[E0133]: use of mutable static is unsafe and requires unsafe function or block
-   --> $DIR/thread-local-static.rs:9:23
-    |
- LL |     std::mem::swap(x, &mut STATIC_VAR_2)
-    |                       ^^^^^^^^^^^^^^^^^ use of mutable static
-    |
-    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
41 error: aborting due to 5 previous errors
42 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-static/thread-local-static.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args thread-local-static.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/thread-local-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-static" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-static/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
   |
   |
LL |     std::mem::swap(x, &mut STATIC_VAR_2)
   |                            ^^^^^^^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/thread-local-static.rs:7:12
   |
   |
LL | const fn g(x: &mut [u32; 8]) {
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0625]: thread-local statics cannot be accessed at compile-time
  --> /checkout/src/test/ui/thread-local-static.rs:9:28
   |
LL |     std::mem::swap(x, &mut STATIC_VAR_2)

error[E0013]: constant functions cannot refer to statics
  --> /checkout/src/test/ui/thread-local-static.rs:9:28
   |
   |
LL |     std::mem::swap(x, &mut STATIC_VAR_2)
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/thread-local-static.rs:9:23
   |
   |
LL |     std::mem::swap(x, &mut STATIC_VAR_2)
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

---
diff of stderr:

2   --> $DIR/safety-fn-body.rs:14:9
3    |
4 LL |         *self += 1;
-    |         ^^^^^^^^^^ dereference of raw pointer
+    |         ^^^^^ dereference of raw pointer
6    |
7    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/safety-fn-body.mir/safety-fn-body.mir.stderr
To only update this specific test, also pass `--test-args traits/safety-fn-body.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/safety-fn-body.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/safety-fn-body.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/safety-fn-body.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
   |
   |
LL |         *self += 1;
   |         ^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/union/union-unsafe.rs#mir stdout ----
diff of stderr:

1 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/union-unsafe.rs:34:5
3    |
3    |
4 LL |     *(u.p) = 13;
-    |     ^^^^^^^^^^^ access to union field
+    |      ^^^^^ access to union field
6    |
7    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe.mir/union-unsafe.mir.stderr
To only update this specific test, also pass `--test-args union/union-unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *(u.p) = 13; //~ ERROR access to union field is unsafe
   |      ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: assignment to union field that might need dropping is unsafe and requires unsafe function or block
   |
   |
LL |     u.a = (RefCell::new(0), 1); //~ ERROR assignment to union field that might need dropping
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to union field that might need dropping
   |
   = note: the previous content of the field will be dropped, which causes undefined behavior if the field was not properly initialized

error[E0133]: assignment to union field that might need dropping is unsafe and requires unsafe function or block
   |
   |
LL |     u.a.0 = RefCell::new(0); //~ ERROR assignment to union field that might need dropping
   |     ^^^^^^^^^^^^^^^^^^^^^^^ assignment to union field that might need dropping
   |
   = note: the previous content of the field will be dropped, which causes undefined behavior if the field was not properly initialized

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *u3.a = T::default(); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *u3.a = T::default(); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     let a = u1.a; //~ ERROR access to union field is unsafe
   |             ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     let U1 { a } = u1; //~ ERROR access to union field is unsafe
   |              ^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     if let U1 { a: 12 } = u1 {} //~ ERROR access to union field is unsafe
   |                    ^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *u2.a = String::from("new"); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *u3.a = 1; //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *u3.a = String::from("new"); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0133`.

---
diff of stderr:

2   --> $DIR/issue-45087-unreachable-unsafe.rs:6:5
3    |
4 LL |     *(1 as *mut u32) = 42;
-    |     ^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer
+    |     ^^^^^^^^^^^^^^^^ dereference of raw pointer
6    |
7    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/issue-45087-unreachable-unsafe.mir/issue-45087-unreachable-unsafe.mir.stderr
To only update this specific test, also pass `--test-args unsafe/issue-45087-unreachable-unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/issue-45087-unreachable-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/issue-45087-unreachable-unsafe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/issue-45087-unreachable-unsafe.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
   |
   |
LL |     *(1 as *mut u32) = 42;
   |     ^^^^^^^^^^^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/unsafe/ranged_ints2_const.rs#mirunsafeck stdout ----
diff of stderr:

+ error[E0133]: mutation of layout constrained field is unsafe and requires unsafe function or block
+    |
+    |
+ LL |     let y = &mut x.0;
+    |             ^^^^^^^^ mutation of layout constrained field
+    |
+    = note: mutating layout constrained fields cannot statically be checked for valid values
1 error[E0658]: mutable references are not allowed in constant functions
2   --> $DIR/ranged_ints2_const.rs:14:13
3    |


24    |
25    = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
26    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
- 
- error[E0133]: mutation of layout constrained field is unsafe and requires unsafe function or block
-   --> $DIR/ranged_ints2_const.rs:14:13
-    |
- LL |     let y = &mut x.0;
-    |             ^^^^^^^^ mutation of layout constrained field
-    |
-    = note: mutating layout constrained fields cannot statically be checked for valid values
36 error: aborting due to 4 previous errors
37 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const.mirunsafeck/ranged_ints2_const.mirunsafeck.stderr
To only update this specific test, also pass `--test-args unsafe/ranged_ints2_const.rs`


error in revision `mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints2_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const.mirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const.mirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: mutation of layout constrained field is unsafe and requires unsafe function or block
   |
   |
LL |     let y = &mut x.0; //~ ERROR mutable references
   |             ^^^^^^^^ mutation of layout constrained field
   |
   = note: mutating layout constrained fields cannot statically be checked for valid values
error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:14:13
   |
   |
LL |     let y = &mut x.0; //~ ERROR mutable references
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:21:22
   |
LL |     let y = unsafe { &mut x.0 }; //~ ERROR mutable references
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:27:22
   |
LL |     unsafe { let y = &mut x.0; } //~ ERROR mutable references
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

---

---- [ui] ui/unsafe/ranged_ints3_const.rs#mirunsafeck stdout ----
diff of stderr:

+ error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe function or block
+    |
+    |
+ LL |     let y = &x.0;
+    |             ^^^^ borrow of layout constrained field with interior mutability
+    |
+    = note: references to fields of layout constrained fields lose the constraints. Coupled with interior mutability, the field can be changed to invalid values
+ 
1 error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
3    |

15    |
16    = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
16    = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
17    = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable
- 
- error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe function or block
-   --> $DIR/ranged_ints3_const.rs:15:13
-    |
- LL |     let y = &x.0;
-    |             ^^^^ borrow of layout constrained field with interior mutability
-    |
-    = note: references to fields of layout constrained fields lose the constraints. Coupled with interior mutability, the field can be changed to invalid values
27 error: aborting due to 3 previous errors
28 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.mirunsafeck/ranged_ints3_const.mirunsafeck.stderr
To only update this specific test, also pass `--test-args unsafe/ranged_ints3_const.rs`


error in revision `mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints3_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.mirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.mirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe function or block
   |
   |
LL |     let y = &x.0; //~ ERROR the borrowed element may contain interior mutability
   |             ^^^^ borrow of layout constrained field with interior mutability
   |
   = note: references to fields of layout constrained fields lose the constraints. Coupled with interior mutability, the field can be changed to invalid values

error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL |     let y = &x.0; //~ ERROR the borrowed element may contain interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable


error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL |     let y = unsafe { &x.0 }; //~ ERROR the borrowed element may contain interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable

---
25 LL |     VOID = ();
-    |     ^^^^^^^^^ use of mutable static
+    |     ^^^^ use of mutable static
27    |
28    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

65   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:31:5
66    |
67 LL |     VOID = ();
67 LL |     VOID = ();
-    |     ^^^^^^^^^ use of mutable static
+    |     ^^^^ use of mutable static
69    |
70    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir/rfc-2585-unsafe_op_in_unsafe_fn.mir.stderr
To only update this specific test, also pass `--test-args unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: call to unsafe function is unsafe and requires unsafe block (error E0133)
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:4:9
   |
LL | #![deny(unsafe_op_in_unsafe_fn)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: dereference of raw pointer is unsafe and requires unsafe block (error E0133)
   |
LL |     *PTR;
LL |     *PTR;
   |     ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error: use of mutable static is unsafe and requires unsafe block (error E0133)
   |
LL |     VOID = ();
   |     ^^^^ use of mutable static
   |
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error: unnecessary `unsafe` block
   |
LL |     unsafe {}
LL |     unsafe {}
   |     ^^^^^^ unnecessary `unsafe` block
note: the lint level is defined here
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:5:9
   |
   |
LL | #![deny(unused_unsafe)]


error: call to unsafe function is unsafe and requires unsafe block (error E0133)
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:25:8
   |
LL | #[deny(warnings)]
   |        ^^^^^^^^
   = note: `#[deny(unsafe_op_in_unsafe_fn)]` implied by `#[deny(warnings)]`
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: dereference of raw pointer is unsafe and requires unsafe block (error E0133)
   |
LL |     *PTR;
LL |     *PTR;
   |     ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error: use of mutable static is unsafe and requires unsafe block (error E0133)
   |
LL |     VOID = ();
   |     ^^^^ use of mutable static
   |
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error: unnecessary `unsafe` block
   |
LL |     unsafe {}
LL |     unsafe {}
   |     ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL |     unsafe { unsafe { unsf() } }
   |     ------   ^^^^^^ unnecessary `unsafe` block
   |     |
   |     because it's nested under this `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL | unsafe fn allow_level() {
   | ----------------------- because it's nested under this `unsafe` fn
...
LL |     unsafe { unsf() }
   |     ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL | unsafe fn nested_allow_level() {
   | ------------------------------ because it's nested under this `unsafe` fn
...
LL |         unsafe { unsf() }
   |         ^^^^^^ unnecessary `unsafe` block

error[E0133]: call to unsafe function is unsafe and requires unsafe block
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
LL |         unsf();
   |         ^^^^^^ call to unsafe function
   |
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0133`.

---
diff of stderr:

2   --> $DIR/unsafe-fn-assign-deref-ptr.rs:5:5
3    |
4 LL |     *p = 0;
-    |     ^^^^^^ dereference of raw pointer
+    |     ^^ dereference of raw pointer
6    |
7    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr.mir/unsafe-fn-assign-deref-ptr.mir.stderr
To only update this specific test, also pass `--test-args unsafe/unsafe-fn-assign-deref-ptr.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-assign-deref-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
   |
   |
LL |     *p = 0; //~ ERROR dereference of raw pointer is unsafe
   |     ^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/unsafe/union.rs#mir stdout ----
diff of stderr:

- warning: unnecessary `unsafe` block
-   --> $DIR/union.rs:61:5
- LL |     unsafe {
- LL |     unsafe {
-    |     ^^^^^^ unnecessary `unsafe` block
-    = note: `#[warn(unused_unsafe)]` on by default
- 
- 
- warning: unnecessary `unsafe` block
-   --> $DIR/union.rs:66:5
- LL |     unsafe {
- LL |     unsafe {
-    |     ^^^^^^ unnecessary `unsafe` block
- warning: 2 warnings emitted
- 
- 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union.mir/union.mir.stderr
To only update this specific test, also pass `--test-args unsafe/union.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/union.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union.mir/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/unsafe/union_destructure.rs#mir stdout ----
diff of stderr:

- warning: unnecessary `unsafe` block
-   --> $DIR/union_destructure.rs:35:5
- LL |     unsafe {
- LL |     unsafe {
-    |     ^^^^^^ unnecessary `unsafe` block
-    = note: `#[warn(unused_unsafe)]` on by default
- 
- 
- warning: unnecessary `unsafe` block
-   --> $DIR/union_destructure.rs:41:5
- LL |     unsafe {
- LL |     unsafe {
-    |     ^^^^^^ unnecessary `unsafe` block
- warning: 2 warnings emitted
- 
- 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union_destructure.mir/union_destructure.mir.stderr
To only update this specific test, also pass `--test-args unsafe/union_destructure.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/union_destructure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union_destructure.mir/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union_destructure.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 11969 passed; 25 failed; 103 ignored; 0 measured; 0 filtered out; finished in 121.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:10
