plain
.............................................................iiiiii.i..iiiiiiiiiii.i.... 11264/13880
........................................................................................ 11352/13880
........................................................................................ 11440/13880
........................................................................................ 11528/13880
........................................F............................................... 11616/13880
..........................................................................F...........F. 11704/13880
.F.F.................................................................................... 11792/13880
........................................................................................ 11968/13880
........................................................................................ 12056/13880
........................................................................................ 12144/13880
....................................................................................i... 12232/13880
....................................................................................i... 12232/13880
....i.........i....i.....................i.....................F......F................. 12320/13880
........................................................................................ 12496/13880
........................................................................................ 12584/13880
........................................................................................ 12672/13880
........................F............................................................... 12760/13880
---
........................................................................................ 13200/13880
........................................................................................ 13288/13880
........................................................................................ 13376/13880
........................................................................................ 13464/13880
...........F............................................................................ 13552/13880
....F....F..............F...F.........FF...F............F.F..F......FF.F................ 13640/13880
..........................................iii........................................... 13816/13880
................................................................
failures:


---- [ui] src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs#mir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `S::f` is unsafe and requires unsafe function or block
3    |
3    |
4 LL |     S::f();
6    |
7    = note: consult the function's documentation for information on how to avoid undefined behavior
8 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `f` is unsafe and requires unsafe function or block
11    |
12 LL |     f();

14    |
14    |
15    = note: consult the function's documentation for information on how to avoid undefined behavior
16 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/async-unsafe-fn-call-in-safe.rs:23:5
-    |
- LL |     S::f();
-    |
-    = note: consult the function's documentation for information on how to avoid undefined behavior
- 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
---
36 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.mir/async-unsafe-fn-call-in-safe.mir.stderr
To only update this specific test, also pass `--test-args async-await/async-unsafe-fn-call-in-safe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.mir/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `S::f` is unsafe and requires unsafe function or block
   |
   |
LL |     S::f();
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior


error[E0133]: call to unsafe function `f` is unsafe and requires unsafe function or block
   |
LL |     f();
   |     ^^^ call to unsafe function
   |
---

+ error[E0133]: access to union field is unsafe and requires unsafe function or block
+   --> $DIR/issue-53114-safety-checks.rs:25:13
+    |
+ LL |     let _ = u1.a;  // #53114: should eventually signal error as well
+    |             ^^^^ access to union field
+    |
+    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
+ error[E0133]: access to union field is unsafe and requires unsafe function or block
+   --> $DIR/issue-53114-safety-checks.rs:26:14
+    |
+ LL |     let _ = &u2.a;
+ LL |     let _ = &u2.a;
+    |              ^^^^ access to union field
+    |
+    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
+ error[E0133]: access to union field is unsafe and requires unsafe function or block
+   --> $DIR/issue-53114-safety-checks.rs:31:17
+    |
+    |
+ LL |     let (_,) = (u1.a,);
+    |                 ^^^^ access to union field
+    |
+    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
+ error[E0133]: access to union field is unsafe and requires unsafe function or block
+   --> $DIR/issue-53114-safety-checks.rs:32:18
+    |
+    |
+ LL |     let (_,) = (&u2.a,);
+    |                  ^^^^ access to union field
+    |
+    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
1 error: reference to packed field is unaligned
2   --> $DIR/issue-53114-safety-checks.rs:23:13
3    |


21    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
22    = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
23 
- error: reference to packed field is unaligned
-   --> $DIR/issue-53114-safety-checks.rs:39:11
-    |
- LL |     match &p.b  { _ => { } }
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
-    = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
- 
- error: reference to packed field is unaligned
-   --> $DIR/issue-53114-safety-checks.rs:45:12
-    |
- LL |     match (&p.b,)  { (_,) => { } }
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
---
-    |
- LL |     let _ = &u2.a;
-    |             ^^^^^ access to union field
-    |
-    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
- error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/issue-53114-safety-checks.rs:31:17
-    |
-    |
- LL |     let (_,) = (u1.a,);
-    |                 ^^^^ access to union field
-    |
-    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
- error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/issue-53114-safety-checks.rs:32:17
-    |
-    |
- LL |     let (_,) = (&u2.a,);
-    |                 ^^^^^ access to union field
-    |
-    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
- error[E0133]: access to union field is unsafe and requires unsafe function or block
71   --> $DIR/issue-53114-safety-checks.rs:41:11
72    |
72    |
73 LL |     match u1.a  { _ => { } }

76    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
78 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/issue-53114-safety-checks.rs:42:11
+   --> $DIR/issue-53114-safety-checks.rs:42:12
80    |
80    |
81 LL |     match &u2.a { _ => { } }
-    |           ^^^^^ access to union field
+    |            ^^^^ access to union field
83    |
84    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


92    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
94 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/issue-53114-safety-checks.rs:48:12
+   --> $DIR/issue-53114-safety-checks.rs:48:13
96    |
96    |
97 LL |     match (&u2.a,) { (_,) => { } }
-    |            ^^^^^ access to union field
+    |             ^^^^ access to union field
99    |
100    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

- error: aborting due to 11 previous errors
+ error: reference to packed field is unaligned
+   --> $DIR/issue-53114-safety-checks.rs:39:11
+   --> $DIR/issue-53114-safety-checks.rs:39:11
+    |
+ LL |     match &p.b  { _ => { } }
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+    = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
+ 
+ error: reference to packed field is unaligned
+   --> $DIR/issue-53114-safety-checks.rs:45:12
+    |
+ LL |     match (&p.b,)  { (_,) => { } }
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
---
To only update this specific test, also pass `--test-args binding/issue-53114-safety-checks.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/issue-53114-safety-checks.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     let _ = u1.a;  // #53114: should eventually signal error as well
   |             ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:26:14
   |
   |
