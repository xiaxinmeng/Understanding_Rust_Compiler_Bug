plain
........................................................................................ 8624/13613
................................i..ii................................................... 8712/13613
...........ii........................................................................... 8800/13613
.................iiii................................................................... 8888/13613
............F.F.......FF...................................i............................ 8976/13613
........................................................................................ 9152/13613
............................................................................i........... 9240/13613
........................................................................................ 9328/13613
....................................................................................i... 9416/13613
---
- error: reference to packed field is unaligned
+ error[E0791]: reference to packed field is unaligned
14   --> $DIR/issue-53114-safety-checks.rs:29:17
15    |
16 LL |     let (_,) = (&p.b,);
17    |                 ^^^^
18    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
---
- error: reference to packed field is unaligned
+ error[E0791]: reference to packed field is unaligned
25   --> $DIR/issue-53114-safety-checks.rs:39:11
26    |
27 LL |     match &p.b  { _ => { } }
28    |           ^^^^
29    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
---
- error: reference to packed field is unaligned
+ error[E0791]: reference to packed field is unaligned
36   --> $DIR/issue-53114-safety-checks.rs:45:12
37    |
38 LL |     match (&p.b,)  { (_,) => { } }
39    |            ^^^^
40    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
---
- Future breakage diagnostic:
- error: reference to packed field is unaligned
-   --> $DIR/issue-53114-safety-checks.rs:29:17
-    |
- LL |     let (_,) = (&p.b,);
-    |
-    = note: `#[deny(unaligned_references)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
-    = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
- 
- Future breakage diagnostic:
- error: reference to packed field is unaligned
-   --> $DIR/issue-53114-safety-checks.rs:39:11
-    |
- LL |     match &p.b  { _ => { } }
-    |
-    = note: `#[deny(unaligned_references)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
-    = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
- 
- Future breakage diagnostic:
- error: reference to packed field is unaligned
-   --> $DIR/issue-53114-safety-checks.rs:45:12
-    |
- LL |     match (&p.b,)  { (_,) => { } }
-    |
-    = note: `#[deny(unaligned_references)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
---
To only update this specific test, also pass `--test-args binding/issue-53114-safety-checks.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/issue-53114-safety-checks.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/issue-53114-safety-checks/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0791]: reference to packed field is unaligned
   |
   |
LL |     let _ = &p.b;  //~ ERROR    reference to packed field
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:29:17
   |
LL |     let (_,) = (&p.b,);  //~ ERROR     reference to packed field
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:39:11
   |
LL |     match &p.b  { _ => { } } //~ ERROR     reference to packed field
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:45:12
   |
LL |     match (&p.b,)  { (_,) => { } } //~ ERROR     reference to packed field
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:26:13
   |
LL |     let _ = &u2.a; //~ ERROR  [E0133]
   |             ^^^^^ access to union field
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:32:17
   |
   |
LL |     let (_,) = (&u2.a,); //~ ERROR   [E0133]
   |                 ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:41:11
   |
   |
LL |     match u1.a  { _ => { } } //~ ERROR   [E0133]
   |           ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error[E0133]: access to union field is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:42:11
   |
   |
LL |     match &u2.a { _ => { } } //~ ERROR   [E0133]
   |           ^^^^^ access to union field
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
  --> /checkout/src/test/ui/binding/issue-53114-safety-checks.rs:48:12
   |
   |
LL |     match (&u2.a,) { (_,) => { } } //~ ERROR   [E0133]
   |            ^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: aborting due to 11 previous errors

Some errors have detailed explanations: E0133, E0791.
For more information about an error, try `rustc --explain E0133`.
---
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/diagnostics/repr_packed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/repr_packed/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0791]: reference to packed field is unaligned
   |
LL |         println!("{}", foo.x);
   |                        ^^^^^
   |
---

---- [ui] src/test/ui/derives/deriving-with-repr-packed.rs stdout ----
diff of stderr:

- error: `Clone` can't be derived on this `#[repr(packed)]` struct with type or const parameters
-   --> $DIR/deriving-with-repr-packed.rs:11:16
-    |
- LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
-    |
- note: the lint level is defined here
- note: the lint level is defined here
+ warning: lint `unaligned_references` has been removed: converted into hard error, see issue #82523 <https://github.com/rust-lang/rust/issues/82523> for more information
9    |
10 LL | #![deny(unaligned_references)]

