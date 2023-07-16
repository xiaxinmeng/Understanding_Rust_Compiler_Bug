plain

running 12705 tests
.................................................................................................... 100/12705
............................................iiiiiiiiiiii..............i.i.................i...i..... 200/12705
..............................................FFF.F................................................. 300/12705
.................................................................................................... 500/12705
.................................................................................................... 600/12705
.................................................................................................... 700/12705
.......................i............................................................................ 800/12705
---
.................................................................................................... 2200/12705
...........................................................................F........................ 2300/12705
.......................................................................................F............ 2400/12705
.................................................................................................... 2500/12705
...........................................F......................................F................. 2600/12705
........................................F....................................F.................F.... 2700/12705
.................................................................................................... 2900/12705
.......i............................................................................................ 3000/12705
.................................................................................................... 3100/12705
.........................iiiii...................................................................... 3200/12705
---
.................................................................................................... 5100/12705
.................................................................................F.................. 5200/12705
.................................................................................................... 5300/12705
.................................................................................................... 5400/12705
.................................................................FF..F.............................. 5500/12705
.................................................................................................... 5700/12705
.................................................................................................... 5800/12705
.................................................................................................... 5900/12705
.............................................................................................F...... 6000/12705
---

---- [ui] ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when const-evaluating + checking `IMPL_REF_BAR`
+ error[E0391]: cycle detected when simplifying constant for the type system `IMPL_REF_BAR`
3    |
3    |
4 LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR;
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
- note: ...which requires const-evaluating + checking `<impl at $DIR/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
+ note: ...which requires simplifying constant for the type system `IMPL_REF_BAR`...
+    |
+    |
+ LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires const-evaluating + checking `IMPL_REF_BAR`...
+    |
+    |
+ LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which requires normalizing `<impl at $DIR/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
+ note: ...which requires simplifying constant for the type system `<impl at $DIR/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
9    |
9    |
10 LL |     const BAR: u32 = IMPL_REF_BAR;
11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires caching mir of `<impl at $DIR/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR` for CTFE...
+ note: ...which requires simplifying constant for the type system `<impl at $DIR/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
14    |
14    |
15 LL |     const BAR: u32 = IMPL_REF_BAR;
16    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
16    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires const-evaluating + checking `IMPL_REF_BAR`...
-   --> $DIR/issue-24949-assoc-const-static-recursion-impl.rs:7:1
+ note: ...which requires const-evaluating + checking `<impl at $DIR/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
19    |
19    |
- LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR;
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires const-evaluating + checking `IMPL_REF_BAR`, completing the cycle
- note: cycle used when simplifying constant for the type system `IMPL_REF_BAR`
-   --> $DIR/issue-24949-assoc-const-static-recursion-impl.rs:7:1
+ LL |     const BAR: u32 = IMPL_REF_BAR;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires caching mir of `<impl at $DIR/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR` for CTFE...
25    |
25    |
- LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR;
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     const BAR: u32 = IMPL_REF_BAR;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which requires normalizing `IMPL_REF_BAR`...
+    = note: ...which again requires simplifying constant for the type system `IMPL_REF_BAR`, completing the cycle
+    = note: cycle used when running analysis passes on this crate
29 error: aborting due to previous error
30 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl/issue-24949-assoc-const-static-recursion-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/issue-24949-assoc-const-static-recursion-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `IMPL_REF_BAR`
   |
   |
LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR; //~ ERROR E0391
   |
   |
note: ...which requires simplifying constant for the type system `IMPL_REF_BAR`...
   |
   |
LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `IMPL_REF_BAR`...
   |
   |
LL | const IMPL_REF_BAR: u32 = GlobalImplRef::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
note: ...which requires simplifying constant for the type system `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = IMPL_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires simplifying constant for the type system `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = IMPL_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = IMPL_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires caching mir of `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-impl.rs:11:1: 13:2>::BAR` for CTFE...
   |
   |
LL |     const BAR: u32 = IMPL_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `IMPL_REF_BAR`...
   = note: ...which again requires simplifying constant for the type system `IMPL_REF_BAR`, completing the cycle
   = note: cycle used when running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
------------------------------------------
------------------------------------------


---- [ui] ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when const-evaluating + checking `DEFAULT_REF_BAR`
+ error[E0391]: cycle detected when simplifying constant for the type system `DEFAULT_REF_BAR`
3    |
3    |
4 LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR;
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
- note: ...which requires const-evaluating + checking `FooDefault::BAR`...
+ note: ...which requires simplifying constant for the type system `DEFAULT_REF_BAR`...
+    |
+    |
+ LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires const-evaluating + checking `DEFAULT_REF_BAR`...
+    |
+    |
+ LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which requires normalizing `<GlobalDefaultRef as FooDefault>::BAR`...
+ note: ...which requires simplifying constant for the type system `FooDefault::BAR`...
9    |
9    |
10 LL |     const BAR: u32 = DEFAULT_REF_BAR;
11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires caching mir of `FooDefault::BAR` for CTFE...
+ note: ...which requires simplifying constant for the type system `FooDefault::BAR`...
14    |
14    |
15 LL |     const BAR: u32 = DEFAULT_REF_BAR;
16    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
16    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires const-evaluating + checking `DEFAULT_REF_BAR`...
-   --> $DIR/issue-24949-assoc-const-static-recursion-trait-default.rs:11:1
+ note: ...which requires const-evaluating + checking `FooDefault::BAR`...
19    |
19    |
- LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR;
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires const-evaluating + checking `DEFAULT_REF_BAR`, completing the cycle
- note: cycle used when simplifying constant for the type system `DEFAULT_REF_BAR`
-   --> $DIR/issue-24949-assoc-const-static-recursion-trait-default.rs:11:1
+ LL |     const BAR: u32 = DEFAULT_REF_BAR;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires caching mir of `FooDefault::BAR` for CTFE...
25    |
25    |
- LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR;
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     const BAR: u32 = DEFAULT_REF_BAR;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which requires normalizing `DEFAULT_REF_BAR`...
+    = note: ...which again requires simplifying constant for the type system `DEFAULT_REF_BAR`, completing the cycle
+    = note: cycle used when running analysis passes on this crate
29 error: aborting due to previous error
30 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default/issue-24949-assoc-const-static-recursion-trait-default.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/issue-24949-assoc-const-static-recursion-trait-default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait-default/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `DEFAULT_REF_BAR`
   |
   |
LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR; //~ ERROR E0391
   |
   |
note: ...which requires simplifying constant for the type system `DEFAULT_REF_BAR`...
   |
   |
LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR; //~ ERROR E0391
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `DEFAULT_REF_BAR`...
   |
   |
LL | const DEFAULT_REF_BAR: u32 = <GlobalDefaultRef>::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `<GlobalDefaultRef as FooDefault>::BAR`...
note: ...which requires simplifying constant for the type system `FooDefault::BAR`...
   |
   |
LL |     const BAR: u32 = DEFAULT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires simplifying constant for the type system `FooDefault::BAR`...
   |
   |
LL |     const BAR: u32 = DEFAULT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `FooDefault::BAR`...
   |
   |
LL |     const BAR: u32 = DEFAULT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires caching mir of `FooDefault::BAR` for CTFE...
   |
   |
LL |     const BAR: u32 = DEFAULT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `DEFAULT_REF_BAR`...
   = note: ...which again requires simplifying constant for the type system `DEFAULT_REF_BAR`, completing the cycle
   = note: cycle used when running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
------------------------------------------
------------------------------------------


---- [ui] ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when const-evaluating + checking `TRAIT_REF_BAR`
+ error[E0391]: cycle detected when simplifying constant for the type system `TRAIT_REF_BAR`
3    |
3    |
4 LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR;
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
- note: ...which requires const-evaluating + checking `<impl at $DIR/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
+ note: ...which requires simplifying constant for the type system `TRAIT_REF_BAR`...
+    |
+    |
+ LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires const-evaluating + checking `TRAIT_REF_BAR`...
+    |
+    |
+ LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which requires normalizing `<GlobalTraitRef as Foo>::BAR`...
+ note: ...which requires simplifying constant for the type system `<impl at $DIR/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
9    |
9    |
10 LL |     const BAR: u32 = TRAIT_REF_BAR;
11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires caching mir of `<impl at $DIR/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR` for CTFE...
+ note: ...which requires simplifying constant for the type system `<impl at $DIR/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
14    |
14    |
15 LL |     const BAR: u32 = TRAIT_REF_BAR;
16    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
16    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires const-evaluating + checking `TRAIT_REF_BAR`...
-   --> $DIR/issue-24949-assoc-const-static-recursion-trait.rs:7:1
+ note: ...which requires const-evaluating + checking `<impl at $DIR/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
19    |
19    |
- LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR;
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires const-evaluating + checking `TRAIT_REF_BAR`, completing the cycle
- note: cycle used when simplifying constant for the type system `TRAIT_REF_BAR`
-   --> $DIR/issue-24949-assoc-const-static-recursion-trait.rs:7:1
+ LL |     const BAR: u32 = TRAIT_REF_BAR;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires caching mir of `<impl at $DIR/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR` for CTFE...
25    |
25    |
- LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR;
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     const BAR: u32 = TRAIT_REF_BAR;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which requires normalizing `TRAIT_REF_BAR`...
+    = note: ...which again requires simplifying constant for the type system `TRAIT_REF_BAR`, completing the cycle
+    = note: cycle used when running analysis passes on this crate
29 error: aborting due to previous error
30 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait/issue-24949-assoc-const-static-recursion-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/issue-24949-assoc-const-static-recursion-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `TRAIT_REF_BAR`
   |
   |
LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR; //~ ERROR E0391
   |
   |
note: ...which requires simplifying constant for the type system `TRAIT_REF_BAR`...
   |
   |
LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `TRAIT_REF_BAR`...
   |
   |
LL | const TRAIT_REF_BAR: u32 = <GlobalTraitRef>::BAR; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `<GlobalTraitRef as Foo>::BAR`...
note: ...which requires simplifying constant for the type system `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = TRAIT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires simplifying constant for the type system `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = TRAIT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR`...
   |
   |