LL |     let _ = &u2.a; //~ ERROR  [E0133]
   |              ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:31:17
   |
   |
LL |     let (_,) = (u1.a,);  //~ ERROR   [E0133]
   |                 ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:32:18
   |
   |
LL |     let (_,) = (&u2.a,); //~ ERROR   [E0133]
   |                  ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:23:13
   |
   |
LL |     let _ = &p.b;  //~ ERROR    reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
   = note: `#[deny(unaligned_references)]` on by default

error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:29:17
   |
LL |     let (_,) = (&p.b,);  //~ ERROR     reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)

error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:41:11
   |
LL |     match u1.a  { _ => { } } //~ ERROR   [E0133]
   |           ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:42:12
   |
   |
LL |     match &u2.a { _ => { } } //~ ERROR   [E0133]
   |            ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:47:12
   |
   |
LL |     match (u1.a,)  { (_,) => { } } //~ ERROR   [E0133]
   |            ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:48:13
   |
   |
LL |     match (&u2.a,) { (_,) => { } } //~ ERROR   [E0133]
   |             ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:39:11
   |
   |
LL |     match &p.b  { _ => { } } //~ ERROR     reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)

error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:45:12
   |
LL |     match (&p.b,)  { (_,) => { } } //~ ERROR     reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
---
Future incompatibility report: Future breakage diagnostic:
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:23:13
   |
LL |     let _ = &p.b;  //~ ERROR    reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
   = note: `#[deny(unaligned_references)]` on by default

Future breakage diagnostic:
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:29:17
   |
LL |     let (_,) = (&p.b,);  //~ ERROR     reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
   = note: `#[deny(unaligned_references)]` on by default

Future breakage diagnostic:
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:39:11
   |
LL |     match &p.b  { _ => { } } //~ ERROR     reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
   = note: `#[deny(unaligned_references)]` on by default

Future breakage diagnostic:
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:45:12
   |
LL |     match (&p.b,)  { (_,) => { } } //~ ERROR     reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
---

---- [ui] src/test/ui/closures/coerce-unsafe-closure-to-unsafe-fn-ptr.rs#mir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `Pin::<P>::new_unchecked` is unsafe and requires unsafe function or block
2   --> $DIR/coerce-unsafe-closure-to-unsafe-fn-ptr.rs:5:31
3    |
4 LL |     let _: unsafe fn() = || { ::std::pin::Pin::new_unchecked(&0_u8); };

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-closure-to-unsafe-fn-ptr.mir/coerce-unsafe-closure-to-unsafe-fn-ptr.mir.stderr
To only update this specific test, also pass `--test-args closures/coerce-unsafe-closure-to-unsafe-fn-ptr.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/coerce-unsafe-closure-to-unsafe-fn-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-closure-to-unsafe-fn-ptr.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-closure-to-unsafe-fn-ptr.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `Pin::<P>::new_unchecked` is unsafe and requires unsafe function or block
   |
   |
LL |     let _: unsafe fn() = || { ::std::pin::Pin::new_unchecked(&0_u8); };
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error
---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `foo` is unsafe and requires unsafe function or block
10   --> $DIR/const-extern-fn-requires-unsafe.rs:9:17
11    |
12 LL |     let a: [u8; foo()];
14    |
15    = note: consult the function's documentation for information on how to avoid undefined behavior
16 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
18 
19 For more information about this error, try `rustc --explain E0133`.
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.mir/const-extern-fn-requires-unsafe.mir.stderr
To only update this specific test, also pass `--test-args consts/const-extern-fn/const-extern-fn-requires-unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `foo` is unsafe and requires unsafe function or block
   |
   |
LL |     let a: [u8; foo()];
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error
---

---- [ui] src/test/ui/error-codes/E0133.rs#mir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `f` is unsafe and requires unsafe function or block
2   --> $DIR/E0133.rs:7:5
4 LL |     f();


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0133.mir/E0133.mir.stderr
To only update this specific test, also pass `--test-args error-codes/E0133.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0133.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0133.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0133.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `f` is unsafe and requires unsafe function or block
   |
LL |     f();
   |     ^^^ call to unsafe function
   |
---
1 error[E0133]: use of extern static is unsafe and requires unsafe function or block
-   --> $DIR/issue-28324.rs:8:24
+   --> $DIR/issue-28324.rs:8:25
3    |
4 LL | pub static BAZ: u32 = *&error_message_count;
+    |                         ^^^^^^^^^^^^^^^^^^^ use of extern static
6    |
6    |
7    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-28324.mir/issue-28324.mir.stderr
To only update this specific test, also pass `--test-args extern/issue-28324.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-28324.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-28324.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-28324.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: use of extern static is unsafe and requires unsafe function or block
   |
   |
LL | pub static BAZ: u32 = *&error_message_count;
   |
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/foreign-unsafe-fn-called.rs#mir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `test::free` is unsafe and requires unsafe function or block
3    |
4 LL |     test::free();



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign-unsafe-fn-called.mir/foreign-unsafe-fn-called.mir.stderr
To only update this specific test, also pass `--test-args foreign-unsafe-fn-called.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/foreign-unsafe-fn-called.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign-unsafe-fn-called.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign-unsafe-fn-called.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `test::free` is unsafe and requires unsafe function or block
   |
LL |     test::free();
   |     ^^^^^^^^^^^^ call to unsafe function
   |
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/issue-45729-unsafe-in-generator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-45729-unsafe-in-generator.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-45729-unsafe-in-generator.mir/auxiliary"
stdout: none
--- stderr -------------------------------
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
------------------------------------------


---- [ui] src/test/ui/impl-trait/auto-trait-leak.rs stdout ----
diff of stderr:

19    |
20 LL | fn cycle1() -> impl Clone {
21    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires unsafety-checking `cycle1`...
+ note: ...which requires checking if `cycle1` contains FFI-unwind calls...
24    |
24    |
25 LL | fn cycle1() -> impl Clone {
29    |
29    |
30 LL | fn cycle1() -> impl Clone {
31    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires unsafety-checking `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
32 note: ...which requires building THIR for `cycle1`...
34    |

