plain
diff of stderr:

16   --> $DIR/type-overflow.rs:10:16
17    |
18 LL |     let fail = 0b1000_0001i8;
-    |                ^^^^^^^^^^^^^ help: consider using the type `u8` instead: `0b1000_0001u8`
20    |
20    |
21    = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into the type `i8` and will become `-127i8`
+ help: consider using the type `u8` instead
+    |
+ LL |     let fail = 0b1000_0001u8;
+    |                ~~~~~~~~~~~~~
+ help: to use as a negative number (decimal `-127`), consider using the type `u8` for the literal and cast it to `i8`
+    |
+ LL |     let fail = 0b1000_0001u8 as i8;
22 
23 warning: literal out of range for `i64`
24   --> $DIR/type-overflow.rs:12:16


25    |
26 LL |     let fail = 0x8000_0000_0000_0000i64;
-    |                ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using the type `u64` instead: `0x8000_0000_0000_0000u64`
28    |
28    |
29    = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into the type `i64` and will become `-9223372036854775808i64`
+ help: consider using the type `u64` instead
+    |
+ LL |     let fail = 0x8000_0000_0000_0000u64;
+    |                ~~~~~~~~~~~~~~~~~~~~~~~~
+ help: to use as a negative number (decimal `-9223372036854775808`), consider using the type `u64` for the literal and cast it to `i64`
+    |
+ LL |     let fail = 0x8000_0000_0000_0000u64 as i64;
30 
31 warning: literal out of range for `u32`
32   --> $DIR/type-overflow.rs:14:16


44    |
45    = note: the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into the type `i128` and will become `-170141183460469231731687303715884105728i128`
46    = help: consider using the type `u128` instead
+ help: to use as a negative number (decimal `-170141183460469231731687303715884105728`), consider using the type `u128` for the literal and cast it to `i128`
+    |
+ LL |     let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000u128 as i128;
47 
48 warning: literal out of range for `i32`
49   --> $DIR/type-overflow.rs:19:16


53    |
54    = note: the literal `0x8FFF_FFFF_FFFF_FFFE` (decimal `10376293541461622782`) does not fit into the type `i32` and will become `-2i32`
55    = help: consider using the type `i128` instead
+ help: to use as a negative number (decimal `-2`), consider using the type `u32` for the literal and cast it to `i32`
+    |
+ LL |     let fail = 0x8FFF_FFFF_FFFF_FFFEu32 as i32;
56 
57 warning: literal out of range for `i8`
58   --> $DIR/type-overflow.rs:21:17

---
To only update this specific test, also pass `--test-args lint/type-overflow.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/type-overflow.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/type-overflow/auxiliary"
stdout: none
warning: literal out of range for `i8`
  --> fake-test-src-base/lint/type-overflow.rs:5:17
   |
LL |     let error = 255i8; //~WARNING literal out of range for `i8`
LL |     let error = 255i8; //~WARNING literal out of range for `i8`
   |                 ^^^^^
   |
   = note: the literal `255i8` does not fit into the type `i8` whose range is `-128..=127`
note: the lint level is defined here
  --> fake-test-src-base/lint/type-overflow.rs:2:9
   |
LL | #![warn(overflowing_literals)]
LL | #![warn(overflowing_literals)]
   |         ^^^^^^^^^^^^^^^^^^^^

warning: literal out of range for `i8`
  --> fake-test-src-base/lint/type-overflow.rs:10:16
   |
LL |     let fail = 0b1000_0001i8; //~WARNING literal out of range for `i8`
   |
   |
   = note: the literal `0b1000_0001i8` (decimal `129`) does not fit into the type `i8` and will become `-127i8`
   |
   |
LL |     let fail = 0b1000_0001u8; //~WARNING literal out of range for `i8`
   |                ~~~~~~~~~~~~~
help: to use as a negative number (decimal `-127`), consider using the type `u8` for the literal and cast it to `i8`
   |
LL |     let fail = 0b1000_0001u8 as i8; //~WARNING literal out of range for `i8`

warning: literal out of range for `i64`
  --> fake-test-src-base/lint/type-overflow.rs:12:16
   |
   |
LL |     let fail = 0x8000_0000_0000_0000i64; //~WARNING literal out of range for `i64`
   |
   |
   = note: the literal `0x8000_0000_0000_0000i64` (decimal `9223372036854775808`) does not fit into the type `i64` and will become `-9223372036854775808i64`
   |
   |
LL |     let fail = 0x8000_0000_0000_0000u64; //~WARNING literal out of range for `i64`
   |                ~~~~~~~~~~~~~~~~~~~~~~~~
help: to use as a negative number (decimal `-9223372036854775808`), consider using the type `u64` for the literal and cast it to `i64`
   |
LL |     let fail = 0x8000_0000_0000_0000u64 as i64; //~WARNING literal out of range for `i64`

warning: literal out of range for `u32`
  --> fake-test-src-base/lint/type-overflow.rs:14:16
   |
   |
LL |     let fail = 0x1_FFFF_FFFFu32; //~WARNING literal out of range for `u32`
   |                ^^^^^^^^^^^^^^^^ help: consider using the type `u64` instead: `0x1_FFFF_FFFFu64`
   |
   = note: the literal `0x1_FFFF_FFFFu32` (decimal `8589934591`) does not fit into the type `u32` and will become `4294967295u32`
warning: literal out of range for `i128`
  --> fake-test-src-base/lint/type-overflow.rs:16:22
   |
   |
LL |     let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000;
   |
   |
   = note: the literal `0x8000_0000_0000_0000_0000_0000_0000_0000` (decimal `170141183460469231731687303715884105728`) does not fit into the type `i128` and will become `-170141183460469231731687303715884105728i128`
   = help: consider using the type `u128` instead
help: to use as a negative number (decimal `-170141183460469231731687303715884105728`), consider using the type `u128` for the literal and cast it to `i128`
   |
LL |     let fail: i128 = 0x8000_0000_0000_0000_0000_0000_0000_0000u128 as i128;

warning: literal out of range for `i32`
  --> fake-test-src-base/lint/type-overflow.rs:19:16
   |
   |
LL |     let fail = 0x8FFF_FFFF_FFFF_FFFE; //~WARNING literal out of range for `i32`
   |
   |
   = note: the literal `0x8FFF_FFFF_FFFF_FFFE` (decimal `10376293541461622782`) does not fit into the type `i32` and will become `-2i32`
   = help: consider using the type `i128` instead
help: to use as a negative number (decimal `-2`), consider using the type `u32` for the literal and cast it to `i32`
   |
LL |     let fail = 0x8FFF_FFFF_FFFF_FFFEu32 as i32; //~WARNING literal out of range for `i32`

warning: literal out of range for `i8`
  --> fake-test-src-base/lint/type-overflow.rs:21:17
   |
   |
LL |     let fail = -0b1111_1111i8; //~WARNING literal out of range for `i8`
   |                 ^^^^^^^^^^^^^ help: consider using the type `i16` instead: `0b1111_1111i16`
   |
   = note: the literal `0b1111_1111i8` (decimal `255`) does not fit into the type `i8`
   = note: and the value `-0b1111_1111i8` will become `1i8`
warning: 7 warnings emitted
------------------------------------------


