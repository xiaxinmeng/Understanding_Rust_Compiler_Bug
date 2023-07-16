plain
...............................................................................................F...F 200
..F................................................................................................. 300
....................................................................................
tests/fail/intrinsics/simd-float-to-int.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "tests/fail/intrinsics/simd-float-to-int.rs" "--edition" "2018"
actual output differed from expected
--- tests/fail/intrinsics/simd-float-to-int.stderr
+++ <stderr output>
+++ <stderr output>
 error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `i32`
+  --> $DIR/simd-float-to-int.rs:LL:CC
    |
-LL |         unsafe { intrinsics::simd_cast(self) }
-LL |         unsafe { intrinsics::simd_cast(self) }
-   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `i32`
+LL |         let _x: i32x2 = f32x2::from_array([f32::MAX, f32::MIN]).to_int_unchecked();
+   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `i32`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: BACKTRACE:
    = note: BACKTRACE:
-   = note: inside `std::simd::Simd::<f32, 2>::to_int_unchecked::<i32>` at RUSTLIB/core/src/../../portable-simd/crates/core_simd/src/vector.rs:LL:CC
-  --> $DIR/simd-float-to-int.rs:LL:CC
-   |
-   |
-LL |         let _x: i32x2 = f32x2::from_array([f32::MAX, f32::MIN]).to_int_unchecked();
+   = note: inside `main` at $DIR/simd-float-to-int.rs:LL:CC
 
 note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
... 4 lines skipped ...
... 4 lines skipped ...


substring `cannot be represented in target type `i32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/simd-float-to-int.rs:1

There were 1 unmatched diagnostics at tests/fail/intrinsics/simd-float-to-int.rs:7
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `i32`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `i32`
   |
   |
LL |         let _x: i32x2 = f32x2::from_array([f32::MAX, f32::MIN]).to_int_unchecked();
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `i32`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/simd-float-to-int.rs:7:25: 7:83
---



tests/fail/intrinsics/simd-gather.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "tests/fail/intrinsics/simd-gather.rs" "--edition" "2018"
actual output differed from expected
--- tests/fail/intrinsics/simd-gather.stderr
+++ <stderr output>
+++ <stderr output>
 error: Undefined Behavior: dereferencing pointer failed: ALLOC has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
+  --> $DIR/simd-gather.rs:LL:CC
    |
    |
-LL |         unsafe { intrinsics::simd_gather(or, ptrs, enable.to_int()) }
-   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: ALLOC has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
+LL |         let _result = Simd::gather_select_unchecked(&vec, Mask::splat(true), idxs, Simd::splat(0));
+   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: ALLOC has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: BACKTRACE:
-   = note: inside `std::simd::Simd::<i8, 4>::gather_select_unchecked` at RUSTLIB/core/src/../../portable-simd/crates/core_simd/src/vector.rs:LL:CC
-   = note: inside `std::simd::Simd::<i8, 4>::gather_select_unchecked` at RUSTLIB/core/src/../../portable-simd/crates/core_simd/src/vector.rs:LL:CC
-note: inside `main`
-  --> $DIR/simd-gather.rs:LL:CC
-   |
-LL |         let _result = Simd::gather_select_unchecked(&vec, Mask::splat(true), idxs, Simd::splat(0));
+   = note: inside `main` at $DIR/simd-gather.rs:LL:CC
 
 note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
... 4 lines skipped ...
... 4 lines skipped ...


substring `pointer to 1 byte starting at offset 9 is out-of-bounds` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/simd-gather.rs:1

There were 1 unmatched diagnostics at tests/fail/intrinsics/simd-gather.rs:9
    Error: Undefined Behavior: dereferencing pointer failed: alloc3 has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
full stderr:
full stderr:
error: Undefined Behavior: dereferencing pointer failed: alloc3 has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
   |
   |
LL |         let _result = Simd::gather_select_unchecked(&vec, Mask::splat(true), idxs, Simd::splat(0));
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: alloc3 has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/simd-gather.rs:9:23: 9:99
---



tests/fail/intrinsics/simd-scatter.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "tests/fail/intrinsics/simd-scatter.rs" "--edition" "2018"
actual output differed from expected
--- tests/fail/intrinsics/simd-scatter.stderr
+++ <stderr output>
+++ <stderr output>
 error: Undefined Behavior: dereferencing pointer failed: ALLOC has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
-   |
-   |
-LL |             intrinsics::simd_scatter(self, ptrs, enable.to_int())
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: ALLOC has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
-   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
-   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
-   = note: BACKTRACE:
-   = note: inside `std::simd::Simd::<i8, 4>::scatter_select_unchecked` at RUSTLIB/core/src/../../portable-simd/crates/core_simd/src/vector.rs:LL:CC
-   = note: inside `std::simd::Simd::<i8, 4>::scatter_select_unchecked` at RUSTLIB/core/src/../../portable-simd/crates/core_simd/src/vector.rs:LL:CC
-note: inside `main`
   --> $DIR/simd-scatter.rs:LL:CC
    |
... 3 lines skipped ...
 LL | |             idxs,
 LL | |         );
~   | |_________^ dereferencing pointer failed: ALLOC has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
+   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
+   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
+   = note: BACKTRACE:
+   = note: inside `main` at $DIR/simd-scatter.rs:LL:CC
---
substring `pointer to 1 byte starting at offset 9 is out-of-bounds` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/simd-scatter.rs:1

There were 1 unmatched diagnostics at tests/fail/intrinsics/simd-scatter.rs:9
    Error: Undefined Behavior: dereferencing pointer failed: alloc1456 has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
full stderr:
full stderr:
error: Undefined Behavior: dereferencing pointer failed: alloc1456 has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
   |
   |
LL | /         Simd::from_array([-27, 82, -41, 124]).scatter_select_unchecked(
LL | |             &mut vec,
LL | |             Mask::splat(true),
LL | |             idxs,
LL | |         );
   | |_________^ dereferencing pointer failed: alloc1456 has size 9, so pointer to 1 byte starting at offset 9 is out-of-bounds
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/simd-scatter.rs:9:9: 13:10