LL |     const BAR: u32 = TRAIT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires caching mir of `<impl at /checkout/src/test/ui/associated-consts/issue-24949-assoc-const-static-recursion-trait.rs:11:1: 13:2>::BAR` for CTFE...
   |
   |
LL |     const BAR: u32 = TRAIT_REF_BAR;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `TRAIT_REF_BAR`...
   = note: ...which again requires simplifying constant for the type system `TRAIT_REF_BAR`, completing the cycle
   = note: cycle used when running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
------------------------------------------
------------------------------------------


---- [ui] ui/associated-consts/defaults-cyclic-fail.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when const-evaluating + checking `Tr::A`
+ error[E0391]: cycle detected when normalizing `<() as Tr>::A`
+    |
+ note: ...which requires simplifying constant for the type system `Tr::A`...
3    |
3    |
4 LL |     const A: u8 = Self::B;
5    |     ^^^^^^^^^^^^^^^^^^^^^^
5    |     ^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires simplifying constant for the type system `Tr::A`...
6    |
6    |
+ LL |     const A: u8 = Self::B;
+    |     ^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires const-evaluating + checking `Tr::A`...
+    |
+    |
+ LL |     const A: u8 = Self::B;
+    |     ^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which requires normalizing `<() as Tr>::B`...
+ note: ...which requires simplifying constant for the type system `Tr::B`...
+    |
+    |
+ LL |     const B: u8 = Self::A;
+    |     ^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires simplifying constant for the type system `Tr::B`...
+    |
+    |
+ LL |     const B: u8 = Self::A;
+    |     ^^^^^^^^^^^^^^^^^^^^^^
7 note: ...which requires const-evaluating + checking `Tr::B`...
9    |


10 LL |     const B: u8 = Self::A;
11    |     ^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires const-evaluating + checking `Tr::A`, completing the cycle
+    = note: ...which again requires normalizing `<() as Tr>::A`, completing the cycle
13 note: cycle used when const-evaluating + checking `main::promoted[1]`
15    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-cyclic-fail/defaults-cyclic-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/defaults-cyclic-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/defaults-cyclic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-cyclic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-cyclic-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when normalizing `<() as Tr>::A`
   |
