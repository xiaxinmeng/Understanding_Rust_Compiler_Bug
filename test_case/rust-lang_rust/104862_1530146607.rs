plain

## Running ui tests in tests/fail against miri for x86_64-unknown-linux-gnu
   Compiler flags: ["--error-format=json", "-Astable-features", "-Aunused", "-Zui-testing", "--target", "x86_64-unknown-linux-gnu"]
   Building test dependencies...
.................FF.............F.......................i.ii........................................ 100
.................................................................................................... 300
.................................................................................................... 300
...................................FF..F...........................................
tests/fail/invalid_char.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "tests/fail/invalid_char.rs" "-Zmiri-disable-alignment-check" "-Zmiri-disable-stacked-borrows" "-Zmiri-disable-validation" "--edition" "2018"
actual output differed from expected
--- tests/fail/invalid_char.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: interpreting an invalid 32-bit value as a char: $HEX
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+error: abnormal termination: panic in a function that cannot unwind
   --> $DIR/invalid_char.rs:LL:CC
    |
    |
-LL |     let _x = c == 'x';
-   |              ^^^^^^^^ interpreting an invalid 32-bit value as a char: $HEX
+LL | / fn main() {
+LL | |     let c = $HEXu32;
+LL | |     assert!(std::char::from_u32(c).is_none());
+LL | |     let c = unsafe { std::mem::transmute::<u32, char>(c) };
+LL | |     let _x = c == 'x';
+LL | | }
+   | |_^ panic in a function that cannot unwind
-   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
-   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
-   = note: BACKTRACE:
    = note: inside `main` at $DIR/invalid_char.rs:LL:CC
    = note: inside `main` at $DIR/invalid_char.rs:LL:CC
 
... 5 lines skipped ...


substring `interpreting an invalid 32-bit value as a char` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/invalid_char.rs:5
    Error: abnormal termination: panic in a function that cannot unwind


full stderr:
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 1114111', tests/fail/invalid_char.rs:8:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: abnormal termination: panic in a function that cannot unwind
  --> tests/fail/invalid_char.rs:5:1
   |
LL | / fn main() {
LL | |     let c = 0xFFFFFFu32;
LL | |     assert!(std::char::from_u32(c).is_none());
LL | |     let c = unsafe { std::mem::transmute::<u32, char>(c) };
LL | |     let _x = c == 'x';
LL | | }
   | |_^ panic in a function that cannot unwind
   = note: inside `main` at tests/fail/invalid_char.rs:5:1: 10:2

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace


error: aborting due to previous error




tests/fail/invalid_bool.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "tests/fail/invalid_bool.rs" "-Zmiri-disable-alignment-check" "-Zmiri-disable-stacked-borrows" "-Zmiri-disable-validation" "--edition" "2018"
actual output differed from expected
--- tests/fail/invalid_bool.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: interpreting an invalid 8-bit value as a bool: 0x02
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+error: abnormal termination: panic in a function that cannot unwind
   --> $DIR/invalid_bool.rs:LL:CC
    |
    |
-LL |     let _x = b == std::hint::black_box(true);
-   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ interpreting an invalid 8-bit value as a bool: 0x02
+LL | / fn main() {
+LL | |     let b = unsafe { std::mem::transmute::<u8, bool>(2) };
+LL | |     let _x = b == std::hint::black_box(true);
+LL | | }
+   | |_^ panic in a function that cannot unwind
-   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
-   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
-   = note: BACKTRACE:
    = note: inside `main` at $DIR/invalid_bool.rs:LL:CC
    = note: inside `main` at $DIR/invalid_bool.rs:LL:CC
 
... 5 lines skipped ...


substring `interpreting an invalid 8-bit value as a bool` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/invalid_bool.rs:5
    Error: abnormal termination: panic in a function that cannot unwind


full stderr:
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 1', tests/fail/invalid_bool.rs:6:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: abnormal termination: panic in a function that cannot unwind
  --> tests/fail/invalid_bool.rs:5:1
   |
LL | / fn main() {
LL | |     let b = unsafe { std::mem::transmute::<u8, bool>(2) };
LL | |     let _x = b == std::hint::black_box(true);
LL | | }
   | |_^ panic in a function that cannot unwind
   = note: inside `main` at tests/fail/invalid_bool.rs:5:1: 8:2

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace


error: aborting due to previous error




tests/fail/invalid_enum_tag.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "tests/fail/invalid_enum_tag.rs" "-Zmiri-disable-alignment-check" "-Zmiri-disable-stacked-borrows" "-Zmiri-disable-validation" "--edition" "2018"
actual output differed from expected
--- tests/fail/invalid_enum_tag.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: enum value has invalid tag: $HEX
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+error: abnormal termination: panic in a function that cannot unwind
   --> $DIR/invalid_enum_tag.rs:LL:CC
    |
    |
-LL |     let _val = mem::discriminant(&f);
-   |                ^^^^^^^^^^^^^^^^^^^^^ enum value has invalid tag: $HEX
+LL | / fn main() {
+LL | |     let f = unsafe { std::mem::transmute::<i32, Foo>(42) };
+LL | |     let _val = mem::discriminant(&f);
+LL | | }
+   | |_^ panic in a function that cannot unwind
-   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
-   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
-   = note: BACKTRACE:
    = note: inside `main` at $DIR/invalid_enum_tag.rs:LL:CC
    = note: inside `main` at $DIR/invalid_enum_tag.rs:LL:CC
 
