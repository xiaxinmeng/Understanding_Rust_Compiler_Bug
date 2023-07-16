plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/range/range_traits-1.rs stdout ----
diff of stderr:

1 error[E0277]: can't compare `std::ops::Range<usize>` with `std::ops::Range<usize>`
-   --> $DIR/range_traits-1.rs:5:5
3    |
3    |
4 LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
-    |                                ---------- in this derive macro expansion
- LL | struct AllTheRanges {
- LL |     a: Range<usize>,
-    |     ^^^^^^^^^^^^^^^ no implementation for `std::ops::Range<usize> < std::ops::Range<usize>` and `std::ops::Range<usize> > std::ops::Range<usize>`
+    |                                ^^^^^^^^^^ no implementation for `std::ops::Range<usize> < std::ops::Range<usize>` and `std::ops::Range<usize> > std::ops::Range<usize>`
10    = help: the trait `PartialOrd` is not implemented for `std::ops::Range<usize>`
+    = note: required for `&std::ops::Range<usize>` to implement `PartialOrd`
+    = note: 1 redundant requirement hidden
+    = note: 1 redundant requirement hidden
+    = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
+    = note: the full type name has been written to '$TEST_BUILD_DIR/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
11    = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)
12 
13 error[E0277]: can't compare `std::ops::RangeTo<usize>` with `std::ops::RangeTo<usize>`
-   --> $DIR/range_traits-1.rs:8:5
+   --> $DIR/range_traits-1.rs:3:32
15    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
16 LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
- ...
- ...
- LL |     b: RangeTo<usize>,
-    |     ^^^^^^^^^^^^^^^^^ no implementation for `std::ops::RangeTo<usize> < std::ops::RangeTo<usize>` and `std::ops::RangeTo<usize> > std::ops::RangeTo<usize>`
+    |                                ^^^^^^^^^^ no implementation for `std::ops::RangeTo<usize> < std::ops::RangeTo<usize>` and `std::ops::RangeTo<usize> > std::ops::RangeTo<usize>`
21    |
22    = help: the trait `PartialOrd` is not implemented for `std::ops::RangeTo<usize>`
+    = note: required for `&std::ops::RangeTo<usize>` to implement `PartialOrd`
+    = note: 1 redundant requirement hidden
+    = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
+    = note: the full type name has been written to '$TEST_BUILD_DIR/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
23    = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)
24 
25 error[E0277]: can't compare `std::ops::RangeFrom<usize>` with `std::ops::RangeFrom<usize>`
-   --> $DIR/range_traits-1.rs:11:5
+   --> $DIR/range_traits-1.rs:3:32
27    |
27    |
28 LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
- ...
- ...
- LL |     c: RangeFrom<usize>,
-    |     ^^^^^^^^^^^^^^^^^^^ no implementation for `std::ops::RangeFrom<usize> < std::ops::RangeFrom<usize>` and `std::ops::RangeFrom<usize> > std::ops::RangeFrom<usize>`
+    |                                ^^^^^^^^^^ no implementation for `std::ops::RangeFrom<usize> < std::ops::RangeFrom<usize>` and `std::ops::RangeFrom<usize> > std::ops::RangeFrom<usize>`
33    |
34    = help: the trait `PartialOrd` is not implemented for `std::ops::RangeFrom<usize>`
+    = note: required for `&std::ops::RangeFrom<usize>` to implement `PartialOrd`
+    = note: 1 redundant requirement hidden
+    = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
+    = note: the full type name has been written to '$TEST_BUILD_DIR/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
35    = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)
36 
37 error[E0277]: can't compare `std::ops::RangeFull` with `std::ops::RangeFull`
-   --> $DIR/range_traits-1.rs:14:5
+   --> $DIR/range_traits-1.rs:3:32
39    |
39    |
40 LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
- ...
- LL |     d: RangeFull,
- LL |     d: RangeFull,
-    |     ^^^^^^^^^^^^ no implementation for `std::ops::RangeFull < std::ops::RangeFull` and `std::ops::RangeFull > std::ops::RangeFull`
+    |                                ^^^^^^^^^^ no implementation for `std::ops::RangeFull < std::ops::RangeFull` and `std::ops::RangeFull > std::ops::RangeFull`
46    = help: the trait `PartialOrd` is not implemented for `std::ops::RangeFull`
46    = help: the trait `PartialOrd` is not implemented for `std::ops::RangeFull`
+    = note: required for `&std::ops::RangeFull` to implement `PartialOrd`
+    = note: 1 redundant requirement hidden
+    = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
+    = note: the full type name has been written to '$TEST_BUILD_DIR/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
47    = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)
48 
49 error[E0277]: can't compare `std::ops::RangeInclusive<usize>` with `std::ops::RangeInclusive<usize>`
-   --> $DIR/range_traits-1.rs:17:5
+   --> $DIR/range_traits-1.rs:3:32
51    |
51    |
52 LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
- ...
- ...
- LL |     e: RangeInclusive<usize>,
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `std::ops::RangeInclusive<usize> < std::ops::RangeInclusive<usize>` and `std::ops::RangeInclusive<usize> > std::ops::RangeInclusive<usize>`
+    |                                ^^^^^^^^^^ no implementation for `std::ops::RangeInclusive<usize> < std::ops::RangeInclusive<usize>` and `std::ops::RangeInclusive<usize> > std::ops::RangeInclusive<usize>`
57    |
58    = help: the trait `PartialOrd` is not implemented for `std::ops::RangeInclusive<usize>`
+    = note: required for `&std::ops::RangeInclusive<usize>` to implement `PartialOrd`
+    = note: 1 redundant requirement hidden
+    = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
+    = note: the full type name has been written to '$TEST_BUILD_DIR/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
59    = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)
60 
61 error[E0277]: can't compare `std::ops::RangeToInclusive<usize>` with `std::ops::RangeToInclusive<usize>`
-   --> $DIR/range_traits-1.rs:20:5
+   --> $DIR/range_traits-1.rs:3:32
63    |
63    |
64 LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
- ...
- ...
- LL |     f: RangeToInclusive<usize>,
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `std::ops::RangeToInclusive<usize> < std::ops::RangeToInclusive<usize>` and `std::ops::RangeToInclusive<usize> > std::ops::RangeToInclusive<usize>`
+    |                                ^^^^^^^^^^ no implementation for `std::ops::RangeToInclusive<usize> < std::ops::RangeToInclusive<usize>` and `std::ops::RangeToInclusive<usize> > std::ops::RangeToInclusive<usize>`
69    |
70    = help: the trait `PartialOrd` is not implemented for `std::ops::RangeToInclusive<usize>`
+    = note: required for `&std::ops::RangeToInclusive<usize>` to implement `PartialOrd`
+    = note: 1 redundant requirement hidden
+    = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
+    = note: the full type name has been written to '$TEST_BUILD_DIR/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
71    = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)
72 
73 error[E0277]: the trait bound `std::ops::Range<usize>: Ord` is not satisfied

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1/range_traits-1.stderr
To only update this specific test, also pass `--test-args range/range_traits-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/range/range_traits-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: can't compare `std::ops::Range<usize>` with `std::ops::Range<usize>`
  --> fake-test-src-base/range/range_traits-1.rs:3:32
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
   |                                ^^^^^^^^^^ no implementation for `std::ops::Range<usize> < std::ops::Range<usize>` and `std::ops::Range<usize> > std::ops::Range<usize>`
   = help: the trait `PartialOrd` is not implemented for `std::ops::Range<usize>`
   = note: required for `&std::ops::Range<usize>` to implement `PartialOrd`
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
   = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: can't compare `std::ops::RangeTo<usize>` with `std::ops::RangeTo<usize>`
  --> fake-test-src-base/range/range_traits-1.rs:3:32
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
   |                                ^^^^^^^^^^ no implementation for `std::ops::RangeTo<usize> < std::ops::RangeTo<usize>` and `std::ops::RangeTo<usize> > std::ops::RangeTo<usize>`
   |
   = help: the trait `PartialOrd` is not implemented for `std::ops::RangeTo<usize>`
   = note: required for `&std::ops::RangeTo<usize>` to implement `PartialOrd`
   = note: 1 redundant requirement hidden
   = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
   = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: can't compare `std::ops::RangeFrom<usize>` with `std::ops::RangeFrom<usize>`
  --> fake-test-src-base/range/range_traits-1.rs:3:32
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
   |                                ^^^^^^^^^^ no implementation for `std::ops::RangeFrom<usize> < std::ops::RangeFrom<usize>` and `std::ops::RangeFrom<usize> > std::ops::RangeFrom<usize>`
   |
   = help: the trait `PartialOrd` is not implemented for `std::ops::RangeFrom<usize>`
   = note: required for `&std::ops::RangeFrom<usize>` to implement `PartialOrd`
   = note: 1 redundant requirement hidden
   = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
   = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: can't compare `std::ops::RangeFull` with `std::ops::RangeFull`
  --> fake-test-src-base/range/range_traits-1.rs:3:32
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
   |                                ^^^^^^^^^^ no implementation for `std::ops::RangeFull < std::ops::RangeFull` and `std::ops::RangeFull > std::ops::RangeFull`
   = help: the trait `PartialOrd` is not implemented for `std::ops::RangeFull`
   = help: the trait `PartialOrd` is not implemented for `std::ops::RangeFull`
   = note: required for `&std::ops::RangeFull` to implement `PartialOrd`
   = note: 1 redundant requirement hidden
   = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
   = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: can't compare `std::ops::RangeInclusive<usize>` with `std::ops::RangeInclusive<usize>`
  --> fake-test-src-base/range/range_traits-1.rs:3:32
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
   |                                ^^^^^^^^^^ no implementation for `std::ops::RangeInclusive<usize> < std::ops::RangeInclusive<usize>` and `std::ops::RangeInclusive<usize> > std::ops::RangeInclusive<usize>`
   |
   = help: the trait `PartialOrd` is not implemented for `std::ops::RangeInclusive<usize>`
   = note: required for `&std::ops::RangeInclusive<usize>` to implement `PartialOrd`
   = note: 1 redundant requirement hidden
   = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
   = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: can't compare `std::ops::RangeToInclusive<usize>` with `std::ops::RangeToInclusive<usize>`
  --> fake-test-src-base/range/range_traits-1.rs:3:32
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
   |                                ^^^^^^^^^^ no implementation for `std::ops::RangeToInclusive<usize> < std::ops::RangeToInclusive<usize>` and `std::ops::RangeToInclusive<usize> > std::ops::RangeToInclusive<usize>`
   |
   = help: the trait `PartialOrd` is not implemented for `std::ops::RangeToInclusive<usize>`
   = note: required for `&std::ops::RangeToInclusive<usize>` to implement `PartialOrd`
   = note: 1 redundant requirement hidden
   = note: required for `(&Range<usize>, &RangeTo<usize>, &RangeFrom<usize>, &RangeFull, &RangeInclusive<usize>, &...)` to implement `PartialOrd`
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1/range_traits-1.long-type-15923496276551163449.txt'
   = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `std::ops::Range<usize>: Ord` is not satisfied
  --> fake-test-src-base/range/range_traits-1.rs:5:5
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
   |                                            --- in this derive macro expansion
LL | struct AllTheRanges {
LL |     a: Range<usize>,
   |     ^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `std::ops::Range<usize>`
   = note: this error originates in the derive macro `Ord` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the trait bound `std::ops::RangeTo<usize>: Ord` is not satisfied
  --> fake-test-src-base/range/range_traits-1.rs:8:5
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
...
...
LL |     b: RangeTo<usize>,
   |     ^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `std::ops::RangeTo<usize>`
   = note: this error originates in the derive macro `Ord` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the trait bound `std::ops::RangeFrom<usize>: Ord` is not satisfied
  --> fake-test-src-base/range/range_traits-1.rs:11:5
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
...
...
LL |     c: RangeFrom<usize>,
   |     ^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `std::ops::RangeFrom<usize>`
   = note: this error originates in the derive macro `Ord` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the trait bound `std::ops::RangeFull: Ord` is not satisfied
  --> fake-test-src-base/range/range_traits-1.rs:14:5
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
...
LL |     d: RangeFull,
   |     ^^^^^^^^^^^^ the trait `Ord` is not implemented for `std::ops::RangeFull`
   |
   |
   = note: this error originates in the derive macro `Ord` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `std::ops::RangeInclusive<usize>: Ord` is not satisfied
  --> fake-test-src-base/range/range_traits-1.rs:17:5
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
...
...
LL |     e: RangeInclusive<usize>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `std::ops::RangeInclusive<usize>`
   = note: this error originates in the derive macro `Ord` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0277]: the trait bound `std::ops::RangeToInclusive<usize>: Ord` is not satisfied
  --> fake-test-src-base/range/range_traits-1.rs:20:5
   |
LL | #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
...
LL |     f: RangeToInclusive<usize>,
LL |     f: RangeToInclusive<usize>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `std::ops::RangeToInclusive<usize>`
   = note: this error originates in the derive macro `Ord` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 12 previous errors