note: ...which requires simplifying constant for the type system `Tr::A`...
   |
   |
LL |     const A: u8 = Self::B;
   |     ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires simplifying constant for the type system `Tr::A`...
   |
   |
LL |     const A: u8 = Self::B;
   |     ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Tr::A`...
   |
   |
LL |     const A: u8 = Self::B;
   |     ^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `<() as Tr>::B`...
note: ...which requires simplifying constant for the type system `Tr::B`...
   |
   |
LL |     const B: u8 = Self::A;
   |     ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires simplifying constant for the type system `Tr::B`...
   |
   |
LL |     const B: u8 = Self::A;
   |     ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Tr::B`...
   |
   |
LL |     const B: u8 = Self::A;
   |     ^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires normalizing `<() as Tr>::A`, completing the cycle
note: cycle used when const-evaluating + checking `main::promoted[1]`
   |
LL | fn main() {
   | ^^^^^^^^^

---

---- [ui] ui/borrowck/issue-88434-minimal-example.rs stdout ----
diff of stderr:

1 error[E0080]: evaluation of constant value failed
- <<<<<<< HEAD
-   --> $DIR/issue-88434-minimal-example.rs:9:5
- =======
-   --> $DIR/issue-88434-minimal-example.rs:12:5
- >>>>>>> a4feb9af018 (bless tests)
7    |
7    |
8 LL | const _CONST: &() = &f(&|_| {});
9    |                      ---------- inside `_CONST` at $DIR/issue-88434-minimal-example.rs:3:22
11 LL |     panic!()
12    |     ^^^^^^^^
13    |     |
13    |     |
- <<<<<<< HEAD
+    |     the evaluated program panicked at 'explicit panic', $DIR/issue-88434-minimal-example.rs:11:5
+    |     the evaluated program panicked at 'explicit panic', $DIR/issue-88434-minimal-example.rs:11:5
16    |     inside `f::<[closure@$DIR/issue-88434-minimal-example.rs:3:25: 3:31]>` at $SRC_DIR/std/src/panic.rs:LL:COL
- =======
-    |     the evaluated program panicked at 'explicit panic', $DIR/issue-88434-minimal-example.rs:12:5
-    |     inside `f::<[closure@$DIR/issue-88434-minimal-example.rs:4:25: 4:31]>` at $SRC_DIR/std/src/panic.rs:LL:COL
- >>>>>>> a4feb9af018 (bless tests)
22    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
23 

- error: any use of this value will cause an error
- error: any use of this value will cause an error
-   --> $DIR/issue-88434-minimal-example.rs:4:21
-    |
- LL | const _CONST: &() = &f(&|_| {});
-    |                     |
-    |                     referenced constant has errors
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
- 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
---
To only update this specific test, also pass `--test-args borrowck/issue-88434-minimal-example.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-minimal-example" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-minimal-example/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL | const _CONST: &() = &f(&|_| {});
   |                      ---------- inside `_CONST` at /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:3:22
...
LL |     panic!() //~ ERROR evaluation of constant value failed
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:11:5
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:11:5
   |     inside `f::<[closure@/checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:3:25: 3:31]>` at /checkout/library/std/src/panic.rs:19:9
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/issue-88434-removal-index-should-be-less.rs:12:5
+   --> $DIR/issue-88434-removal-index-should-be-less.rs:11:5
3    |
4 LL | const _CONST: &[u8] = &f(&[], |_| {});
5    |                        -------------- inside `_CONST` at $DIR/issue-88434-removal-index-should-be-less.rs:3:24
7 LL |     panic!()
8    |     ^^^^^^^^
9    |     |
-    |     the evaluated program panicked at 'explicit panic', $DIR/issue-88434-removal-index-should-be-less.rs:12:5
-    |     the evaluated program panicked at 'explicit panic', $DIR/issue-88434-removal-index-should-be-less.rs:12:5
-    |     inside `f::<[closure@$DIR/issue-88434-removal-index-should-be-less.rs:4:31: 4:37]>` at $SRC_DIR/std/src/panic.rs:LL:COL
+    |     the evaluated program panicked at 'explicit panic', $DIR/issue-88434-removal-index-should-be-less.rs:11:5
+    |     inside `f::<[closure@$DIR/issue-88434-removal-index-should-be-less.rs:3:31: 3:37]>` at $SRC_DIR/std/src/panic.rs:LL:COL
13    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
14 

- error: any use of this value will cause an error
- error: any use of this value will cause an error
-   --> $DIR/issue-88434-removal-index-should-be-less.rs:4:23
-    |
- LL | const _CONST: &[u8] = &f(&[], |_| {});
-    |                       |
-    |                       referenced constant has errors
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
- 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
---
To only update this specific test, also pass `--test-args borrowck/issue-88434-removal-index-should-be-less.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-removal-index-should-be-less" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-removal-index-should-be-less/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});
   |                        -------------- inside `_CONST` at /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:3:24
...
LL |     panic!() //~ ERROR evaluation of constant value failed
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:11:5
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:11:5
   |     inside `f::<[closure@/checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:3:31: 3:37]>` at /checkout/library/std/src/panic.rs:19:9
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---

4 LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
5    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |         |
-    |         memory access failed: alloc5 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
+    |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
8    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
10   ::: $DIR/out_of_bounds_read.rs:13:33

18 LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
19    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
19    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
20    |         |
-    |         memory access failed: alloc5 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
+    |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
22    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
24   ::: $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

37 LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
38    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
38    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
39    |         |
-    |         memory access failed: alloc5 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
+    |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
41    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
43   ::: $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/out_of_bounds_read.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-ptr/out_of_bounds_read.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-ptr/out_of_bounds_read.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         |
   |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:660:9
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:33
   |
   |
LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
   |                                 ----------------------- inside `_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:33
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mod.rs:660:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:660:9
  ::: /checkout/library/core/src/ptr/const_ptr.rs:822:18
   |
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::const_ptr::<impl *const u32>::read` at /checkout/library/core/src/ptr/const_ptr.rs:822:18
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:39
   |
   |
LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };
   |                                       ------------------- inside `_CONST_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:39
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/mod.rs:660:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         memory access failed: alloc7 has size 4, so pointer to 4 bytes starting at offset 4 is out-of-bounds
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:660:9
  ::: /checkout/library/core/src/ptr/mut_ptr.rs:936:18
   |
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::mut_ptr::<impl *mut u32>::read` at /checkout/library/core/src/ptr/mut_ptr.rs:936:18
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:15:37
   |
   |
LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };
   |                                     --------------------------------- inside `_MUT_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:15:37
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] ui/consts/const-eval/const-eval-query-stack.rs stdout ----
diff of stderr:

19    |
20 LL |     let x: &'static i32 = &X;
21    |                            ^ referenced constant has errors
- error: erroneous constant used
-   --> $DIR/const-eval-query-stack.rs:23:27
-    |
-    |
- LL |     let x: &'static i32 = &X;
-    |                           ^^ referenced constant has errors
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
- 
33 query stack during panic:
- #0 [optimized_mir] optimizing MIR for `main`
- #0 [optimized_mir] optimizing MIR for `main`
- #1 [collect_and_partition_mono_items] collect_and_partition_mono_items
+ #0 [try_normalize_mir_const_after_erasing_regions] normalizing `main::promoted[1]`
+ #1 [optimized_mir] optimizing MIR for `main`
+ #2 [collect_and_partition_mono_items] collect_and_partition_mono_items
37 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/const-eval-query-stack.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-query-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:19:16
   |
   |
LL | const X: i32 = 1 / 0; //~WARN any use of this value will cause an error
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:18:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:23:28
   |
LL |     let x: &'static i32 = &X;
   |                            ^ referenced constant has errors
error: internal compiler error: ty::ConstKind::Error constructed but no error reported


thread 'rustc' panicked at 'aborting after 2 errors due to `-Z treat-err-as-bug=2`', compiler/rustc_errors/src/lib.rs:1292:36
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   1: core::panicking::panic_fmt
   2: <rustc_errors::HandlerInner>::panic_if_treat_err_as_bug
   4: <rustc_errors::HandlerInner>::emit_diag_at_span::<rustc_span::span_encoding::Span>
   5: <rustc_errors::HandlerInner>::span_bug::<rustc_span::span_encoding::Span>
   6: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span>
   7: <rustc_middle::ty::context::TyCtxt>::const_error
   7: <rustc_middle::ty::context::TyCtxt>::const_error
   8: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_const
   9: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  10: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize::<rustc_middle::mir::ConstantKind>
  11: <rustc_infer::infer::InferCtxtBuilder>::enter::<core::result::Result<rustc_middle::mir::ConstantKind, rustc_middle::traits::query::NoSolution>, rustc_traits::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle::mir::ConstantKind>::{closure#0}>
  12: <rustc_traits::normalize_erasing_regions::provide::{closure#1} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::ConstantKind>)>>::call_once
  13: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl::plumbing::QueryCtxt>
  14: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  15: <rustc_middle::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  16: <rustc_middle::ty::context::TyCtxt>::try_normalize_erasing_regions::<rustc_middle::mir::ConstantKind>
  17: <rustc_middle::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle::mir::ConstantKind>
  18: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_mir_transform::const_prop::ConstPropMachine>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle::mir::ConstantKind>
  19: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_mir_transform::const_prop::ConstPropMachine>>::eval_operand
  20: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_mir_transform::const_prop::ConstPropMachine>>::eval_rvalue_into_place
  21: <rustc_mir_transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_statement
  22: <rustc_mir_transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_body
  24: rustc_mir_transform::pass_manager::run_passes
  25: rustc_mir_transform::optimized_mir
  26: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::mir::Body>>
  27: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
  27: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
  28: <rustc_middle::ty::context::TyCtxt>::instance_mir
  29: rustc_monomorphize::collector::collect_neighbours
  30: rustc_monomorphize::collector::collect_items_rec
  31: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
  32: rustc_monomorphize::collector::collect_crate_mono_items
  33: rustc_monomorphize::partitioning::collect_and_partition_mono_items
  34: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>>
  35: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
  36: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  37: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  38: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  39: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorGuaranteed>>
  40: <rustc_interface::queries::Queries>::ongoing_codegen
  41: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (f5bcc034b 2022-03-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=2
query stack during panic:
query stack during panic:
#0 [try_normalize_mir_const_after_erasing_regions] normalizing `main::promoted[1]`
#1 [optimized_mir] optimizing MIR for `main`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] ui/consts/const-mut-refs/issue-76510.rs stdout ----
---- [ui] ui/consts/const-mut-refs/issue-76510.rs stdout ----
diff of 64bit.stderr:

19 LL | const S: &'static mut str = &mut " hello ";
20    |                             ^^^^^^^^^^^^^^ cannot borrow as mutable
- error[E0080]: evaluation of constant value failed
-   --> $DIR/issue-76510.rs:11:70
-    |
-    |
- LL |         let s = transmute::<(*const u8, usize), &ManuallyDrop<str>>((S.as_ptr(), 3));
-    |                                                                      ^ referenced constant has errors
+ error: aborting due to 3 previous errors
- error: aborting due to 4 previous errors
- 
- Some errors have detailed explanations: E0080, E0596, E0658, E0764.
- For more information about an error, try `rustc --explain E0080`.
- For more information about an error, try `rustc --explain E0080`.
+ Some errors have detailed explanations: E0596, E0658, E0764.
+ For more information about an error, try `rustc --explain E0596`.
32 


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/issue-76510/issue-76510.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-mut-refs/issue-76510.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-mut-refs/issue-76510.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/issue-76510" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/issue-76510/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: mutable references are not allowed in the final value of constants
   |
   |
LL | const S: &'static mut str = &mut " hello ";


error[E0658]: mutation through a reference is not allowed in constants
   |
   |
LL | const S: &'static mut str = &mut " hello ";
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0596]: cannot borrow data in a `&` reference as mutable
   |
   |
LL | const S: &'static mut str = &mut " hello ";
   |                             ^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0596, E0658, E0764.
For more information about an error, try `rustc --explain E0596`.
For more information about an error, try `rustc --explain E0596`.
------------------------------------------


---- [ui] ui/consts/const-tup-index-span.rs stdout ----
diff of stderr:

11 LL | const TUP: (usize,) = (5usize << 64,);
13 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-tup-index-span.rs:6:18
-    |
-    |
- LL | const ARR: [i32; TUP.0] = [];
-    |                  ^^^ referenced constant has errors
+ error: aborting due to previous error
- error: aborting due to 2 previous errors
- 
- Some errors have detailed explanations: E0080, E0308.
- For more information about an error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/const-tup-index-span.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-tup-index-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-tup-index-span" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-tup-index-span/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-tup-index-span.rs:3:23
   |
   |
LL | const TUP: (usize,) = 5usize << 64;
   |
   |
   = note: expected tuple `(usize,)`
               found type `usize`
help: use a trailing comma to create a tuple with one element
   |
LL | const TUP: (usize,) = (5usize << 64,);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
18 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-integer-bool-ops.rs:8:18
-    |
- LL | const ARR: [i32; X] = [99; 34];
-    |                  ^ referenced constant has errors
25 error[E0308]: mismatched types
26   --> $DIR/const-integer-bool-ops.rs:11:19
27    |


40 LL | const X1: usize = 42 || 39;
41    |                   ^^^^^^^^ expected `usize`, found `bool`
42 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-integer-bool-ops.rs:18:19
-    |
- LL | const ARR1: [i32; X1] = [99; 47];
-    |                   ^^ referenced constant has errors
49 error[E0308]: mismatched types
50   --> $DIR/const-integer-bool-ops.rs:21:19
51    |


64 LL | const X2: usize = -42 || -39;
65    |                   ^^^^^^^^^^ expected `usize`, found `bool`
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-integer-bool-ops.rs:28:19
-    |
-    |
- LL | const ARR2: [i32; X2] = [99; 18446744073709551607];
-    |                   ^^ referenced constant has errors
73 error[E0308]: mismatched types
74   --> $DIR/const-integer-bool-ops.rs:31:19
75    |


88 LL | const X3: usize = -42 && -39;
89    |                   ^^^^^^^^^^ expected `usize`, found `bool`
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-integer-bool-ops.rs:38:19
-    |
-    |
- LL | const ARR3: [i32; X3] = [99; 6];
-    |                   ^^ referenced constant has errors
97 error[E0308]: mismatched types
98   --> $DIR/const-integer-bool-ops.rs:41:18
99    |


100 LL | const Y: usize = 42.0 == 42.0;
101    |                  ^^^^^^^^^^^^ expected `usize`, found `bool`
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-integer-bool-ops.rs:44:19
-    |
-    |
- LL | const ARRR: [i32; Y] = [99; 1];
-    |                   ^ referenced constant has errors
109 error[E0308]: mismatched types
110   --> $DIR/const-integer-bool-ops.rs:47:19
111    |


112 LL | const Y1: usize = 42.0 >= 42.0;
113    |                   ^^^^^^^^^^^^ expected `usize`, found `bool`
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-integer-bool-ops.rs:50:20
-    |
-    |
- LL | const ARRR1: [i32; Y1] = [99; 1];
-    |                    ^^ referenced constant has errors
121 error[E0308]: mismatched types
122   --> $DIR/const-integer-bool-ops.rs:53:19
123    |


124 LL | const Y2: usize = 42.0 <= 42.0;
125    |                   ^^^^^^^^^^^^ expected `usize`, found `bool`
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-integer-bool-ops.rs:56:20
-    |
-    |
- LL | const ARRR2: [i32; Y2] = [99; 1];
-    |                    ^^ referenced constant has errors
133 error[E0308]: mismatched types
134   --> $DIR/const-integer-bool-ops.rs:59:19
135    |


136 LL | const Y3: usize = 42.0 > 42.0;
137    |                   ^^^^^^^^^^^ expected `usize`, found `bool`
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-integer-bool-ops.rs:62:20
-    |
-    |
- LL | const ARRR3: [i32; Y3] = [99; 0];
-    |                    ^^ referenced constant has errors
145 error[E0308]: mismatched types
146   --> $DIR/const-integer-bool-ops.rs:65:19
147    |


148 LL | const Y4: usize = 42.0 < 42.0;
149    |                   ^^^^^^^^^^^ expected `usize`, found `bool`
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-integer-bool-ops.rs:68:20
-    |
-    |
- LL | const ARRR4: [i32; Y4] = [99; 0];
-    |                    ^^ referenced constant has errors
157 error[E0308]: mismatched types
158   --> $DIR/const-integer-bool-ops.rs:71:19
159    |


160 LL | const Y5: usize = 42.0 != 42.0;
161    |                   ^^^^^^^^^^^^ expected `usize`, found `bool`
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-integer-bool-ops.rs:74:20
-    |
-    |
- LL | const ARRR5: [i32; Y5] = [99; 0];
-    |                    ^^ referenced constant has errors
+ error: aborting due to 18 previous errors
- error: aborting due to 28 previous errors
- 
- Some errors have detailed explanations: E0080, E0308.
- For more information about an error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/const-integer-bool-ops.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-integer-bool-ops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-integer-bool-ops" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-integer-bool-ops/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:1:18
   |
LL | const X: usize = 42 && 39;
   |                  ^^ expected `bool`, found integer
---

error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:21:19
   |
LL | const X2: usize = -42 || -39;
   |                   ^^^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:21:26
   |
   |
LL | const X2: usize = -42 || -39;
   |                          ^^^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:21:19
   |
   |
LL | const X2: usize = -42 || -39;
   |                   ^^^^^^^^^^ expected `usize`, found `bool`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:31:19
   |
   |
LL | const X3: usize = -42 && -39;
   |                   ^^^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:31:26
   |
   |
LL | const X3: usize = -42 && -39;
   |                          ^^^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:31:19
   |
   |
LL | const X3: usize = -42 && -39;
   |                   ^^^^^^^^^^ expected `usize`, found `bool`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:41:18
   |
   |
LL | const Y: usize = 42.0 == 42.0;
   |                  ^^^^^^^^^^^^ expected `usize`, found `bool`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:47:19
   |
   |
LL | const Y1: usize = 42.0 >= 42.0;
   |                   ^^^^^^^^^^^^ expected `usize`, found `bool`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:53:19
   |
   |
LL | const Y2: usize = 42.0 <= 42.0;
   |                   ^^^^^^^^^^^^ expected `usize`, found `bool`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:59:19
   |
   |
LL | const Y3: usize = 42.0 > 42.0;
   |                   ^^^^^^^^^^^ expected `usize`, found `bool`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:65:19
   |
   |
LL | const Y4: usize = 42.0 < 42.0;
   |                   ^^^^^^^^^^^ expected `usize`, found `bool`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-integer-bool-ops.rs:71:19
   |
   |
LL | const Y5: usize = 42.0 != 42.0;
   |                   ^^^^^^^^^^^^ expected `usize`, found `bool`
error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] ui/consts/invalid-union.rs stdout ----
diff of 64bit.stderr:

6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
8    = note: the raw bytes of the constant (size: 8, align: 8) {
+                ╾───────alloc7────────╼                         │ ╾──────╼
10            }
11 
12 error: erroneous constant used
12 error: erroneous constant used


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/invalid-union.64bit.stderr
To only update this specific test, also pass `--test-args consts/invalid-union.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/invalid-union.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | fn main() { //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^ type validation failed at .<deref>.y.<enum-variant(B)>.0: encountered `UnsafeCell` in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:42:25
  --> /checkout/src/test/ui/consts/invalid-union.rs:42:25
   |
LL |     let _: &'static _ = &C; //~ ERROR erroneous constant used
   |                         ^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] ui/consts/issue-36163.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when const-evaluating + checking `Foo::B::{constant#0}`
