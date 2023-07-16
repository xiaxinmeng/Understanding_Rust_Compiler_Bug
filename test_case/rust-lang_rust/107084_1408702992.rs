plain
tests/fail/shims/sync/libc_pthread_rwlock_write_write_deadlock.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/function_pointers/cast_fn_ptr2.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/function_pointers/cast_fn_ptr2.rs"
actual output differed from expected
--- tests/fail/function_pointers/cast_fn_ptr2.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: calling a function with argument of type (i32, i32) passing data of type i32
+error: Undefined Behavior: calling a function with argument of type Tuple([Int(I32), Int(I32)]) passing data of type Int(I32)
    |
 LL |     g(42)
 LL |     g(42)
-   |     ^^^^^ calling a function with argument of type (i32, i32) passing data of type i32
+   |     ^^^^^ calling a function with argument of type Tuple([Int(I32), Int(I32)]) passing data of type Int(I32)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `calling a function with argument of type (i32, i32) passing data of type i32` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/function_pointers/cast_fn_ptr2.rs:6
There were 1 unmatched diagnostics at tests/fail/function_pointers/cast_fn_ptr2.rs:6
    Error: Undefined Behavior: calling a function with argument of type Tuple([Int(I32), Int(I32)]) passing data of type Int(I32)
full stderr:
full stderr:
error: Undefined Behavior: calling a function with argument of type Tuple([Int(I32), Int(I32)]) passing data of type Int(I32)
   |
LL |     g(42)
LL |     g(42)
   |     ^^^^^ calling a function with argument of type Tuple([Int(I32), Int(I32)]) passing data of type Int(I32)
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/function_pointers/cast_fn_ptr2.rs:6:5: 6:10
---



tests/fail/function_pointers/cast_fn_ptr4.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/function_pointers/cast_fn_ptr4.rs"
actual output differed from expected
--- tests/fail/function_pointers/cast_fn_ptr4.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: calling a function with argument of type *const [i32] passing data of type *const i32
+error: Undefined Behavior: calling a function with argument of type RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not }) passing data of type RawPtr(TypeAndMut { ty: Int(I32), mutbl: Not })
    |
    |
 LL |     g(&42 as *const i32)
-   |     ^^^^^^^^^^^^^^^^^^^^ calling a function with argument of type *const [i32] passing data of type *const i32
+   |     ^^^^^^^^^^^^^^^^^^^^ calling a function with argument of type RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not }) passing data of type RawPtr(TypeAndMut { ty: Int(I32), mutbl: Not })
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `calling a function with argument of type *const [i32] passing data of type *const i32` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/function_pointers/cast_fn_ptr4.rs:6
There were 1 unmatched diagnostics at tests/fail/function_pointers/cast_fn_ptr4.rs:6
    Error: Undefined Behavior: calling a function with argument of type RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not }) passing data of type RawPtr(TypeAndMut { ty: Int(I32), mutbl: Not })
full stderr:
full stderr:
error: Undefined Behavior: calling a function with argument of type RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not }) passing data of type RawPtr(TypeAndMut { ty: Int(I32), mutbl: Not })
   |
   |
LL |     g(&42 as *const i32)
   |     ^^^^^^^^^^^^^^^^^^^^ calling a function with argument of type RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not }) passing data of type RawPtr(TypeAndMut { ty: Int(I32), mutbl: Not })
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/function_pointers/cast_fn_ptr4.rs:6:5: 6:25
---



tests/fail/function_pointers/cast_fn_ptr5.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/function_pointers/cast_fn_ptr5.rs"
actual output differed from expected
--- tests/fail/function_pointers/cast_fn_ptr5.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: calling a function with return type u32 passing return place of type ()
+error: Undefined Behavior: calling a function with return type Uint(U32) passing return place of type Tuple([])
    |
 LL |     g()
 LL |     g()
