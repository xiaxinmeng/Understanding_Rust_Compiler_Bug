plain
9    |
10 LL | #![deny(unused_attributes)]
11    |         ^^^^^^^^^^^^^^^^^

12    = note: attribute `repr` with an empty list has no effect
14 error: unused attribute
-   --> $DIR/empty-attributes.rs:14:1
+   --> $DIR/empty-attributes.rs:12:1
16    |
16    |
17 LL | #[target_feature()]


20    = note: attribute `target_feature` with an empty list has no effect
22 error: unused attribute
-   --> $DIR/empty-attributes.rs:4:1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/empty-attributes.rs:2:1
+   --> $DIR/empty-attributes.rs:2:1
24    |
25 LL | #![allow()]
26    | ^^^^^^^^^^^ help: remove this attribute

28    = note: attribute `allow` with an empty list has no effect
30 error: unused attribute
-   --> $DIR/empty-attributes.rs:5:1
+   --> $DIR/empty-attributes.rs:3:1
32    |
32    |
33 LL | #![expect()]


36    = note: attribute `expect` with an empty list has no effect
38 error: unused attribute
-   --> $DIR/empty-attributes.rs:6:1
+   --> $DIR/empty-attributes.rs:4:1
40    |
40    |
41 LL | #![warn()]
42    | ^^^^^^^^^^ help: remove this attribute

44    = note: attribute `warn` with an empty list has no effect
46 error: unused attribute
-   --> $DIR/empty-attributes.rs:7:1
+   --> $DIR/empty-attributes.rs:5:1
48    |
48    |
49 LL | #![deny()]
50    | ^^^^^^^^^^ help: remove this attribute

52    = note: attribute `deny` with an empty list has no effect
54 error: unused attribute
-   --> $DIR/empty-attributes.rs:8:1
+   --> $DIR/empty-attributes.rs:6:1
56    |
56    |
57 LL | #![forbid()]


60    = note: attribute `forbid` with an empty list has no effect
62 error: unused attribute
-   --> $DIR/empty-attributes.rs:9:1
+   --> $DIR/empty-attributes.rs:7:1
64    |
---
To only update this specific test, also pass `--test-args empty/empty-attributes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-attributes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-attributes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-attributes/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/empty/empty-attributes.rs:9:1
   |
   |
LL | #[repr()] //~ ERROR unused attribute
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/empty/empty-attributes.rs:1:9
   |
   |
LL | #![deny(unused_attributes)]
   |         ^^^^^^^^^^^^^^^^^
   = note: attribute `repr` with an empty list has no effect
error: unused attribute
  --> /checkout/src/test/ui/empty/empty-attributes.rs:12:1
   |
   |
LL | #[target_feature()] //~ ERROR unused attribute
   |
   |
   = note: attribute `target_feature` with an empty list has no effect
error: unused attribute
  --> /checkout/src/test/ui/empty/empty-attributes.rs:2:1
   |
   |
LL | #![allow()] //~ ERROR unused attribute
   |
   |
   = note: attribute `allow` with an empty list has no effect
error: unused attribute
  --> /checkout/src/test/ui/empty/empty-attributes.rs:3:1
   |
   |
LL | #![expect()] //~ ERROR unused attribute
   |
   |
   = note: attribute `expect` with an empty list has no effect
error: unused attribute
  --> /checkout/src/test/ui/empty/empty-attributes.rs:4:1
   |
   |
LL | #![warn()] //~ ERROR unused attribute
   |
   |
   = note: attribute `warn` with an empty list has no effect
error: unused attribute
  --> /checkout/src/test/ui/empty/empty-attributes.rs:5:1
   |
   |
LL | #![deny()] //~ ERROR unused attribute
   |
   |
   = note: attribute `deny` with an empty list has no effect
error: unused attribute
  --> /checkout/src/test/ui/empty/empty-attributes.rs:6:1
   |
   |
LL | #![forbid()] //~ ERROR unused attribute
   |
   |
   = note: attribute `forbid` with an empty list has no effect
error: unused attribute
  --> /checkout/src/test/ui/empty/empty-attributes.rs:7:1
   |
   |
LL | #![feature()] //~ ERROR unused attribute
   |
   |
   = note: attribute `feature` with an empty list has no effect
error: aborting due to 8 previous errors
------------------------------------------