+ error[E0391]: cycle detected when simplifying constant for the type system `Foo::B::{constant#0}`
3    |
4 LL |     B = A,

5    |         ^
5    |         ^
6    |
+ note: ...which requires simplifying constant for the type system `Foo::B::{constant#0}`...
+    |
+ LL |     B = A,
+    |         ^
+    |         ^
+ note: ...which requires const-evaluating + checking `Foo::B::{constant#0}`...
+    |
+ LL |     B = A,
+    |         ^
+    |         ^
+    = note: ...which requires normalizing `A`...
+ note: ...which requires simplifying constant for the type system `A`...
+    |
+    |
+ LL | const A: isize = Foo::B as isize;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires simplifying constant for the type system `A`...
+    |
+    |
+ LL | const A: isize = Foo::B as isize;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
7 note: ...which requires const-evaluating + checking `A`...
9    |


10 LL | const A: isize = Foo::B as isize;
11    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires const-evaluating + checking `Foo::B::{constant#0}`, completing the cycle
- note: cycle used when simplifying constant for the type system `Foo::B::{constant#0}`
-   --> $DIR/issue-36163.rs:4:9
+    = note: ...which requires normalizing `A`...
+    = note: ...which again requires simplifying constant for the type system `Foo::B::{constant#0}`, completing the cycle
+ note: cycle used when collecting item types in top-level module
15    |
- LL |     B = A,
-    |         ^
-    |         ^
+ LL | / const A: isize = Foo::B as isize;
+ LL | |
+ LL | | enum Foo {
+ LL | |     B = A,
+ LL | | }
+ LL | |
+ LL | | fn main() {}
18 
19 error: aborting due to previous error
20 

