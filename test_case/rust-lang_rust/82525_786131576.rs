plain
.................................................................................................... 4800/11497
.................................................................................................... 4900/11497
.................................................................................................... 5000/11497
.................................................................................................... 5100/11497
.........................................................................i...i.......F.........F.... 5200/11497
.................................................................................................... 5400/11497
.................................................................................................... 5500/11497
.................................................................................................... 5600/11497
.................................................................................................... 5700/11497
---
.................................................................................................... 9200/11497
.................................................................................................... 9300/11497
.................................................................................................... 9400/11497
..................................................i......i.......................................... 9500/11497
.........................................................................................iiiiiii..ii 9600/11497
.................................................................................................... 9800/11497
.................................................................................................... 9900/11497
.................................................................................................... 10000/11497
.................................................................................................... 10100/11497
---

---- [ui] ui/binding/issue-53114-safety-checks.rs stdout ----
diff of stderr:

- warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
+ warning: reference to packed field is unaligned
3    |
3    |
4 LL |     let _ = &p.b;
5    |             ^^^^
6    |
6    |
-    = note: `#[warn(safe_packed_borrows)]` on by default
+    = note: `#[warn(unaligned_references)]` on by default
-    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
-    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
-    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
+ warning: reference to packed field is unaligned
+   --> $DIR/issue-53114-safety-checks.rs:39:11
+    |
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
+ 
12 error[E0133]: access to union field is unsafe and requires unsafe function or block
14    |

17    |
17    |
18    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
19 
- warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
-   --> $DIR/issue-53114-safety-checks.rs:29:17
-    |
- LL |     let (_,) = (&p.b,);
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
-    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
-    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
- 
30 error[E0133]: access to union field is unsafe and requires unsafe function or block
32    |

43    |
43    |
44    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
45 
- warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
-   --> $DIR/issue-53114-safety-checks.rs:39:11
-    |
- LL |     match &p.b  { _ => { } }
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
-    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
-    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
- 
56 error[E0133]: access to union field is unsafe and requires unsafe function or block
58    |

68    |           ^^^^^ access to union field
69    |
69    |
70    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
- 
- warning: borrow of packed field is unsafe and requires unsafe function or block (error E0133)
-   --> $DIR/issue-53114-safety-checks.rs:45:12
-    |
- LL |     match (&p.b,)  { (_,) => { } }
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
-    = note: for more information, see issue #46043 <https://github.com/rust-lang/rust/issues/46043>
-    = note: fields of packed structs might be misaligned: dereferencing a misaligned pointer or even just creating a misaligned reference is undefined behavior
81 
82 error[E0133]: access to union field is unsafe and requires unsafe function or block


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks/issue-53114-safety-checks.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks/issue-53114-safety-checks.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binding/issue-53114-safety-checks.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/issue-53114-safety-checks.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:23:13
   |
LL |     let _ = &p.b;  //~ WARN    E0133
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
LL |     let (_,) = (&p.b,);  //~ WARN     E0133
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:39:11
   |
   |
LL |     match &p.b  { _ => { } } //~ WARN     E0133
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:45:12
   |
   |
LL |     match (&p.b,)  { (_,) => { } } //~ WARN     E0133
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     let _ = &u2.a; //~ ERROR  [E0133]
   |             ^^^^^ access to union field
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
   |                 ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

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
   |           ^^^^^ access to union field
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
   |            ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: aborting due to 7 previous errors; 4 warnings emitted

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/issues/issue-27060.rs stdout ----

error: ui test compiled successfully!
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `safe_packed_borrows`
   |
   |
LL | #[deny(safe_packed_borrows)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: reference to packed field is unaligned
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060.rs:17:17
   |