60    |
60    |
61 LL | fn cycle2() -> impl Clone {
62    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires unsafety-checking `cycle2`...
+ note: ...which requires checking if `cycle2` contains FFI-unwind calls...
65    |
65    |
66 LL | fn cycle2() -> impl Clone {
67    | ^^^^^^^^^^^^^^^^^^^^^^^^^
67    | ^^^^^^^^^^^^^^^^^^^^^^^^^
68 note: ...which requires building MIR for `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires unsafety-checking `cycle2`...
70    |
70    |
71 LL | fn cycle2() -> impl Clone {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auto-trait-leak.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
stdout: none
--- stderr -------------------------------
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
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires preparing `cycle1` for borrow checking...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires checking if `cycle1` contains FFI-unwind calls...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
   |
LL |     send(cycle2().clone());
   |     ^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
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
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires preparing `cycle2` for borrow checking...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires checking if `cycle2` contains FFI-unwind calls...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
   |
LL |     send(cycle1().clone());
   |     ^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
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
---

6    |
7    = note: consult the function's documentation for information on how to avoid undefined behavior
8 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `unchecked_sub` is unsafe and requires unsafe function or block
11    |
12 LL |     let sub = std::intrinsics::unchecked_sub(x, y);

14    |
14    |
15    = note: consult the function's documentation for information on how to avoid undefined behavior
16 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `unchecked_mul` is unsafe and requires unsafe function or block
19    |
20 LL |     let mul = std::intrinsics::unchecked_mul(x, y);



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/unchecked_math_unsafe.mir/unchecked_math_unsafe.mir.stderr
To only update this specific test, also pass `--test-args intrinsics/unchecked_math_unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/intrinsics/unchecked_math_unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/unchecked_math_unsafe.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/unchecked_math_unsafe.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `unchecked_add` is unsafe and requires unsafe function or block
   |
   |
LL |     let add = std::intrinsics::unchecked_add(x, y); //~ ERROR call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior


error[E0133]: call to unsafe function `unchecked_sub` is unsafe and requires unsafe function or block
   |
   |
LL |     let sub = std::intrinsics::unchecked_sub(x, y); //~ ERROR call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior


error[E0133]: call to unsafe function `unchecked_mul` is unsafe and requires unsafe function or block
   |
   |
LL |     let mul = std::intrinsics::unchecked_mul(x, y); //~ ERROR call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to 3 previous errors
---

---- [ui] src/test/ui/issues/issue-16538.rs#mir stdout ----
diff of stderr:

- error[E0015]: cannot call non-const fn `Y::foo` in statics
-   --> $DIR/issue-16538.rs:14:23
+ error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
3    |
3    |
4 LL | static foo: &Y::X = &*Y::foo(Y::x as *const Y::X);
+    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer
6    |
-    = note: calls in statics are limited to constant functions, tuple structs and tuple variants
-    = note: calls in statics are limited to constant functions, tuple structs and tuple variants
-    = note: consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate: https://crates.io/crates/once_cell
+    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
10 error[E0133]: use of extern static is unsafe and requires unsafe function or block
11   --> $DIR/issue-16538.rs:14:30

15    |
15    |
16    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
- error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
-   --> $DIR/issue-16538.rs:14:21
-   --> $DIR/issue-16538.rs:14:21
+ error[E0015]: cannot call non-const fn `Y::foo` in statics
20    |
20    |
21 LL | static foo: &Y::X = &*Y::foo(Y::x as *const Y::X);
-    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer
23    |
23    |
-    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
+    = note: calls in statics are limited to constant functions, tuple structs and tuple variants
+    = note: consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate: https://crates.io/crates/once_cell
26 error: aborting due to 3 previous errors
27 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16538.mir/issue-16538.mir.stderr
To only update this specific test, also pass `--test-args issues/issue-16538.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16538.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16538.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16538.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
   |
   |
LL | static foo: &Y::X = &*Y::foo(Y::x as *const Y::X);
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error[E0133]: use of extern static is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/issues/issue-16538.rs:14:30
   |
   |
LL | static foo: &Y::X = &*Y::foo(Y::x as *const Y::X);
   |
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior

error[E0015]: cannot call non-const fn `Y::foo` in statics
   |
   |
LL | static foo: &Y::X = &*Y::foo(Y::x as *const Y::X);
   |
   = note: calls in statics are limited to constant functions, tuple structs and tuple variants
   = note: calls in statics are limited to constant functions, tuple structs and tuple variants
   = note: consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate: https://crates.io/crates/once_cell
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0015, E0133.
For more information about an error, try `rustc --explain E0015`.
---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `std::ptr::write` is unsafe and requires unsafe function or block
2   --> $DIR/issue-28776.rs:7:5
3    |
4 LL |     (&ptr::write)(1 as *mut _, 42);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28776.mir/issue-28776.mir.stderr
To only update this specific test, also pass `--test-args issues/issue-28776.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28776.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28776.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28776.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `std::ptr::write` is unsafe and requires unsafe function or block
   |
   |
LL |     (&ptr::write)(1 as *mut _, 42);
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error
---
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `rand` is unsafe and requires unsafe function or block
2   --> $DIR/issue-5844.rs:8:5
3    |
4 LL |     issue_5844_aux::rand();

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5844.mir/issue-5844.mir.stderr
To only update this specific test, also pass `--test-args issues/issue-5844.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-5844.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5844.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5844.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `rand` is unsafe and requires unsafe function or block
   |
   |
LL |     issue_5844_aux::rand(); //~ ERROR: requires unsafe
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error
---

---- [ui] src/test/ui/pattern/usefulness/issue-57472.rs stdout ----
diff of stderr:

16 LL |             Punned { bar: [_] } => println!("bar"),
18 
- error: aborting due to 2 previous errors
+ error[E0133]: access to union field is unsafe and requires unsafe function or block
+   --> $DIR/issue-57472.rs:30:27
+   --> $DIR/issue-57472.rs:30:27
+    |
+ LL |             Punned { foo: [_] } => println!("foo"),
+    |                           ^^^ access to union field
+    |
+    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
+ error[E0133]: access to union field is unsafe and requires unsafe function or block
+   --> $DIR/issue-57472.rs:31:27
+    |
+    |
+ LL |             Punned { bar: [_] } => println!("bar"),
+    |                           ^^^ access to union field
+    |
+    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
+ error: aborting due to 4 previous errors
+ 
+ For more information about this error, try `rustc --explain E0133`.
21 
---
To only update this specific test, also pass `--test-args pattern/usefulness/issue-57472.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/issue-57472.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/issue-57472" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/issue-57472/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/pattern/usefulness/issue-57472.rs:15:13
   |
   |
LL |             Punned { bar: [_], .. } => println!("bar"),
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/pattern/usefulness/issue-57472.rs:2:9
   |
   |
LL | #![deny(unreachable_patterns)]
   |         ^^^^^^^^^^^^^^^^^^^^

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/issue-57472.rs:31:13
   |
LL |             Punned { bar: [_] } => println!("bar"),

error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/pattern/usefulness/issue-57472.rs:30:27
   |
   |
LL |             Punned { foo: [_] } => println!("foo"),
   |                           ^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/pattern/usefulness/issue-57472.rs:31:27
   |
   |
LL |             Punned { bar: [_] } => println!("bar"),
   |                           ^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs#mir stdout ----
diff of stderr:

- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
3    |
4 LL |     sse2();

6    |
6    |
7    = note: can only be called if the required target features are available
8 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
11    |
11    |
12 LL |     avx_bmi2();
14    |
15    = note: can only be called if the required target features are available
16 
16 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
19    |
19    |
20 LL |     Quux.avx_bmi2();
22    |
23    = note: can only be called if the required target features are available
24 
24 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
27    |
27    |
28 LL |     avx_bmi2();
30    |
31    = note: can only be called if the required target features are available
32 
32 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
35    |
35    |
36 LL |     Quux.avx_bmi2();
38    |
39    = note: can only be called if the required target features are available
40 
40 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
43    |
44 LL |     sse2();

46    |
46    |
47    = note: can only be called if the required target features are available
48 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
51    |
51    |
52 LL |     avx_bmi2();
54    |
55    = note: can only be called if the required target features are available
56 
56 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
59    |
59    |
60 LL |     Quux.avx_bmi2();
62    |
63    = note: can only be called if the required target features are available
64 
64 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
67    |
68 LL |     sse2();

70    |
70    |
71    = note: can only be called if the required target features are available
72 
- error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
+ error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
75    |
75    |
76 LL | const name: () = sse2();

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.mir/safe-calls.mir.stderr
To only update this specific test, also pass `--test-args rfcs/rfc-2396-target_feature-11/safe-calls.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
   |
LL |     sse2();
LL |     sse2();
   |     ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
   |
   |
LL |     avx_bmi2();
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
   |
   |
LL |     Quux.avx_bmi2();
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
   |
   |
LL |     avx_bmi2();
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
   |
   |
LL |     Quux.avx_bmi2();
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
   |
LL |     sse2();
LL |     sse2();
   |     ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
   |
   |
LL |     avx_bmi2();
   |     ^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block
   |
   |
LL |     Quux.avx_bmi2();
   |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
   |
LL |     sse2();
LL |     sse2();
   |     ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available


error[E0133]: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block
   |
   |
LL | const name: () = sse2();
   |                  ^^^^^^ call to function with `#[target_feature]`
   = note: can only be called if the required target features are available

error: aborting due to 10 previous errors


For more information about this error, try `rustc --explain E0133`.
------------------------------------------


---- [ui] src/test/ui/span/lint-unused-unsafe.rs#mir stdout ----
diff of stderr:

1 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:26:13
3    |
3    |
- LL | fn bad1() { unsafe {} }
-    |             ^^^^^^ unnecessary `unsafe` block
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
7 note: the lint level is defined here
8   --> $DIR/lint-unused-unsafe.rs:14:9

11    |         ^^^^^^^^^^^^^
11    |         ^^^^^^^^^^^^^
12 
13 error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ ...
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ LL |                 let _ = async { unsf() };
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ ...
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ ...
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ ...
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ LL |                 let _ = async { unsf() };
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ ...
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+ LL |             let _ = async { unsafe {
+ LL |             let _ = async { unsafe {
+    |                             ------ because it's nested under this `unsafe` block
+ ...
+ LL |                 let _ = async { unsafe { let _ = async { unsf() }; }};
+    |                                 ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+    |
+ LL |         let _x: [(); unsafe { unsafe { size() } }] = [];
+    |                      ------   ^^^^^^ unnecessary `unsafe` block
+    |                      |
+    |                      because it's nested under this `unsafe` block
+ 
+ error: unnecessary `unsafe` block
+    |
+    |
+ LL | fn bad1() { unsafe {} }
+    |             ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
15    |
15    |
16 LL | fn bad2() { unsafe { bad1() } }
20   --> $DIR/lint-unused-unsafe.rs:28:20
21    |
21    |
22 LL | unsafe fn bad3() { unsafe {} }
-    |                    ^^^^^^ unnecessary `unsafe` block
+    | ----------------   ^^^^^^ unnecessary `unsafe` block
+    | |
+    | because it's nested under this `unsafe` fn
24 
25 error: unnecessary `unsafe` block


29    |             ^^^^^^ unnecessary `unsafe` block
30 
31 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:32:5
33    |
34 LL |     unsafe {
34 LL |     unsafe {
-    |     ^^^^^^ unnecessary `unsafe` block
+    |     ------ because it's nested under this `unsafe` block
+ LL |         unsafe {                         // don't put the warning here
+    |         ^^^^^^ unnecessary `unsafe` block
36 
37 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:39:5
39    |
40 LL |     unsafe {
40 LL |     unsafe {
-    |     ^^^^^^ unnecessary `unsafe` block
+    |     ------ because it's nested under this `unsafe` block
+ LL |         unsafe {
+    |         ^^^^^^ unnecessary `unsafe` block
42 
43 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:74:9
45    |
46 LL |         unsafe {
46 LL |         unsafe {
-    |         ^^^^^^ unnecessary `unsafe` block
+    |         ------ because it's nested under this `unsafe` block
+ LL |             unsafe {}
+    |             ^^^^^^ unnecessary `unsafe` block
48 
49 error: unnecessary `unsafe` block
+    |
+ LL |         unsafe {
+ LL |         unsafe {
+    |         ------ because it's nested under this `unsafe` block
+ LL |             unsafe {}
+ LL |             unsafe {}
+    |             ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
51    |
52 LL |         unsafe {


53    |         ^^^^^^ unnecessary `unsafe` block
54 
55 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:84:13
57    |
- LL |             unsafe {}
+ LL |         unsafe {
+ LL |         unsafe {
+    |         ------ because it's nested under this `unsafe` block
+ LL |             unsafe { unsf() }
59    |             ^^^^^^ unnecessary `unsafe` block
60 
61 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:85:13
+   --> $DIR/lint-unused-unsafe.rs:92:13
63    |
- LL |             unsafe {}
- LL |             unsafe {}
+ LL |         unsafe {
+    |         ------ because it's nested under this `unsafe` block
+ LL |             unsafe { unsf() }
+ LL |             unsafe { unsf() }
65    |             ^^^^^^ unnecessary `unsafe` block
66 
67 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:90:9
+   --> $DIR/lint-unused-unsafe.rs:93:13
69    |
70 LL |         unsafe {
70 LL |         unsafe {
-    |         ^^^^^^ unnecessary `unsafe` block
+    |         ------ because it's nested under this `unsafe` block
+ ...
+ LL |             unsafe { unsf() }
+    |             ^^^^^^ unnecessary `unsafe` block
72 
73 error: unnecessary `unsafe` block


131    |                 ^^^^^^ unnecessary `unsafe` block
132 
133 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:124:9
135    |
136 LL |         unsafe {
136 LL |         unsafe {
-    |         ^^^^^^ unnecessary `unsafe` block
+    |         ------ because it's nested under this `unsafe` block
+ LL |             unsafe {}
+    |             ^^^^^^ unnecessary `unsafe` block
138 
139 error: unnecessary `unsafe` block
+    |
+ LL |         unsafe {
+ LL |         unsafe {
+    |         ------ because it's nested under this `unsafe` block
+ LL |             unsafe {}
+ LL |             unsafe {}
+    |             ^^^^^^ unnecessary `unsafe` block
+ 
+ error: unnecessary `unsafe` block
141    |
142 LL |         unsafe {


143    |         ^^^^^^ unnecessary `unsafe` block
144 
145 error: unnecessary `unsafe` block
-   --> $DIR/lint-unused-unsafe.rs:135:13
147    |
- LL |             unsafe {}
+ LL |         unsafe {
+ LL |         unsafe {
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-mut-foreign-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-foreign-requires-unsafe.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-foreign-requires-unsafe.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     a += 3; //~ ERROR: requires unsafe
   |     ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/static/static-mut-foreign-requires-unsafe.rs:10:5
   |
   |
LL |     a = 4; //~ ERROR: requires unsafe
   |     ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/static/static-mut-foreign-requires-unsafe.rs:11:14
   |
   |
LL |     let _b = a; //~ ERROR: requires unsafe
   |              ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/static/safe-extern-statics.rs#mir stdout ----
diff of stderr:

7    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
9 error[E0133]: use of extern static is unsafe and requires unsafe function or block
-   --> $DIR/safe-extern-statics.rs:14:14
+   --> $DIR/safe-extern-statics.rs:14:15
11    |
11    |
12 LL |     let ra = &A;
-    |              ^^ use of extern static
+    |               ^ use of extern static
14    |
15    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior


23    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
25 error[E0133]: use of extern static is unsafe and requires unsafe function or block
-   --> $DIR/safe-extern-statics.rs:16:15
+   --> $DIR/safe-extern-statics.rs:16:16
27    |
27    |
28 LL |     let xra = &XA;
+    |                ^^ use of extern static
30    |
30    |
31    = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/safe-extern-statics.mir/safe-extern-statics.mir.stderr
To only update this specific test, also pass `--test-args static/safe-extern-statics.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/safe-extern-statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/safe-extern-statics.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/safe-extern-statics.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: use of extern static is unsafe and requires unsafe function or block
   |
   |
LL |     let a = A; //~ ERROR use of extern static is unsafe
   |             ^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error[E0133]: use of extern static is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/static/safe-extern-statics.rs:14:15
   |
   |
LL |     let ra = &A; //~ ERROR use of extern static is unsafe
   |               ^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error[E0133]: use of extern static is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/static/safe-extern-statics.rs:15:14
   |
   |
LL |     let xa = XA; //~ ERROR use of extern static is unsafe
   |              ^^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error[E0133]: use of extern static is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/static/safe-extern-statics.rs:16:16
   |
   |
LL |     let xra = &XA; //~ ERROR use of extern static is unsafe
   |                ^^ use of extern static
   |
   = note: extern statics are not controlled by the Rust type system: invalid data, aliasing violations or data races will cause undefined behavior
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-mut-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-requires-unsafe.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-requires-unsafe.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     a += 3;         //~ ERROR: requires unsafe
   |     ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/static/static-mut-requires-unsafe.rs:8:5
   |
   |
LL |     a = 4;          //~ ERROR: requires unsafe
   |     ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/static/static-mut-requires-unsafe.rs:9:14
   |
   |
LL |     let _b = a;     //~ ERROR: requires unsafe
   |              ^ use of mutable static
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/thread-local-static.rs stdout ----
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
-    |
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/thread-local-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-static" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-static/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
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

---- [ui] src/test/ui/threads-sendsync/issue-43733.rs#mir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `$LOCALKEYINNER::<T>::get` is unsafe and requires unsafe function or block
3    |
3    |
4 LL |     __KEY.get(Default::default)
6    |
7    = note: consult the function's documentation for information on how to avoid undefined behavior
8 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `LocalKey::<T>::new` is unsafe and requires unsafe function or block
11    |
11    |
12 LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir/issue-43733.mir.stderr
To only update this specific test, also pass `--test-args threads-sendsync/issue-43733.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `__FastLocalKeyInner::<T>::get` is unsafe and requires unsafe function or block
   |
   |
LL |     __KEY.get(Default::default)
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior


error[E0133]: call to unsafe function `LocalKey::<T>::new` is unsafe and requires unsafe function or block
   |
   |
LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to 2 previous errors
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/safety-fn-body.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/safety-fn-body.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/safety-fn-body.mir/auxiliary"
stdout: none
--- stderr -------------------------------
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
------------------------------------------


---- [ui] src/test/ui/type-alias-enum-variants/self-in-enum-definition.rs stdout ----
diff of stderr:

44    |
45 LL |     V3 = Self::V1 {} as u8 + 2,
46    |          ^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires unsafety-checking `Alpha::V3::{constant#0}`...
-   --> $DIR/self-in-enum-definition.rs:5:10
-    |
- LL |     V3 = Self::V1 {} as u8 + 2,
-    |          ^^^^^^^^^^^^^^^^^^^^^
52 note: ...which requires building MIR for `Alpha::V3::{constant#0}`...
54    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition/self-in-enum-definition.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-enum-variants/self-in-enum-definition.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-enum-variants/self-in-enum-definition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `Alpha::V3::{constant#0}`
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |
   |
note: ...which requires simplifying constant for the type system `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires caching mir of `Alpha::V3::{constant#0}` for CTFE...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires elaborating drops for `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires borrow-checking `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires preparing `Alpha::V3::{constant#0}` for borrow checking...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing layout of `Alpha`...
   = note: ...which again requires simplifying constant for the type system `Alpha::V3::{constant#0}`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
LL | / #[repr(u8)]
LL | / #[repr(u8)]
LL | | enum Alpha {
LL | |     V1 = 41,
LL | |     V2 = Self::V1 as u8 + 1,    // OK; See #50072.
LL | |
LL | | fn main() {}
   | |____________^

---
1 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/union-unsafe.rs:33:5
+   --> $DIR/union-unsafe.rs:33:6
3    |
4 LL |     *(u.p) = 13;
-    |     ^^^^^^^^^^^ access to union field
+    |      ^^^^^ access to union field
6    |
7    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


39    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
41 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/union-unsafe.rs:64:12
+   --> $DIR/union-unsafe.rs:64:8
43    |
43    |
44 LL |     if let U1 { a: 12 } = u1 {}
-    |            ^^^^^^^^^^^^ access to union field
+    |        ^^^^^^^^^^^^^^^^^^^^^ access to union field
46    |
47    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe.mir/union-unsafe.mir.stderr
To only update this specific test, also pass `--test-args union/union-unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *(u.p) = 13; //~ ERROR access to union field is unsafe
   |      ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/union/union-unsafe.rs:46:6
   |
   |
LL |     *u3.a = T::default(); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/union/union-unsafe.rs:52:6
   |
   |
LL |     *u3.a = T::default(); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/union/union-unsafe.rs:60:13
   |
   |
LL |     let a = u1.a; //~ ERROR access to union field is unsafe
   |             ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/union/union-unsafe.rs:63:14
   |
   |
LL |     let U1 { a } = u1; //~ ERROR access to union field is unsafe
   |              ^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/union/union-unsafe.rs:64:8
   |
   |
LL |     if let U1 { a: 12 } = u1 {} //~ ERROR access to union field is unsafe
   |        ^^^^^^^^^^^^^^^^^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/union/union-unsafe.rs:69:6
   |
   |
LL |     *u2.a = String::from("new"); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/union/union-unsafe.rs:73:6
   |
   |
LL |     *u3.a = 1; //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/union/union-unsafe.rs:77:6
   |
   |
LL |     *u3.a = String::from("new"); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/unsafe/issue-45087-unreachable-unsafe.rs#mir stdout ----
diff of stderr:

2   --> $DIR/issue-45087-unreachable-unsafe.rs:7:5
3    |
4 LL |     *(1 as *mut u32) = 42;
-    |     ^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer
+    |     ^^^^^^^^^^^^^^^^ dereference of raw pointer
6    |
7    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

10   --> $DIR/issue-45087-unreachable-unsafe.rs:17:5
11    |
12 LL |     *a = 1;
12 LL |     *a = 1;
-    |     ^^^^^^ dereference of raw pointer
+    |     ^^ dereference of raw pointer
14    |
15    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

18   --> $DIR/issue-45087-unreachable-unsafe.rs:29:5
19    |
19    |
20 LL |     *b = 1;
-    |     ^^^^^^ dereference of raw pointer
+    |     ^^ dereference of raw pointer
22    |
23    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/issue-45087-unreachable-unsafe.mir/issue-45087-unreachable-unsafe.mir.stderr
To only update this specific test, also pass `--test-args unsafe/issue-45087-unreachable-unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/issue-45087-unreachable-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/issue-45087-unreachable-unsafe.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/issue-45087-unreachable-unsafe.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
   |
   |
LL |     *(1 as *mut u32) = 42;
   |     ^^^^^^^^^^^^^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/issue-45087-unreachable-unsafe.rs:17:5
   |
LL |     *a = 1;
LL |     *a = 1;
   |     ^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/issue-45087-unreachable-unsafe.rs:29:5
   |
   |
LL |     *b = 1;
   |     ^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/unsafe/issue-3080.rs#mir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `X::with` is unsafe and requires unsafe function or block
3    |
3    |
4 LL |     X(()).with();

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/issue-3080.mir/issue-3080.mir.stderr
To only update this specific test, also pass `--test-args unsafe/issue-3080.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/issue-3080.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/issue-3080.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/issue-3080.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `X::with` is unsafe and requires unsafe function or block
   |
   |
LL |     X(()).with(); //~ ERROR requires unsafe function or block
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error
---

---- [ui] src/test/ui/unsafe/ranged_ints2_const.rs#mirunsafeck stdout ----
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints2_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const.mirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const.mirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
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

---- [ui] src/test/ui/unsafe/ranged_ints3_const.rs#mirunsafeck stdout ----
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints3_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.mirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.mirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
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
diff of stderr:

2   --> $DIR/union-assignop.rs:19:5
3    |
4 LL |     foo.a += 5;
-    |     ^^^^^^^^^^ access to union field
+    |     ^^^^^ access to union field
6    |
7    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union-assignop.mirunsafeck/union-assignop.mirunsafeck.stderr
To only update this specific test, also pass `--test-args unsafe/union-assignop.rs`


error in revision `mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/union-assignop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union-assignop.mirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union-assignop.mirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     foo.a += 5; //~ ERROR access to union field is unsafe
   |     ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/union-assignop.rs:20:6
   |
   |
LL |     *foo.b += NonCopy; //~ ERROR access to union field is unsafe
   |      ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/union-assignop.rs:21:6
   |
   |
LL |     *foo.b = NonCopy; //~ ERROR access to union field is unsafe
   |      ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/union-assignop.rs:23:5
   |
   |
LL |     foo.a; //~ ERROR access to union field is unsafe
   |     ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/union-assignop.rs:25:5
   |
   |
LL |     foo.b; //~ ERROR access to union field is unsafe
   |     ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/union-assignop.rs:27:13
   |
   |
LL |     foo.b = foo.b;
   |             ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs#mir stdout ----
diff of stderr:

- error: call to unsafe function is unsafe and requires unsafe block (error E0133)
+ error: call to unsafe function `unsf` is unsafe and requires unsafe block (error E0133)
2   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:12:5
4 LL |     unsf();

23   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:17:5
24    |
24    |
25 LL |     VOID = ();
-    |     ^^^^^^^^^ use of mutable static
+    |     ^^^^ use of mutable static
27    |
28    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

39 LL | #![deny(unused_unsafe)]
40    |         ^^^^^^^^^^^^^
41 
41 
- error: call to unsafe function is unsafe and requires unsafe block (error E0133)
+ error: call to unsafe function `unsf` is unsafe and requires unsafe block (error E0133)
43   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:28:5
45 LL |     unsf();

65   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:33:5
66    |
66    |
67 LL |     VOID = ();
-    |     ^^^^^^^^^ use of mutable static
+    |     ^^^^ use of mutable static
69    |
70    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior


76    |     ^^^^^^ unnecessary `unsafe` block
77 
78 error: unnecessary `unsafe` block
-   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:49:5
+   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:49:14
80    |
81 LL |     unsafe { unsafe { unsf() } }
-    |     ^^^^^^ unnecessary `unsafe` block
+    |     ------   ^^^^^^ unnecessary `unsafe` block
+    |     |
+    |     because it's nested under this `unsafe` block
- error[E0133]: call to unsafe function is unsafe and requires unsafe block
- error[E0133]: call to unsafe function is unsafe and requires unsafe block
+ error[E0133]: call to unsafe function `unsf` is unsafe and requires unsafe block
85   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:76:5
87 LL |     unsf();

89    |
90    = note: consult the function's documentation for information on how to avoid undefined behavior
90    = note: consult the function's documentation for information on how to avoid undefined behavior
91 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `unsf` is unsafe and requires unsafe function or block
93   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:81:9
95 LL |         unsf();


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir/rfc-2585-unsafe_op_in_unsafe_fn.mir.stderr
To only update this specific test, also pass `--test-args unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error: call to unsafe function `unsf` is unsafe and requires unsafe block (error E0133)
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
note: the lint level is defined here
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:4:9
   |
LL | #![deny(unsafe_op_in_unsafe_fn)]
   |         ^^^^^^^^^^^^^^^^^^^^^^

error: dereference of raw pointer is unsafe and requires unsafe block (error E0133)
   |
LL |     *PTR;
   |     ^^^^ dereference of raw pointer
   |
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
LL | #![deny(unused_unsafe)]
LL | #![deny(unused_unsafe)]
   |         ^^^^^^^^^^^^^

error: call to unsafe function `unsf` is unsafe and requires unsafe block (error E0133)
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
---
LL | #[deny(warnings)]
   |        ^^^^^^^^
   = note: `#[deny(unsafe_op_in_unsafe_fn)]` implied by `#[deny(warnings)]`

error: dereference of raw pointer is unsafe and requires unsafe block (error E0133)
   |
LL |     *PTR;
   |     ^^^^ dereference of raw pointer
   |
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

error[E0133]: call to unsafe function `unsf` is unsafe and requires unsafe block
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function `unsf` is unsafe and requires unsafe function or block
   |
LL |         unsf();
   |         ^^^^^^ call to unsafe function
   |
---

---- [ui] src/test/ui/unsafe/union.rs#mir stdout ----
diff of stderr:

7    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
9 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/union.rs:32:11
+   --> $DIR/union.rs:34:20
11    |
11    |
- LL |     match foo {
-    |           ^^^ access to union field
+ LL |               pizza: Pizza {
+    |  ____________________^
+ LL | |                 topping: Some(PizzaTopping::Cheese) | Some(PizzaTopping::Pineapple) | None
+ LL | |             }
+    | |_____________^ access to union field
14    |
15    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

- error: aborting due to 2 previous errors
+ error[E0133]: access to union field is unsafe and requires unsafe function or block
+   --> $DIR/union.rs:42:20
+   --> $DIR/union.rs:42:20
+    |
+ LL |         Foo { zst: () } => {},
+    |                    ^^ access to union field
+    |
+    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
+ error[E0133]: access to union field is unsafe and requires unsafe function or block
+   --> $DIR/union.rs:45:22
+    |
+    |
+ LL |         Foo { pizza: Pizza { .. } } => {},
+    |                      ^^^^^^^^^^^^ access to union field
+    |
+    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
+ error: aborting due to 4 previous errors
18 
19 For more information about this error, try `rustc --explain E0133`.
20 
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union.mir/union.mir.stderr
To only update this specific test, also pass `--test-args unsafe/union.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/union.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |         Foo { bar: _a } => {}, //~ ERROR access to union field is unsafe
   |                    ^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/union.rs:34:20
   |
   |
LL |               pizza: Pizza { //[thir]~ ERROR access to union field is unsafe
   |  ____________________^
LL | |                 topping: Some(PizzaTopping::Cheese) | Some(PizzaTopping::Pineapple) | None
LL | |             }
   | |_____________^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/union.rs:42:20
   |
   |
LL |         Foo { zst: () } => {}, //[thir]~ ERROR access to union field is unsafe
   |                    ^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/union.rs:45:22
   |
   |
LL |         Foo { pizza: Pizza { .. } } => {}, //[thir]~ ERROR access to union field is unsafe
   |                      ^^^^^^^^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
---
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-assign-deref-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-assign-deref-ptr.mir/auxiliary"
stdout: none
--- stderr -------------------------------
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
------------------------------------------


---- [ui] src/test/ui/unsafe/unsafe-const-fn.rs#mir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `dummy` is unsafe and requires unsafe function or block
3    |
3    |
4 LL | const VAL: u32 = dummy(0xFFFF);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-const-fn.mir/unsafe-const-fn.mir.stderr
To only update this specific test, also pass `--test-args unsafe/unsafe-const-fn.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-const-fn.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-const-fn.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `dummy` is unsafe and requires unsafe function or block
   |
   |
LL | const VAL: u32 = dummy(0xFFFF);
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error
---

---- [ui] src/test/ui/unsafe/unsafe-fn-called-from-safe.rs#mir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `f` is unsafe and requires unsafe function or block
3    |
4 LL |     f();



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-called-from-safe.mir/unsafe-fn-called-from-safe.mir.stderr
To only update this specific test, also pass `--test-args unsafe/unsafe-fn-called-from-safe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-called-from-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-called-from-safe.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-called-from-safe.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `f` is unsafe and requires unsafe function or block
   |
LL |     f();
   |     ^^^ call to unsafe function
   |
---

---- [ui] src/test/ui/unsafe/union_destructure.rs#mir stdout ----
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/union_destructure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union_destructure.mir/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/union_destructure.mir/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/unsafe/unsafe-fn-used-as-value.rs#mir stdout ----
diff of stderr:


- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `f` is unsafe and requires unsafe function or block
3    |
4 LL |     x();



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-used-as-value.mir/unsafe-fn-used-as-value.mir.stderr
To only update this specific test, also pass `--test-args unsafe/unsafe-fn-used-as-value.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-used-as-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-used-as-value.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-used-as-value.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: call to unsafe function `f` is unsafe and requires unsafe function or block
   |
LL |     x();
   |     ^^^ call to unsafe function
   |
---
diff of stderr:

2   --> $DIR/unsafe-not-inherited.rs:6:31
3    |
4 LL |     unsafe {static BAR: u64 = FOO;}
-    |     |
-    |     |
-    |     items do not inherit unsafety from separate enclosing items
8    |
8    |
9    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
+ error[E0133]: call to unsafe function `unsafe_call` is unsafe and requires unsafe function or block
13    |
- LL |     unsafe {
- LL |     unsafe {
-    |     ------ items do not inherit unsafety from separate enclosing items
17 LL |             unsafe_call();
18    |             ^^^^^^^^^^^^^ call to unsafe function
19    |

---
To only update this specific test, also pass `--test-args unsafe/unsafe-not-inherited.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-not-inherited.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-not-inherited" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-not-inherited/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
   |
   |
LL |     unsafe {static BAR: u64 = FOO;}
   |
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error[E0133]: call to unsafe function `unsafe_call` is unsafe and requires unsafe function or block
   |
LL |             unsafe_call();
   |             ^^^^^^^^^^^^^ call to unsafe function
   |