11    |         ^^^^^^^^^^^^^^^^^^^^
11    |         ^^^^^^^^^^^^^^^^^^^^
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- error: `PartialEq` can't be derived on this `#[repr(packed)]` struct with type or const parameters
-   --> $DIR/deriving-with-repr-packed.rs:11:32
18    |
- LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
25 
25 
- error: `Hash` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
-   --> $DIR/deriving-with-repr-packed.rs:19:19
- LL | #[derive(Default, Hash)]
-    |                   ^^^^
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- error: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
-   --> $DIR/deriving-with-repr-packed.rs:39:10
- LL | #[derive(Debug, Default)]
-    |          ^^^^^
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- error: aborting due to 4 previous errors
- 
- Future incompatibility report: Future breakage diagnostic:
- error: `Clone` can't be derived on this `#[repr(packed)]` struct with type or const parameters
+ error[E0791]: `Clone` can't be derived on this `#[repr(packed)]` struct with type or const parameters
51    |
51    |
52 LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
53    |                ^^^^^
54    |
- note: the lint level is defined here
-   --> $DIR/deriving-with-repr-packed.rs:1:9
-   --> $DIR/deriving-with-repr-packed.rs:1:9
-    |
- LL | #![deny(unaligned_references)]
-    |         ^^^^^^^^^^^^^^^^^^^^
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
62    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
63 
- Future breakage diagnostic:
- error: `PartialEq` can't be derived on this `#[repr(packed)]` struct with type or const parameters
+ error[E0791]: `PartialEq` can't be derived on this `#[repr(packed)]` struct with type or const parameters
67    |
67    |
68 LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
69    |                                ^^^^^^^^^
70    |
- note: the lint level is defined here
-   --> $DIR/deriving-with-repr-packed.rs:1:9
-   --> $DIR/deriving-with-repr-packed.rs:1:9
-    |
- LL | #![deny(unaligned_references)]
-    |         ^^^^^^^^^^^^^^^^^^^^
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
78    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
- Future breakage diagnostic:
- Future breakage diagnostic:
- error: `Hash` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
+ error[E0791]: `Hash` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
83    |
84 LL | #[derive(Default, Hash)]

85    |                   ^^^^
---
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
94    = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)
95 
- Future breakage diagnostic:
- error: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
+ error[E0791]: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
99    |
100 LL | #[derive(Debug, Default)]

101    |          ^^^^^
---
To only update this specific test, also pass `--test-args derives/deriving-with-repr-packed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/deriving-with-repr-packed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-with-repr-packed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-with-repr-packed/auxiliary"
stdout: none
--- stderr -------------------------------
warning: lint `unaligned_references` has been removed: converted into hard error, see issue #82523 <https://github.com/rust-lang/rust/issues/82523> for more information
   |
LL | #![deny(unaligned_references)]
   |         ^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(renamed_and_removed_lints)]` on by default

error[E0791]: `Clone` can't be derived on this `#[repr(packed)]` struct with type or const parameters
   |
   |
LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
   |
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0791]: `PartialEq` can't be derived on this `#[repr(packed)]` struct with type or const parameters
   |
   |
LL | #[derive(Copy, Clone, Default, PartialEq, Eq)]
   |
   |
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0791]: `Hash` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
   |
LL | #[derive(Default, Hash)]
   |                   ^^^^
   |
   |
   = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0791]: `Debug` can't be derived on this `#[repr(packed)]` struct that does not derive `Copy`
   |
LL | #[derive(Debug, Default)]
   |          ^^^^^
   |
---
---- [ui] src/test/ui/packed/issue-27060-rpass.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed/issue-27060-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/issue-27060-rpass/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/issue-27060-rpass/auxiliary"
stdout: none
--- stderr -------------------------------
warning: lint `unaligned_references` has been removed: converted into hard error, see issue #82523 <https://github.com/rust-lang/rust/issues/82523> for more information
   |