... 5 lines skipped ...


substring `enum value has invalid tag` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/invalid_enum_tag.rs:15
    Error: abnormal termination: panic in a function that cannot unwind


full stderr:
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 3', tests/fail/invalid_enum_tag.rs:16:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: abnormal termination: panic in a function that cannot unwind
  --> tests/fail/invalid_enum_tag.rs:15:1
   |
LL | / fn main() {
LL | |     let f = unsafe { std::mem::transmute::<i32, Foo>(42) };
LL | |     let _val = mem::discriminant(&f);
LL | | }
   | |_^ panic in a function that cannot unwind
   = note: inside `main` at tests/fail/invalid_enum_tag.rs:15:1: 18:2

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace


error: aborting due to previous error




tests/fail/validity/invalid_bool.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "tests/fail/validity/invalid_bool.rs" "--edition" "2018"
actual output differed from expected
--- tests/fail/validity/invalid_bool.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: constructing invalid value: encountered 0x02, but expected a boolean
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+error: abnormal termination: panic in a function that cannot unwind
   --> $DIR/invalid_bool.rs:LL:CC
    |
    |
-LL |     let _b = unsafe { std::mem::transmute::<u8, bool>(2) };
-   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0x02, but expected a boolean
+LL | / fn main() {
+LL | |     let _b = unsafe { std::mem::transmute::<u8, bool>(2) };
+LL | | }
+   | |_^ panic in a function that cannot unwind
-   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
-   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
-   = note: BACKTRACE:
    = note: inside `main` at $DIR/invalid_bool.rs:LL:CC
    = note: inside `main` at $DIR/invalid_bool.rs:LL:CC
 
... 5 lines skipped ...


substring `expected a boolean` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/validity/invalid_bool.rs:1
    Error: abnormal termination: panic in a function that cannot unwind


full stderr:
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 1', tests/fail/validity/invalid_bool.rs:2:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: abnormal termination: panic in a function that cannot unwind
  --> tests/fail/validity/invalid_bool.rs:1:1
   |
LL | / fn main() {
LL | |     let _b = unsafe { std::mem::transmute::<u8, bool>(2) };
LL | | }
   | |_^ panic in a function that cannot unwind
   = note: inside `main` at tests/fail/validity/invalid_bool.rs:1:1: 3:2

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace


error: aborting due to previous error




tests/fail/validity/invalid_char.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "tests/fail/validity/invalid_char.rs" "--edition" "2018"
actual output differed from expected
--- tests/fail/validity/invalid_char.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: constructing invalid value: encountered $HEX, but expected a valid unicode scalar value (in `0..=$HEX` but not in `$HEX..=$HEX`)
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+error: abnormal termination: panic in a function that cannot unwind
   --> $DIR/invalid_char.rs:LL:CC
    |
    |
-LL |     let _val = match unsafe { std::mem::transmute::<i32, char>(-1) } {
-   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered $HEX, but expected a valid unicode scalar value (in `0..=$HEX` but not in `$HEX..=$HEX`)
+LL | / fn main() {
+LL | |     assert!(std::char::from_u32(-1_i32 as u32).is_none());
+LL | |     let _val = match unsafe { std::mem::transmute::<i32, char>(-1) } {
+LL | |
+LL | |     };
+LL | | }
+   | |_^ panic in a function that cannot unwind
    |
---
 
... 5 lines skipped ...


substring `encountered 0xffffffff, but expected a valid unicode scalar value` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/validity/invalid_char.rs:1
    Error: abnormal termination: panic in a function that cannot unwind


full stderr:
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 1114111', tests/fail/validity/invalid_char.rs:3:31
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: abnormal termination: panic in a function that cannot unwind
  --> tests/fail/validity/invalid_char.rs:1:1
   |
LL | / fn main() {
LL | |     assert!(std::char::from_u32(-1_i32 as u32).is_none());
LL | |     let _val = match unsafe { std::mem::transmute::<i32, char>(-1) } {
LL | |
LL | |     };
LL | | }
   | |_^ panic in a function that cannot unwind
   |
---



tests/fail/validity/invalid_enum_tag.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "tests/fail/validity/invalid_enum_tag.rs" "--edition" "2018"
actual output differed from expected
--- tests/fail/validity/invalid_enum_tag.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: constructing invalid value at .<enum-tag>: encountered $HEX, but expected a valid enum tag
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+error: abnormal termination: panic in a function that cannot unwind
   --> $DIR/invalid_enum_tag.rs:LL:CC
    |
    |
-LL |     let _f = unsafe { std::mem::transmute::<i32, Foo>(42) };
-   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered $HEX, but expected a valid enum tag
+LL | / fn main() {
+LL | |     let _f = unsafe { std::mem::transmute::<i32, Foo>(42) };
+LL | | }
+   | |_^ panic in a function that cannot unwind
-   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
-   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
-   = note: BACKTRACE:
    = note: inside `main` at $DIR/invalid_enum_tag.rs:LL:CC
---
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: abnormal termination: panic in a function that cannot unwind
  --> tests/fail/validity/invalid_enum_tag.rs:9:1
   |
LL | / fn main() {
LL | |     let _f = unsafe { std::mem::transmute::<i32, Foo>(42) };
LL | | }
   | |_^ panic in a function that cannot unwind
   = note: inside `main` at tests/fail/validity/invalid_enum_tag.rs:9:1: 11:2

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