-   |     ^^^ calling a function with return type u32 passing return place of type ()
+   |     ^^^ calling a function with return type Uint(U32) passing return place of type Tuple([])
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `calling a function with return type u32 passing return place of type ()` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/function_pointers/cast_fn_ptr5.rs:8
There were 1 unmatched diagnostics at tests/fail/function_pointers/cast_fn_ptr5.rs:8
    Error: Undefined Behavior: calling a function with return type Uint(U32) passing return place of type Tuple([])
full stderr:
full stderr:
error: Undefined Behavior: calling a function with return type Uint(U32) passing return place of type Tuple([])
   |
LL |     g()
LL |     g()
   |     ^^^ calling a function with return type Uint(U32) passing return place of type Tuple([])
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/function_pointers/cast_fn_ptr5.rs:8:5: 8:8
---



tests/fail/intrinsics/float_to_int_32_inf1.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_32_inf1.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_32_inf1.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `i32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `Int(I32)`
    |
    |
 LL |         float_to_int_unchecked::<f32, i32>(f32::INFINITY);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `i32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `Int(I32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_32_inf1.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_32_inf1.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `Int(I32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `Int(I32)`
   |
   |
LL |         float_to_int_unchecked::<f32, i32>(f32::INFINITY);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `Int(I32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_32_inf1.rs:10:9: 10:58
---



tests/fail/intrinsics/float_to_int_32_neg.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_32_neg.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_32_neg.stderr
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `u32`
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `u32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `Uint(U32)`
    |
    |
 LL |         float_to_int_unchecked::<f32, u32>(-1.000000001f32);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `u32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `Uint(U32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_32_neg.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_32_neg.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `Uint(U32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `Uint(U32)`
   |
   |
LL |         float_to_int_unchecked::<f32, u32>(-1.000000001f32);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `Uint(U32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_32_neg.rs:10:9: 10:60
---



tests/fail/intrinsics/float_to_int_32_nanneg.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_32_nanneg.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_32_nanneg.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `u32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
    |
    |
 LL |         float_to_int_unchecked::<f32, u32>(-f32::NAN);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `u32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_32_nanneg.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_32_nanneg.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
   |
   |
LL |         float_to_int_unchecked::<f32, u32>(-f32::NAN);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_32_nanneg.rs:10:9: 10:54
---



tests/fail/intrinsics/float_to_int_32_infneg1.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_32_infneg1.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_32_infneg1.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `i32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Int(I32)`
    |
    |
 LL |         float_to_int_unchecked::<f32, i32>(f32::NEG_INFINITY);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `i32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Int(I32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_32_infneg1.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_32_infneg1.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Int(I32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Int(I32)`
   |
   |
LL |         float_to_int_unchecked::<f32, i32>(f32::NEG_INFINITY);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Int(I32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_32_infneg1.rs:10:9: 10:62
---



tests/fail/intrinsics/float_to_int_32_too_big1.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_32_too_big1.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_32_too_big1.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2.14748365E+9 which cannot be represented in target type `i32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2.14748365E+9 which cannot be represented in target type `Int(I32)`
    |
    |
 LL |         float_to_int_unchecked::<f32, i32>(2147483648.0f32);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 2.14748365E+9 which cannot be represented in target type `i32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 2.14748365E+9 which cannot be represented in target type `Int(I32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_32_too_big1.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_32_too_big1.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2.14748365E+9 which cannot be represented in target type `Int(I32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2.14748365E+9 which cannot be represented in target type `Int(I32)`
   |
   |
LL |         float_to_int_unchecked::<f32, i32>(2147483648.0f32);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 2.14748365E+9 which cannot be represented in target type `Int(I32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_32_too_big1.rs:10:9: 10:60
---



tests/fail/intrinsics/float_to_int_32_nan.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_32_nan.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_32_nan.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `u32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
    |
    |
 LL |         float_to_int_unchecked::<f32, u32>(f32::NAN);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `u32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_32_nan.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_32_nan.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
   |
   |
LL |         float_to_int_unchecked::<f32, u32>(f32::NAN);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_32_nan.rs:10:9: 10:53
---



tests/fail/intrinsics/float_to_int_32_too_big2.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_32_too_big2.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_32_too_big2.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 4.2949673E+9 which cannot be represented in target type `u32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 4.2949673E+9 which cannot be represented in target type `Uint(U32)`
    |
    |
 LL |         float_to_int_unchecked::<f32, u32>((u32::MAX - 127) as f32);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 4.2949673E+9 which cannot be represented in target type `u32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 4.2949673E+9 which cannot be represented in target type `Uint(U32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_32_too_big2.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_32_too_big2.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 4.2949673E+9 which cannot be represented in target type `Uint(U32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 4.2949673E+9 which cannot be represented in target type `Uint(U32)`
   |
   |
LL |         float_to_int_unchecked::<f32, u32>((u32::MAX - 127) as f32);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 4.2949673E+9 which cannot be represented in target type `Uint(U32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_32_too_big2.rs:10:9: 10:68
---



tests/fail/intrinsics/float_to_int_32_too_small1.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_32_too_small1.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_32_too_small1.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2.1474839E+9 which cannot be represented in target type `i32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2.1474839E+9 which cannot be represented in target type `Int(I32)`
    |
    |
 LL |         float_to_int_unchecked::<f32, i32>(-2147483904.0f32);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -2.1474839E+9 which cannot be represented in target type `i32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -2.1474839E+9 which cannot be represented in target type `Int(I32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_32_too_small1.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_32_too_small1.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2.1474839E+9 which cannot be represented in target type `Int(I32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2.1474839E+9 which cannot be represented in target type `Int(I32)`
   |
   |
LL |         float_to_int_unchecked::<f32, i32>(-2147483904.0f32);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -2.1474839E+9 which cannot be represented in target type `Int(I32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_32_too_small1.rs:10:9: 10:61
---



tests/fail/intrinsics/float_to_int_64_inf1.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_inf1.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_inf1.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `u128`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `Uint(U128)`
    |
    |
 LL |         float_to_int_unchecked::<f64, u128>(f64::INFINITY);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `u128`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `Uint(U128)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u128`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_inf1.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_inf1.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `Uint(U128)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `Uint(U128)`
   |
   |
LL |         float_to_int_unchecked::<f64, u128>(f64::INFINITY);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on +Inf which cannot be represented in target type `Uint(U128)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_inf1.rs:10:9: 10:59
---



tests/fail/intrinsics/float_to_int_64_infneg1.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_infneg1.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_infneg1.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `u128`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Uint(U128)`
    |
    |
 LL |         float_to_int_unchecked::<f64, u128>(f64::NEG_INFINITY);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `u128`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Uint(U128)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u128`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_infneg1.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_infneg1.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Uint(U128)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Uint(U128)`
   |
   |
LL |         float_to_int_unchecked::<f64, u128>(f64::NEG_INFINITY);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Uint(U128)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_infneg1.rs:10:9: 10:63
---



tests/fail/intrinsics/float_to_int_64_infneg2.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_infneg2.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_infneg2.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `i128`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Int(I128)`
    |
    |
 LL |         float_to_int_unchecked::<f64, i128>(f64::NEG_INFINITY);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `i128`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Int(I128)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i128`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_infneg2.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_infneg2.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Int(I128)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Int(I128)`
   |
   |
LL |         float_to_int_unchecked::<f64, i128>(f64::NEG_INFINITY);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -Inf which cannot be represented in target type `Int(I128)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_infneg2.rs:10:9: 10:63
---



tests/fail/intrinsics/float_to_int_64_nan.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_nan.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_nan.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `u32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
    |
    |
 LL |         float_to_int_unchecked::<f64, u32>(f64::NAN);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `u32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_nan.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_nan.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
   |
   |
LL |         float_to_int_unchecked::<f64, u32>(f64::NAN);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on NaN which cannot be represented in target type `Uint(U32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_nan.rs:10:9: 10:53
---



tests/fail/intrinsics/float_to_int_64_neg.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_neg.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_neg.stderr
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `u128`
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `u128`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `Uint(U128)`
    |
    |
 LL |         float_to_int_unchecked::<f64, u128>(-1.0000000000001f64);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `u128`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `Uint(U128)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u128`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_neg.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_neg.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `Uint(U128)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `Uint(U128)`
   |
   |
LL |         float_to_int_unchecked::<f64, u128>(-1.0000000000001f64);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -1 which cannot be represented in target type `Uint(U128)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_neg.rs:10:9: 10:65
---



tests/fail/intrinsics/float_to_int_64_too_big1.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_too_big1.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_too_big1.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2147483648 which cannot be represented in target type `i32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2147483648 which cannot be represented in target type `Int(I32)`
    |
    |
 LL |         float_to_int_unchecked::<f64, i32>(2147483648.0f64);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 2147483648 which cannot be represented in target type `i32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 2147483648 which cannot be represented in target type `Int(I32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_too_big1.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_too_big1.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2147483648 which cannot be represented in target type `Int(I32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2147483648 which cannot be represented in target type `Int(I32)`
   |
   |
LL |         float_to_int_unchecked::<f64, i32>(2147483648.0f64);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 2147483648 which cannot be represented in target type `Int(I32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_too_big1.rs:10:9: 10:60
---



tests/fail/intrinsics/float_to_int_64_too_big3.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_too_big3.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_too_big3.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 1.8446744073709552E+19 which cannot be represented in target type `u64`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 1.8446744073709552E+19 which cannot be represented in target type `Uint(U64)`
    |
    |
 LL |         float_to_int_unchecked::<f64, u64>(18446744073709551616.0f64);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 1.8446744073709552E+19 which cannot be represented in target type `u64`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 1.8446744073709552E+19 which cannot be represented in target type `Uint(U64)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u64`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_too_big3.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_too_big3.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 1.8446744073709552E+19 which cannot be represented in target type `Uint(U64)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 1.8446744073709552E+19 which cannot be represented in target type `Uint(U64)`
   |
   |
LL |         float_to_int_unchecked::<f64, u64>(18446744073709551616.0f64);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 1.8446744073709552E+19 which cannot be represented in target type `Uint(U64)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_too_big3.rs:10:9: 10:70
---



tests/fail/intrinsics/float_to_int_64_too_big2.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_too_big2.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_too_big2.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 9.2233720368547758E+18 which cannot be represented in target type `i64`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 9.2233720368547758E+18 which cannot be represented in target type `Int(I64)`
    |
    |
 LL |         float_to_int_unchecked::<f64, i64>(9223372036854775808.0f64);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 9.2233720368547758E+18 which cannot be represented in target type `i64`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 9.2233720368547758E+18 which cannot be represented in target type `Int(I64)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i64`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_too_big2.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_too_big2.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 9.2233720368547758E+18 which cannot be represented in target type `Int(I64)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 9.2233720368547758E+18 which cannot be represented in target type `Int(I64)`
   |
   |
LL |         float_to_int_unchecked::<f64, i64>(9223372036854775808.0f64);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 9.2233720368547758E+18 which cannot be represented in target type `Int(I64)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_too_big2.rs:10:9: 10:69
---



tests/fail/intrinsics/float_to_int_64_too_big5.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_too_big5.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_too_big5.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2.4028236692093845E+38 which cannot be represented in target type `i128`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2.4028236692093845E+38 which cannot be represented in target type `Int(I128)`
    |
    |
 LL |         float_to_int_unchecked::<f64, i128>(240282366920938463463374607431768211455.0f64);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 2.4028236692093845E+38 which cannot be represented in target type `i128`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 2.4028236692093845E+38 which cannot be represented in target type `Int(I128)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i128`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_too_big5.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_too_big5.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2.4028236692093845E+38 which cannot be represented in target type `Int(I128)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 2.4028236692093845E+38 which cannot be represented in target type `Int(I128)`
   |
   |
LL |         float_to_int_unchecked::<f64, i128>(240282366920938463463374607431768211455.0f64);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 2.4028236692093845E+38 which cannot be represented in target type `Int(I128)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_too_big5.rs:10:9: 10:90
---



tests/fail/intrinsics/float_to_int_64_too_big7.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_too_big7.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_too_big7.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1.7976931348623157E+308 which cannot be represented in target type `i128`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1.7976931348623157E+308 which cannot be represented in target type `Int(I128)`
    |
    |
 LL |         float_to_int_unchecked::<f64, i128>(f64::MIN);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -1.7976931348623157E+308 which cannot be represented in target type `i128`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -1.7976931348623157E+308 which cannot be represented in target type `Int(I128)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i128`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_too_big7.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_too_big7.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1.7976931348623157E+308 which cannot be represented in target type `Int(I128)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -1.7976931348623157E+308 which cannot be represented in target type `Int(I128)`
   |
   |
LL |         float_to_int_unchecked::<f64, i128>(f64::MIN);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -1.7976931348623157E+308 which cannot be represented in target type `Int(I128)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_too_big7.rs:10:9: 10:54
---



tests/fail/intrinsics/float_to_int_64_too_small1.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_too_small1.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_too_small1.stderr
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2147483649 which cannot be represented in target type `i32`
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2147483649 which cannot be represented in target type `i32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2147483649 which cannot be represented in target type `Int(I32)`
    |
    |
 LL |         float_to_int_unchecked::<f64, i32>(-2147483649.0f64);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -2147483649 which cannot be represented in target type `i32`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -2147483649 which cannot be represented in target type `Int(I32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_too_small1.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_too_small1.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2147483649 which cannot be represented in target type `Int(I32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2147483649 which cannot be represented in target type `Int(I32)`
   |
   |
LL |         float_to_int_unchecked::<f64, i32>(-2147483649.0f64);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -2147483649 which cannot be represented in target type `Int(I32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_too_small1.rs:10:9: 10:61
---



tests/fail/intrinsics/float_to_int_64_too_big6.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_too_big6.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_too_big6.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 1.7976931348623157E+308 which cannot be represented in target type `u128`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 1.7976931348623157E+308 which cannot be represented in target type `Uint(U128)`
    |
    |
 LL |         float_to_int_unchecked::<f64, u128>(f64::MAX);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 1.7976931348623157E+308 which cannot be represented in target type `u128`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 1.7976931348623157E+308 which cannot be represented in target type `Uint(U128)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u128`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_too_big6.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_too_big6.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 1.7976931348623157E+308 which cannot be represented in target type `Uint(U128)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 1.7976931348623157E+308 which cannot be represented in target type `Uint(U128)`
   |
   |
LL |         float_to_int_unchecked::<f64, u128>(f64::MAX);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 1.7976931348623157E+308 which cannot be represented in target type `Uint(U128)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_too_big6.rs:10:9: 10:54
---



tests/fail/intrinsics/float_to_int_64_too_big4.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_too_big4.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_too_big4.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.4028236692093846E+38 which cannot be represented in target type `u128`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.4028236692093846E+38 which cannot be represented in target type `Uint(U128)`
    |
    |
 LL |         float_to_int_unchecked::<f64, u128>(u128::MAX as f64);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 3.4028236692093846E+38 which cannot be represented in target type `u128`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 3.4028236692093846E+38 which cannot be represented in target type `Uint(U128)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `u128`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_too_big4.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_too_big4.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.4028236692093846E+38 which cannot be represented in target type `Uint(U128)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.4028236692093846E+38 which cannot be represented in target type `Uint(U128)`
   |
   |
LL |         float_to_int_unchecked::<f64, u128>(u128::MAX as f64);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 3.4028236692093846E+38 which cannot be represented in target type `Uint(U128)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_too_big4.rs:10:9: 10:62
---



tests/fail/intrinsics/float_to_int_64_too_small2.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_too_small2.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_too_small2.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -9.2233720368547778E+18 which cannot be represented in target type `i64`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -9.2233720368547778E+18 which cannot be represented in target type `Int(I64)`
    |
    |
 LL |         float_to_int_unchecked::<f64, i64>(-9223372036854777856.0f64);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -9.2233720368547778E+18 which cannot be represented in target type `i64`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -9.2233720368547778E+18 which cannot be represented in target type `Int(I64)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i64`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_too_small2.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_too_small2.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -9.2233720368547778E+18 which cannot be represented in target type `Int(I64)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -9.2233720368547778E+18 which cannot be represented in target type `Int(I64)`
   |
   |
LL |         float_to_int_unchecked::<f64, i64>(-9223372036854777856.0f64);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -9.2233720368547778E+18 which cannot be represented in target type `Int(I64)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_too_small2.rs:10:9: 10:70
---



tests/fail/intrinsics/float_to_int_64_too_small3.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/float_to_int_64_too_small3.rs"
actual output differed from expected
--- tests/fail/intrinsics/float_to_int_64_too_small3.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2.4028236692093845E+38 which cannot be represented in target type `i128`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2.4028236692093845E+38 which cannot be represented in target type `Int(I128)`
    |
    |
 LL |         float_to_int_unchecked::<f64, i128>(-240282366920938463463374607431768211455.0f64);
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -2.4028236692093845E+38 which cannot be represented in target type `i128`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -2.4028236692093845E+38 which cannot be represented in target type `Int(I128)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `cannot be represented in target type `i128`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/float_to_int_64_too_small3.rs:10

There were 1 unmatched diagnostics at tests/fail/intrinsics/float_to_int_64_too_small3.rs:10
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2.4028236692093845E+38 which cannot be represented in target type `Int(I128)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on -2.4028236692093845E+38 which cannot be represented in target type `Int(I128)`
   |
   |
LL |         float_to_int_unchecked::<f64, i128>(-240282366920938463463374607431768211455.0f64);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on -2.4028236692093845E+38 which cannot be represented in target type `Int(I128)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/intrinsics/float_to_int_64_too_small3.rs:10:9: 10:91
---



tests/fail/intrinsics/simd-float-to-int.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "tests/fail/intrinsics/simd-float-to-int.rs"
actual output differed from expected
--- tests/fail/intrinsics/simd-float-to-int.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `i32`
+error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `Int(I32)`
    |
 LL |         unsafe { intrinsics::simd_cast(self) }
 LL |         unsafe { intrinsics::simd_cast(self) }
-   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `i32`
+   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `Int(I32)`
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 14 lines skipped ...



substring `cannot be represented in target type `i32`` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/simd-float-to-int.rs:1

There were 1 unmatched diagnostics that occurred outside the testfile and had no pattern
    Error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `Int(I32)`
full stderr:
full stderr:
error: Undefined Behavior: `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `Int(I32)`
   |
LL |         unsafe { intrinsics::simd_cast(self) }
LL |         unsafe { intrinsics::simd_cast(self) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `float_to_int_unchecked` intrinsic called on 3.40282347E+38 which cannot be represented in target type `Int(I32)`
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: BACKTRACE:
   = note: inside `std::simd::Simd::<f32, 2>::to_int_unchecked::<i32>` at /checkout/library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:245:18: 245:45
  --> tests/fail/intrinsics/simd-float-to-int.rs:7:25
   |
   |
LL |         let _x: i32x2 = f32x2::from_array([f32::MAX, f32::MIN]).to_int_unchecked();

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error