LL | #[allow(unaligned_references)]
   |         ^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(renamed_and_removed_lints)]` on by default

error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/issue-27060-rpass.rs:15:13
   |
LL |     let _ = &good.data; // ok
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/issue-27060-rpass.rs:16:13
   |
LL |     let _ = &good.data2[0]; // ok
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/issue-27060-rpass.rs:18:13
   |
LL |     let _ = &good.data;
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/issue-27060-rpass.rs:19:13
   |
LL |     let _ = &good.data2[0];
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)

---
- error: reference to packed field is unaligned
+ error[E0791]: reference to packed field is unaligned
2   --> $DIR/issue-27060.rs:15:13
3    |
4 LL |     let _ = &good.data;
5    |             ^^^^^^^^^^
6    |
-    = note: `#[deny(unaligned_references)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
---
- error: reference to packed field is unaligned
+ error[E0791]: reference to packed field is unaligned
14   --> $DIR/issue-27060.rs:17:13
15    |
16 LL |     let _ = &good.data2[0];
17    |             ^^^^^^^^^^^^^^
18    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
---
- error: reference to packed field is unaligned
+ error[E0791]: reference to packed field is unaligned
25   --> $DIR/issue-27060.rs:20:13
26    |
27 LL |     let _ = &good.data;
28    |             ^^^^^^^^^^
29    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
---
- error: reference to packed field is unaligned
+ error[E0791]: reference to packed field is unaligned
36   --> $DIR/issue-27060.rs:22:13
37    |
38 LL |     let _ = &good.data2[0];
39    |             ^^^^^^^^^^^^^^
40    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
---
- Future incompatibility report: Future breakage diagnostic:
- error: reference to packed field is unaligned
-   --> $DIR/issue-27060.rs:15:13
-    |
- LL |     let _ = &good.data;
-    |
-    = note: `#[deny(unaligned_references)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
-    = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
- 
- Future breakage diagnostic:
- error: reference to packed field is unaligned
-   --> $DIR/issue-27060.rs:17:13
-    |
- LL |     let _ = &good.data2[0];
-    |
-    = note: `#[deny(unaligned_references)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
-    = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
- 
- Future breakage diagnostic:
- error: reference to packed field is unaligned
-   --> $DIR/issue-27060.rs:20:13
-    |
- LL |     let _ = &good.data;
-    |
-    = note: `#[deny(unaligned_references)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
-    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
-    = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
- 
- Future breakage diagnostic:
- error: reference to packed field is unaligned
-   --> $DIR/issue-27060.rs:22:13
-    |
- LL |     let _ = &good.data2[0];
-    |
-    = note: `#[deny(unaligned_references)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
---
To only update this specific test, also pass `--test-args packed/issue-27060.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed/issue-27060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/issue-27060" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/issue-27060/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0791]: reference to packed field is unaligned
   |
   |
LL |     let _ = &good.data; //~ ERROR reference to packed field
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/issue-27060.rs:17:13
   |
LL |     let _ = &good.data2[0]; //~ ERROR reference to packed field
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/issue-27060.rs:20:13
   |
LL |     let _ = &good.data; //~ ERROR reference to packed field
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/issue-27060.rs:22:13
   |
LL |     let _ = &good.data2[0]; //~ ERROR reference to packed field
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)

---
---- [ui] src/test/ui/packed/packed-struct-borrow-element.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed/packed-struct-borrow-element.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-struct-borrow-element/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-struct-borrow-element/auxiliary"
stdout: none
--- stderr -------------------------------
warning: lint `unaligned_references` has been removed: converted into hard error, see issue #82523 <https://github.com/rust-lang/rust/issues/82523> for more information
   |
LL | #[warn(unaligned_references)]
   |        ^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(renamed_and_removed_lints)]` on by default

error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/packed-struct-borrow-element.rs:26:15
   |
LL |     let brw = &foo.baz; //~WARN reference to packed field is unaligned
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)


error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/packed-struct-borrow-element.rs:31:15
   |
LL |     let brw = &foo.baz; //~WARN reference to packed field is unaligned
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)

---
---- [ui] src/test/ui/packed/packed-struct-borrow-element-64bit.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed/packed-struct-borrow-element-64bit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-struct-borrow-element-64bit/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/packed-struct-borrow-element-64bit/auxiliary"
stdout: none
--- stderr -------------------------------
warning: lint `unaligned_references` has been removed: converted into hard error, see issue #82523 <https://github.com/rust-lang/rust/issues/82523> for more information
   |
LL | #[warn(unaligned_references)]
   |        ^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(renamed_and_removed_lints)]` on by default

error[E0791]: reference to packed field is unaligned
  --> /checkout/src/test/ui/packed/packed-struct-borrow-element-64bit.rs:15:15
   |
LL |     let brw = &foo.baz; //~WARN reference to packed field is unaligned
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)