LL |         let _ = &good.data; // ok
   |
   = note: `#[warn(unaligned_references)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060.rs:18:17
   |
   |
LL |         let _ = &good.data2[0]; // ok
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060.rs:21:13
   |
   |
LL |     let _ = &good.data; //~ ERROR borrow of packed field is unsafe
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060.rs:23:13
   |
   |
LL |     let _ = &good.data2[0]; //~ ERROR borrow of packed field is unsafe
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: 5 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-27060-rpass.rs stdout ----
normalized stderr:
warning: unknown lint: `safe_packed_borrows`
   |
   |
LL | #[allow(safe_packed_borrows)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: reference to packed field is unaligned
warning: reference to packed field is unaligned
  --> $DIR/issue-27060-rpass.rs:16:17
   |
LL |         let _ = &good.data; // ok
   |
   = note: `#[warn(unaligned_references)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> $DIR/issue-27060-rpass.rs:17:17
   |
   |
LL |         let _ = &good.data2[0]; // ok
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> $DIR/issue-27060-rpass.rs:20:13
   |
   |
LL |     let _ = &good.data;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> $DIR/issue-27060-rpass.rs:21:13
   |
   |
LL |     let _ = &good.data2[0];
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)

warning: unnecessary `unsafe` block
   |
LL |     unsafe {
LL |     unsafe {
   |     ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

warning: 6 warnings emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060-rpass/issue-27060-rpass.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-27060-rpass.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27060-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27060-rpass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `safe_packed_borrows`
   |
   |
LL | #[allow(safe_packed_borrows)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: reference to packed field is unaligned
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060-rpass.rs:16:17
   |
LL |         let _ = &good.data; // ok
   |
   = note: `#[warn(unaligned_references)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060-rpass.rs:17:17
   |
   |
LL |         let _ = &good.data2[0]; // ok
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060-rpass.rs:20:13
   |
   |
LL |     let _ = &good.data;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
warning: reference to packed field is unaligned
  --> /checkout/src/test/ui/issues/issue-27060-rpass.rs:21:13
   |
   |
LL |     let _ = &good.data2[0];
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)

warning: unnecessary `unsafe` block
   |
LL |     unsafe {
LL |     unsafe {
   |     ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

warning: 6 warnings emitted

---
---- [ui] ui/lint/unaligned_references.rs stdout ----
diff of stderr:

9    |
10 LL | #![deny(unaligned_references)]
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
12    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
14 error: reference to packed field is unaligned


17 LL |         let _ = &good.data;
19    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
20    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
22 error: reference to packed field is unaligned


25 LL |         let _ = &good.data as *const _;
27    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
28    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
30 error: reference to packed field is unaligned


33 LL |         let _: *const _ = &good.data;
35    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
36    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
38 error: reference to packed field is unaligned


41 LL |         let _ = good.data.clone();
43    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
44    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
46 error: reference to packed field is unaligned


49 LL |         let _ = &good.data2[0];
51    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
+    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
52    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
54 error: aborting due to 6 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unaligned_references/unaligned_references.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/unaligned_references.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unaligned_references.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unaligned_references" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unaligned_references/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/lint/unaligned_references.rs:15:17
   |
LL |         let _ = &good.ptr; //~ ERROR reference to packed field
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unaligned_references.rs:1:9
   |
   |
LL | #![deny(unaligned_references)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/lint/unaligned_references.rs:16:17
   |
   |
LL |         let _ = &good.data; //~ ERROR reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/lint/unaligned_references.rs:18:17
   |
   |
LL |         let _ = &good.data as *const _; //~ ERROR reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/lint/unaligned_references.rs:19:27
   |
   |
LL |         let _: *const _ = &good.data; //~ ERROR reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/lint/unaligned_references.rs:21:17
   |
   |
LL |         let _ = good.data.clone(); //~ ERROR reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: reference to packed field is unaligned
  --> /checkout/src/test/ui/lint/unaligned_references.rs:23:17
   |
   |
LL |         let _ = &good.data2[0]; //~ ERROR reference to packed field
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: aborting due to 6 previous errors


------------------------------------------
---
test result: FAILED. 11400 passed; 4 failed; 93 ignored; 0 measured; 0 filtered out; finished in 130.50s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:35