---
To only update this specific test, also pass `--test-args consts/issue-36163.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-36163.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-36163" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-36163/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `Foo::B::{constant#0}`
   |
   |
LL |     B = A, //~ ERROR E0391
   |
   |
note: ...which requires simplifying constant for the type system `Foo::B::{constant#0}`...
   |
   |
LL |     B = A, //~ ERROR E0391
   |         ^
note: ...which requires const-evaluating + checking `Foo::B::{constant#0}`...
   |
   |
LL |     B = A, //~ ERROR E0391
   |         ^
   = note: ...which requires normalizing `A`...
note: ...which requires simplifying constant for the type system `A`...
   |
   |
LL | const A: isize = Foo::B as isize;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires simplifying constant for the type system `A`...
   |
   |
LL | const A: isize = Foo::B as isize;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `A`...
   |
   |
LL | const A: isize = Foo::B as isize;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `A`...
   = note: ...which again requires simplifying constant for the type system `Foo::B::{constant#0}`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
   |
LL | / const A: isize = Foo::B as isize;
LL | |
LL | | enum Foo {
LL | |     B = A, //~ ERROR E0391
LL | | }
LL | |
LL | | fn main() {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] ui/issues/issue-17252.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when const-evaluating + checking `FOO`
+ error[E0391]: cycle detected when normalizing `FOO`
+    |
+ note: ...which requires simplifying constant for the type system `FOO`...
3    |
4 LL | const FOO: usize = FOO;

5    | ^^^^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires simplifying constant for the type system `FOO`...
6    |
6    |
-    = note: ...which immediately requires const-evaluating + checking `FOO` again
+ LL | const FOO: usize = FOO;
+    | ^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires const-evaluating + checking `FOO`...
+    |
+ LL | const FOO: usize = FOO;
+    | ^^^^^^^^^^^^^^^^^^^^^^^
+    | ^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which again requires normalizing `FOO`, completing the cycle
8 note: cycle used when const-evaluating + checking `main::{constant#0}`
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17252/issue-17252.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17252.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17252.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17252" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17252/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when normalizing `FOO`
   |
note: ...which requires simplifying constant for the type system `FOO`...
   |
   |
LL | const FOO: usize = FOO; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires simplifying constant for the type system `FOO`...
   |
   |
LL | const FOO: usize = FOO; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `FOO`...
   |
   |
LL | const FOO: usize = FOO; //~ ERROR E0391
   | ^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires normalizing `FOO`, completing the cycle
note: cycle used when const-evaluating + checking `main::{constant#0}`
   |
   |
LL |     let _x: [u8; FOO]; // caused stack overflow prior to fix

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] ui/issues/issue-23302-1.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when const-evaluating + checking `X::A::{constant#0}`
+ error[E0391]: cycle detected when simplifying constant for the type system `X::A::{constant#0}`
3    |
3    |
4 LL |     A = X::A as isize,
5    |         ^^^^^^^^^^^^^
6    |
6    |
-    = note: ...which immediately requires const-evaluating + checking `X::A::{constant#0}` again
- note: cycle used when simplifying constant for the type system `X::A::{constant#0}`
+ note: ...which requires simplifying constant for the type system `X::A::{constant#0}`...
10    |
10    |
11 LL |     A = X::A as isize,
12    |         ^^^^^^^^^^^^^
12    |         ^^^^^^^^^^^^^
+ note: ...which requires const-evaluating + checking `X::A::{constant#0}`...
+    |
+    |
+ LL |     A = X::A as isize,
+    |         ^^^^^^^^^^^^^
+    = note: ...which requires normalizing `X::A as isize`...
+    = note: ...which again requires simplifying constant for the type system `X::A::{constant#0}`, completing the cycle
+ note: cycle used when collecting item types in top-level module
+    |
+    |
+ LL | enum X {
13 
14 error: aborting due to previous error
15 

---
To only update this specific test, also pass `--test-args issues/issue-23302-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `X::A::{constant#0}`
   |
   |
LL |     A = X::A as isize, //~ ERROR E0391
   |
   |
note: ...which requires simplifying constant for the type system `X::A::{constant#0}`...
   |
   |
LL |     A = X::A as isize, //~ ERROR E0391
   |         ^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `X::A::{constant#0}`...
   |
   |
LL |     A = X::A as isize, //~ ERROR E0391
   |         ^^^^^^^^^^^^^
   = note: ...which requires normalizing `X::A as isize`...
   = note: ...which again requires simplifying constant for the type system `X::A::{constant#0}`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
   |
LL | enum X {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] ui/issues/issue-23302-2.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when const-evaluating + checking `Y::A::{constant#0}`
+ error[E0391]: cycle detected when simplifying constant for the type system `Y::A::{constant#0}`
3    |
3    |
4 LL |     A = Y::B as isize,
5    |         ^^^^^^^^^^^^^
6    |
6    |
-    = note: ...which immediately requires const-evaluating + checking `Y::A::{constant#0}` again
- note: cycle used when simplifying constant for the type system `Y::A::{constant#0}`
+ note: ...which requires simplifying constant for the type system `Y::A::{constant#0}`...
10    |
10    |
11 LL |     A = Y::B as isize,
12    |         ^^^^^^^^^^^^^
12    |         ^^^^^^^^^^^^^
+ note: ...which requires const-evaluating + checking `Y::A::{constant#0}`...
+    |
+    |
+ LL |     A = Y::B as isize,
+    |         ^^^^^^^^^^^^^
+    = note: ...which requires normalizing `Y::B as isize`...
+    = note: ...which again requires simplifying constant for the type system `Y::A::{constant#0}`, completing the cycle
+ note: cycle used when collecting item types in top-level module
+    |
+    |
+ LL | enum Y {
13 
14 error: aborting due to previous error
15 

---
To only update this specific test, also pass `--test-args issues/issue-23302-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `Y::A::{constant#0}`
   |
   |
LL |     A = Y::B as isize, //~ ERROR E0391
   |
   |
note: ...which requires simplifying constant for the type system `Y::A::{constant#0}`...
   |
   |
LL |     A = Y::B as isize, //~ ERROR E0391
   |         ^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Y::A::{constant#0}`...
   |
   |
LL |     A = Y::B as isize, //~ ERROR E0391
   |         ^^^^^^^^^^^^^
   = note: ...which requires normalizing `Y::B as isize`...
   = note: ...which again requires simplifying constant for the type system `Y::A::{constant#0}`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
   |
LL | enum Y {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] ui/issues/issue-23302-3.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when const-evaluating + checking `A`
+ error[E0391]: cycle detected when simplifying constant for the type system `A`
3    |
3    |
4 LL | const A: i32 = B;
5    | ^^^^^^^^^^^^^^^^^
6    |
6    |
- note: ...which requires const-evaluating + checking `B`...
-   --> $DIR/issue-23302-3.rs:3:1
+ note: ...which requires simplifying constant for the type system `A`...
9    |
9    |
- LL | const B: i32 = A;
+ LL | const A: i32 = B;
11    | ^^^^^^^^^^^^^^^^^
-    = note: ...which again requires const-evaluating + checking `A`, completing the cycle
- note: cycle used when simplifying constant for the type system `A`
+ note: ...which requires const-evaluating + checking `A`...
15    |
15    |
16 LL | const A: i32 = B;
17    | ^^^^^^^^^^^^^^^^^
17    | ^^^^^^^^^^^^^^^^^
+    = note: ...which requires normalizing `B`...
+ note: ...which requires simplifying constant for the type system `B`...
+    |
+    |
+ LL | const B: i32 = A;
+    | ^^^^^^^^^^^^^^^^^
+ note: ...which requires simplifying constant for the type system `B`...
+    |
+    |
+ LL | const B: i32 = A;
+    | ^^^^^^^^^^^^^^^^^
+ note: ...which requires const-evaluating + checking `B`...
+    |
+    |
+ LL | const B: i32 = A;
+    | ^^^^^^^^^^^^^^^^^
+    = note: ...which requires normalizing `A`...
+    = note: ...which again requires simplifying constant for the type system `A`, completing the cycle
+    = note: cycle used when running analysis passes on this crate
19 error: aborting due to previous error
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3/issue-23302-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-23302-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `A`
   |
   |
LL | const A: i32 = B; //~ ERROR cycle detected
   |
   |
note: ...which requires simplifying constant for the type system `A`...
   |
   |
LL | const A: i32 = B; //~ ERROR cycle detected
   | ^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `A`...
   |
   |
LL | const A: i32 = B; //~ ERROR cycle detected
   | ^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `B`...
note: ...which requires simplifying constant for the type system `B`...
   |
   |
LL | const B: i32 = A;
   | ^^^^^^^^^^^^^^^^^
note: ...which requires simplifying constant for the type system `B`...
   |
   |
LL | const B: i32 = A;
   | ^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `B`...
   |
   |
LL | const B: i32 = A;
   | ^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `A`...
   = note: ...which again requires simplifying constant for the type system `A`, completing the cycle
   = note: cycle used when running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
------------------------------------------
---
8 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/issue-41394.rs:7:9
-    |
- LL |     A = Foo::A as isize
-    |         ^^^^^^^^^^^^^^^ referenced constant has errors
+ error: aborting due to previous error
- error: aborting due to 2 previous errors
- 
- Some errors have detailed explanations: E0080, E0369.
- For more information about an error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args issues/issue-41394.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: cannot add `{integer}` to `&str`
   |
   |
LL |     A = "" + 1
   |         -- ^ - {integer}
   |         &str

error: aborting due to previous error

---
14 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/issue-54954.rs:11:15
-    |
- LL | fn f(z: [f32; ARR_LEN]) -> [f32; ARR_LEN] {
-    |               ^^^^^^^ referenced constant has errors
+ error: aborting due to 2 previous errors
- error[E0080]: evaluation of constant value failed
-   --> $DIR/issue-54954.rs:11:34
-    |
-    |
- LL | fn f(z: [f32; ARR_LEN]) -> [f32; ARR_LEN] {
-    |                                  ^^^^^^^ referenced constant has errors
- error: aborting due to 4 previous errors
- 
- Some errors have detailed explanations: E0080, E0283, E0379.
- For more information about an error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args issues/issue-54954.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-54954.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54954" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54954/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-54954.rs:5:5
   |
   |
LL |     const fn const_val<T: Sized>() -> usize {

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-54954.rs:1:24
   |
   |
LL | const ARR_LEN: usize = Tt::const_val::<[i8; 123]>();
   |
   = note: cannot satisfy `_: Tt`

error: aborting due to 2 previous errors
---

---- [ui] ui/issues/issue-69602-type-err-during-codegen-ice.rs stdout ----
diff of stderr:

13 LL | impl TraitB for B {
14    | ^^^^^^^^^^^^^^^^^ missing `MyA` in implementation
- error[E0080]: evaluation of constant value failed
-   --> $DIR/issue-69602-type-err-during-codegen-ice.rs:21:17
-    |
-    |
- LL |     let _ = [0; B::VALUE];
-    |                 ^^^^^^^^ referenced constant has errors
+ error: aborting due to 2 previous errors
- error: aborting due to 3 previous errors
- 
- Some errors have detailed explanations: E0046, E0080, E0437.
+ Some errors have detailed explanations: E0046, E0437.
---
To only update this specific test, also pass `--test-args issues/issue-69602-type-err-during-codegen-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-69602-type-err-during-codegen-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69602-type-err-during-codegen-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69602-type-err-during-codegen-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0437]: type `M` is not a member of trait `TraitB`
   |
   |
LL |     type M   = A; //~ ERROR type `M` is not a member of trait `TraitB`
   |     ^^^^^^^^^^^^^ not a member of trait `TraitB`
error[E0046]: not all trait items implemented, missing: `MyA`
  --> /checkout/src/test/ui/issues/issue-69602-type-err-during-codegen-ice.rs:16:1
   |
   |
LL |     type MyA: TraitA;
   |     ----------------- `MyA` from trait
...
LL | impl TraitB for B { //~ ERROR not all trait items implemented, missing: `MyA`
   | ^^^^^^^^^^^^^^^^^ missing `MyA` in implementation
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0046, E0437.
For more information about an error, try `rustc --explain E0046`.
For more information about an error, try `rustc --explain E0046`.
------------------------------------------


---- [ui] ui/issues/issue-77919.rs stdout ----
diff of stderr:

26 LL | impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}
27    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `VAL` in implementation
- error[E0080]: evaluation of constant value failed
-   --> $DIR/issue-77919.rs:2:9
-    |
-    |
- LL |     [1; <Multiply<Five, Five>>::VAL];
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
+ error: aborting due to 3 previous errors
- error: aborting due to 4 previous errors
- 
- Some errors have detailed explanations: E0046, E0080, E0412.
+ Some errors have detailed explanations: E0046, E0412.
---
To only update this specific test, also pass `--test-args issues/issue-77919.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-77919.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-77919" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-77919/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0412]: cannot find type `PhantomData` in this scope
   |
   |
LL |     _n: PhantomData, //~ ERROR cannot find type `PhantomData` in this scope
   |
help: consider importing this struct
   |
LL | use std::marker::PhantomData;
LL | use std::marker::PhantomData;
   |

error[E0412]: cannot find type `VAL` in this scope
  --> /checkout/src/test/ui/issues/issue-77919.rs:12:63
   |
LL | impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}
   |          -                                                    ^^^ not found in this scope
   |          |
   |          help: you might be missing a type parameter: `, VAL`
error[E0046]: not all trait items implemented, missing: `VAL`
  --> /checkout/src/test/ui/issues/issue-77919.rs:12:1
   |
LL |     const VAL: T;
LL |     const VAL: T;
   |     ------------- `VAL` from trait
...
LL | impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `VAL` in implementation
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0046, E0412.
For more information about an error, try `rustc --explain E0046`.
---
13 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/issue-50599.rs:4:29
-    |
- LL |     let mut digits = [0u32; M];
-    |                             ^ referenced constant has errors
+ error: aborting due to previous error
- error: aborting due to 2 previous errors
- 
- Some errors have detailed explanations: E0080, E0425.
- For more information about an error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args resolve/issue-50599.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-50599.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-50599" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-50599/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `LOG10_2` in module `std::f64`
   |
   |
LL |     const M: usize = (f64::from(N) * std::f64::LOG10_2) as usize; //~ ERROR cannot find value
   |                                                ^^^^^^^ not found in `std::f64`
help: consider importing one of these items
   |
LL | use std::f32::consts::LOG10_2;
   |
---
---- [ui] ui/type/type-dependent-def-issue-49241.rs stdout ----
diff of stderr:

6    |     |
7    |     help: consider using `let` instead of `const`: `let l`
- error[E0080]: evaluation of constant value failed
-   --> $DIR/type-dependent-def-issue-49241.rs:4:18
-    |
-    |
- LL |     let s: [u32; l] = v.into_iter().collect();
-    |                  ^ referenced constant has errors
+ error: aborting due to previous error
- error: aborting due to 2 previous errors
- 
- Some errors have detailed explanations: E0080, E0435.
- For more information about an error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args type/type-dependent-def-issue-49241.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-dependent-def-issue-49241.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:3:22
   |
   |
LL |     const l: usize = v.count(); //~ ERROR attempt to use a non-constant value in a constant
   |     -------          ^ non-constant value
   |     |
   |     help: consider using `let` instead of `const`: `let l`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0435`.
------------------------------------------
