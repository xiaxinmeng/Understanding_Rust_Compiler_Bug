plain

actual output differed from expected tests/fail/backtrace/bad-backtrace-ptr.stderr
Diff < left / right > :
<error: Undefined Behavior: null pointer is not a valid pointer for this operation
>error: Undefined Behavior: out-of-bounds pointer use: null pointer is a dangling pointer (it has no provenance)
    |
 LL |         miri_resolve_frame(std::ptr::null_mut(), 0);
<   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ null pointer is not a valid pointer for this operation
<   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ null pointer is not a valid pointer for this operation
>   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer use: null pointer is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `main` at $DIR/bad-backtrace-ptr.rs:LL:CC
---

`null pointer is not a valid pointer for this operation` not found in stderr output
expected because of pattern here: tests/fail/backtrace/bad-backtrace-ptr.rs:7

There were 1 unmatched diagnostics at tests/fail/backtrace/bad-backtrace-ptr.rs:7
    Error: Undefined Behavior: out-of-bounds pointer use: null pointer is a dangling pointer (it has no provenance)

tests/fail/dangling_pointers/deref-invalid-ptr.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/dangling_pointers/deref-invalid-ptr.rs" "--error-format=json" "-Zmiri-disable-validation" "-Zmiri-permissive-provenance"


actual output differed from expected tests/fail/dangling_pointers/deref-invalid-ptr.stderr
Diff < left / right > :
<error: Undefined Behavior: dereferencing pointer failed: 0x10 is not a valid pointer
>error: Undefined Behavior: dereferencing pointer failed: 0x10[noalloc] is a dangling pointer (it has no provenance)
    |
    |
 LL |     let _y = unsafe { &*x as *const u32 };
<   |                       ^^^ dereferencing pointer failed: 0x10 is not a valid pointer
>   |                       ^^^ dereferencing pointer failed: 0x10[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `main` at $DIR/deref-invalid-ptr.rs:LL:CC
---

`is not a valid pointer` not found in stderr output
expected because of pattern here: tests/fail/dangling_pointers/deref-invalid-ptr.rs:6

There were 1 unmatched diagnostics at tests/fail/dangling_pointers/deref-invalid-ptr.rs:6
    Error: Undefined Behavior: dereferencing pointer failed: 0x10[noalloc] is a dangling pointer (it has no provenance)

tests/fail/dangling_pointers/null_pointer_deref.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/dangling_pointers/null_pointer_deref.rs" "--error-format=json"


actual output differed from expected tests/fail/dangling_pointers/null_pointer_deref.stderr
Diff < left / right > :
<error: Undefined Behavior: dereferencing pointer failed: null pointer is not a valid pointer
>error: Undefined Behavior: dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   --> $DIR/null_pointer_deref.rs:LL:CC
    |
 LL |     let x: i32 = unsafe { *std::ptr::null() };
<   |                           ^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is not a valid pointer
>   |                           ^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `main` at $DIR/null_pointer_deref.rs:LL:CC
---
<error: Undefined Behavior: dereferencing pointer failed: null pointer is not a valid pointer
>error: Undefined Behavior: dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   --> $DIR/null_pointer_deref_zst.rs:LL:CC
    |
 LL |     let x: () = unsafe { *std::ptr::null() };
<   |                          ^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is not a valid pointer
>   |                          ^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `main` at $DIR/null_pointer_deref_zst.rs:LL:CC
---
    = note: inside `std::ptr::mut_ptr::<impl *mut [u8; 0]>::write` at RUSTLIB/core/src/ptr/mut_ptr.rs:LL:CC
 note: inside `main` at $DIR/null_pointer_write_zst.rs:LL:CC
   --> $DIR/null_pointer_write_zst.rs:LL:CC
    |
 LL |     unsafe { std::ptr::null_mut::<[u8; 0]>().write(zst_val) };
 
 note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
 
 error: aborting due to previous error
---
<error: Undefined Behavior: dereferencing pointer failed: null pointer is not a valid pointer
>error: Undefined Behavior: dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
   --> $DIR/null_pointer_write.rs:LL:CC
    |
 LL |     unsafe { *std::ptr::null_mut() = 0i32 };
<   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is not a valid pointer
>   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: null pointer is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `main` at $DIR/null_pointer_write.rs:LL:CC
---

actual output differed from expected tests/fail/dangling_pointers/wild_pointer_deref.stderr
Diff < left / right > :
<error: Undefined Behavior: dereferencing pointer failed: 0x2c is not a valid pointer
>error: Undefined Behavior: dereferencing pointer failed: 0x2c[noalloc] is a dangling pointer (it has no provenance)
   --> $DIR/wild_pointer_deref.rs:LL:CC
    |
 LL |     let x = unsafe { *p };
<   |                      ^^ dereferencing pointer failed: 0x2c is not a valid pointer
>   |                      ^^ dereferencing pointer failed: 0x2c[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `main` at $DIR/wild_pointer_deref.rs:LL:CC
---

`is not a valid pointer` not found in stderr output
expected because of pattern here: tests/fail/dangling_pointers/wild_pointer_deref.rs:5

There were 1 unmatched diagnostics at tests/fail/dangling_pointers/wild_pointer_deref.rs:5
    Error: Undefined Behavior: dereferencing pointer failed: 0x2c[noalloc] is a dangling pointer (it has no provenance)

tests/fail/dangling_pointers/storage_dead_dangling.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/dangling_pointers/storage_dead_dangling.rs" "--error-format=json" "-Zmiri-disable-validation" "-Zmir-opt-level=0" "-Zmiri-permissive-provenance"


actual output differed from expected tests/fail/dangling_pointers/storage_dead_dangling.stderr
Diff < left / right > :
<error: Undefined Behavior: dereferencing pointer failed: $HEX is not a valid pointer
>error: Undefined Behavior: dereferencing pointer failed: $HEX[noalloc] is a dangling pointer (it has no provenance)
    |
    |
 LL |     unsafe { &mut *(LEAK as *mut i32) };
<   |              ^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: $HEX is not a valid pointer
>   |              ^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: $HEX[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: backtrace:
    = note: inside `evil` at $DIR/storage_dead_dangling.rs:LL:CC
   --> $DIR/storage_dead_dangling.rs:LL:CC
    |
    |
 LL |     evil();
 
 note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
 
 error: aborting due to previous error
---

`is not a valid pointer` not found in stderr output
expected because of pattern here: tests/fail/dangling_pointers/storage_dead_dangling.rs:13

There were 1 unmatched diagnostics at tests/fail/dangling_pointers/storage_dead_dangling.rs:13
    Error: Undefined Behavior: dereferencing pointer failed: 0x28610[noalloc] is a dangling pointer (it has no provenance)

tests/fail/function_pointers/cast_int_to_fn_ptr.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/function_pointers/cast_int_to_fn_ptr.rs" "--error-format=json" "-Zmiri-disable-validation"


actual output differed from expected tests/fail/function_pointers/cast_int_to_fn_ptr.stderr
Diff < left / right > :
<error: Undefined Behavior: 0x2a is not a valid pointer
>error: Undefined Behavior: out-of-bounds pointer use: 0x2a[noalloc] is a dangling pointer (it has no provenance)
    |
 LL |     g(42)
<   |     ^^^^^ 0x2a is not a valid pointer
<   |     ^^^^^ 0x2a is not a valid pointer
>   |     ^^^^^ out-of-bounds pointer use: 0x2a[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `main` at $DIR/cast_int_to_fn_ptr.rs:LL:CC
---

`not a valid pointer` not found in stderr output
expected because of pattern here: tests/fail/function_pointers/cast_int_to_fn_ptr.rs:7

There were 1 unmatched diagnostics at tests/fail/function_pointers/cast_int_to_fn_ptr.rs:7
    Error: Undefined Behavior: out-of-bounds pointer use: 0x2a[noalloc] is a dangling pointer (it has no provenance)

tests/fail/intrinsics/copy_null.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/intrinsics/copy_null.rs" "--error-format=json"

---
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/intrinsics/out_of_bounds_ptr_1.rs" "--error-format=json"

actual output differed from expected tests/fail/intrinsics/out_of_bounds_ptr_1.stderr
Diff < left / right > :
<error: Undefined Behavior: pointer arithmetic failed: ALLOC has size 4, so pointer to 5 bytes starting at offset 0 is out-of-bounds
>error: Undefined Behavior: out-of-bounds pointer arithmetic: ALLOC has size 4, so pointer to 5 bytes starting at offset 0 is out-of-bounds
    |
 LL |         unsafe { intrinsics::offset(self, count) }
 LL |         unsafe { intrinsics::offset(self, count) }
<   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer arithmetic failed: ALLOC has size 4, so pointer to 5 bytes starting at offset 0 is out-of-bounds
>   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: ALLOC has size 4, so pointer to 5 bytes starting at offset 0 is out-of-bounds
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::ptr::const_ptr::<impl *const i8>::offset` at RUSTLIB/core/src/ptr/const_ptr.rs:LL:CC
---
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/intrinsics/out_of_bounds_ptr_3.rs" "--error-format=json"

actual output differed from expected tests/fail/intrinsics/out_of_bounds_ptr_3.stderr
Diff < left / right > :
<error: Undefined Behavior: pointer arithmetic failed: ALLOC has size 4, so pointer to 1 byte starting at offset -1 is out-of-bounds
>error: Undefined Behavior: out-of-bounds pointer arithmetic: ALLOC has size 4, so pointer to 1 byte starting at offset -1 is out-of-bounds
    |
 LL |         unsafe { intrinsics::offset(self, count) }
 LL |         unsafe { intrinsics::offset(self, count) }
<   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer arithmetic failed: ALLOC has size 4, so pointer to 1 byte starting at offset -1 is out-of-bounds
>   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: ALLOC has size 4, so pointer to 1 byte starting at offset -1 is out-of-bounds
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::ptr::const_ptr::<impl *const i8>::offset` at RUSTLIB/core/src/ptr/const_ptr.rs:LL:CC
---

actual output differed from expected tests/fail/intrinsics/ptr_offset_0_plus_0.stderr
Diff < left / right > :
<error: Undefined Behavior: pointer arithmetic failed: null pointer is not a valid pointer
>error: Undefined Behavior: out-of-bounds pointer arithmetic: null pointer is a dangling pointer (it has no provenance)
    |
 LL |         unsafe { intrinsics::offset(self, count) as *mut T }
<   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer arithmetic failed: null pointer is not a valid pointer
<   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer arithmetic failed: null pointer is not a valid pointer
>   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: null pointer is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::ptr::mut_ptr::<impl *mut i32>::offset` at RUSTLIB/core/src/ptr/mut_ptr.rs:LL:CC
---
`pointer arithmetic failed: null pointer is not a valid pointer` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/ptr_offset_0_plus_0.rs:1

There were 1 unmatched diagnostics that occurred outside the testfile and had not pattern
    Error: Undefined Behavior: out-of-bounds pointer arithmetic: null pointer is a dangling pointer (it has no provenance)

tests/fail/intrinsics/ptr_offset_int_plus_int.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/intrinsics/ptr_offset_int_plus_int.rs" "--error-format=json" "-Zmiri-permissive-provenance"


actual output differed from expected tests/fail/intrinsics/ptr_offset_int_plus_int.stderr
Diff < left / right > :
<error: Undefined Behavior: pointer arithmetic failed: 0x1 is not a valid pointer
>error: Undefined Behavior: out-of-bounds pointer arithmetic: 0x1[noalloc] is a dangling pointer (it has no provenance)
    |
 LL |         unsafe { intrinsics::offset(self, count) as *mut T }
<   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer arithmetic failed: 0x1 is not a valid pointer
<   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer arithmetic failed: 0x1 is not a valid pointer
>   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: 0x1[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::ptr::mut_ptr::<impl *mut u8>::offset` at RUSTLIB/core/src/ptr/mut_ptr.rs:LL:CC
---
`is not a valid pointer` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/ptr_offset_int_plus_int.rs:1

There were 1 unmatched diagnostics that occurred outside the testfile and had not pattern
    Error: Undefined Behavior: out-of-bounds pointer arithmetic: 0x1[noalloc] is a dangling pointer (it has no provenance)

tests/fail/intrinsics/ptr_offset_int_plus_ptr.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/intrinsics/ptr_offset_int_plus_ptr.rs" "--error-format=json" "-Zmiri-permissive-provenance"


actual output differed from expected tests/fail/intrinsics/ptr_offset_int_plus_ptr.stderr
Diff < left / right > :
<error: Undefined Behavior: pointer arithmetic failed: 0x1 is not a valid pointer
>error: Undefined Behavior: out-of-bounds pointer arithmetic: 0x1[noalloc] is a dangling pointer (it has no provenance)
    |
 LL |         unsafe { intrinsics::offset(self, count) as *mut T }
<   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer arithmetic failed: 0x1 is not a valid pointer
<   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer arithmetic failed: 0x1 is not a valid pointer
>   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: 0x1[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::ptr::mut_ptr::<impl *mut u8>::offset` at RUSTLIB/core/src/ptr/mut_ptr.rs:LL:CC
---
`is not a valid pointer` not found in stderr output
expected because of pattern here: tests/fail/intrinsics/ptr_offset_int_plus_ptr.rs:1

There were 1 unmatched diagnostics that occurred outside the testfile and had not pattern
    Error: Undefined Behavior: out-of-bounds pointer arithmetic: 0x1[noalloc] is a dangling pointer (it has no provenance)

tests/fail/intrinsics/ptr_offset_ptr_plus_0.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/intrinsics/ptr_offset_ptr_plus_0.rs" "--error-format=json"


actual output differed from expected tests/fail/intrinsics/ptr_offset_ptr_plus_0.stderr
Diff < left / right > :
<error: Undefined Behavior: pointer arithmetic failed: ALLOC has size 4, so pointer at offset 32 is out-of-bounds
>error: Undefined Behavior: out-of-bounds pointer arithmetic: ALLOC has size 4, so pointer at offset 32 is out-of-bounds
    |
 LL |         unsafe { intrinsics::offset(self, count) as *mut T }
 LL |         unsafe { intrinsics::offset(self, count) as *mut T }
<   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer arithmetic failed: ALLOC has size 4, so pointer at offset 32 is out-of-bounds
>   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: ALLOC has size 4, so pointer at offset 32 is out-of-bounds
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::ptr::mut_ptr::<impl *mut u32>::offset` at RUSTLIB/core/src/ptr/mut_ptr.rs:LL:CC
    = note: inside `std::ptr::mut_ptr::<impl *mut u32>::offset` at RUSTLIB/core/src/ptr/mut_ptr.rs:LL:CC
 note: inside `main` at $DIR/ptr_offset_ptr_plus_0.rs:LL:CC
   --> $DIR/ptr_offset_ptr_plus_0.rs:LL:CC
    |
 LL |     let _x = unsafe { x.offset(0) }; // UB despite offset 0, the pointer is not inbounds of the only object it can point to
 
 note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
 
 error: aborting due to previous error
---
<error: Undefined Behavior: memory access failed: null pointer is not a valid pointer
>error: Undefined Behavior: memory access failed: null pointer is a dangling pointer (it has no provenance)
   --> $DIR/write_bytes_null.rs:LL:CC
    |
 LL |     unsafe { write_bytes::<u8>(std::ptr::null_mut(), 0, 0) };
>   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ memory access failed: null pointer is a dangling pointer (it has no provenance)
    |
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
---
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/provenance/provenance_transmute.rs" "--error-format=json" "-Zmiri-permissive-provenance"

actual output differed from expected tests/fail/provenance/provenance_transmute.stderr
Diff < left / right > :
<error: Undefined Behavior: dereferencing pointer failed: $HEX is not a valid pointer
>error: Undefined Behavior: dereferencing pointer failed: $HEX[noalloc] is a dangling pointer (it has no provenance)
    |
    |
 LL |         let _val = *left_ptr;
<   |                    ^^^^^^^^^ dereferencing pointer failed: $HEX is not a valid pointer
>   |                    ^^^^^^^^^ dereferencing pointer failed: $HEX[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `deref` at $DIR/provenance_transmute.rs:LL:CC
    = note: inside `deref` at $DIR/provenance_transmute.rs:LL:CC
 note: inside `main` at $DIR/provenance_transmute.rs:LL:CC
   --> $DIR/provenance_transmute.rs:LL:CC
    |
 LL |         deref(ptr1, ptr2.with_addr(ptr1.addr()));
 
 note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
 
 error: aborting due to previous error
---
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/provenance/ptr_int_unexposed.rs" "--error-format=json" "-Zmiri-permissive-provenance"

actual output differed from expected tests/fail/provenance/ptr_int_unexposed.stderr
Diff < left / right > :
<error: Undefined Behavior: dereferencing pointer failed: $HEX is not a valid pointer
>error: Undefined Behavior: dereferencing pointer failed: $HEX[noalloc] is a dangling pointer (it has no provenance)
    |
    |
 LL |     assert_eq!(unsafe { *ptr }, 3);
<   |                         ^^^^ dereferencing pointer failed: $HEX is not a valid pointer
>   |                         ^^^^ dereferencing pointer failed: $HEX[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `main` at $DIR/ptr_int_unexposed.rs:LL:CC
---
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/provenance/ptr_invalid.rs" "--error-format=json"

actual output differed from expected tests/fail/provenance/ptr_invalid.stderr
Diff < left / right > :
<error: Undefined Behavior: dereferencing pointer failed: $HEX is not a valid pointer
>error: Undefined Behavior: dereferencing pointer failed: $HEX[noalloc] is a dangling pointer (it has no provenance)
    |
    |
 LL |     let _val = unsafe { *xptr_invalid };
<   |                         ^^^^^^^^^^^^^ dereferencing pointer failed: $HEX is not a valid pointer
>   |                         ^^^^^^^^^^^^^ dereferencing pointer failed: $HEX[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `main` at $DIR/ptr_invalid.rs:LL:CC
---

`is not a valid pointer` not found in stderr output
expected because of pattern here: tests/fail/provenance/ptr_invalid.rs:8

There were 1 unmatched diagnostics at tests/fail/provenance/ptr_invalid.rs:8
    Error: Undefined Behavior: dereferencing pointer failed: 0x285f0[noalloc] is a dangling pointer (it has no provenance)

tests/fail/provenance/ptr_invalid_offset.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/provenance/ptr_invalid_offset.rs" "--error-format=json" "-Zmiri-strict-provenance"


actual output differed from expected tests/fail/provenance/ptr_invalid_offset.stderr
Diff < left / right > :
<error: Undefined Behavior: pointer arithmetic failed: $HEX is not a valid pointer
>error: Undefined Behavior: out-of-bounds pointer arithmetic: $HEX[noalloc] is a dangling pointer (it has no provenance)
    |
 LL |         unsafe { intrinsics::offset(self, count) }
 LL |         unsafe { intrinsics::offset(self, count) }
<   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pointer arithmetic failed: $HEX is not a valid pointer
>   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer arithmetic: $HEX[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::ptr::const_ptr::<impl *const u8>::offset` at RUSTLIB/core/src/ptr/const_ptr.rs:LL:CC
    = note: inside `std::ptr::const_ptr::<impl *const u8>::offset` at RUSTLIB/core/src/ptr/const_ptr.rs:LL:CC
 note: inside `main` at $DIR/ptr_invalid_offset.rs:LL:CC
   --> $DIR/ptr_invalid_offset.rs:LL:CC
    |
 LL |     let _ = unsafe { roundtrip.offset(1) };
 
 note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
 
 error: aborting due to previous error
---
`not a valid pointer` not found in stderr output
expected because of pattern here: tests/fail/provenance/ptr_invalid_offset.rs:2

There were 1 unmatched diagnostics that occurred outside the testfile and had not pattern
    Error: Undefined Behavior: out-of-bounds pointer arithmetic: 0x28608[noalloc] is a dangling pointer (it has no provenance)

tests/fail/stacked_borrows/issue-miri-1050-2.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/stacked_borrows/issue-miri-1050-2.rs" "--error-format=json"


actual output differed from expected tests/fail/stacked_borrows/issue-miri-1050-2.stderr
Diff < left / right > :
<error: Undefined Behavior: 0x4 is not a valid pointer
>error: Undefined Behavior: out-of-bounds pointer use: 0x4[noalloc] is a dangling pointer (it has no provenance)
    |
 LL |         Box(unsafe { Unique::new_unchecked(raw) }, alloc)
<   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ 0x4 is not a valid pointer
<   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ 0x4 is not a valid pointer
>   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer use: 0x4[noalloc] is a dangling pointer (it has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::boxed::Box::<i32>::from_raw_in` at RUSTLIB/alloc/src/boxed.rs:LL:CC
---
`is not a valid pointer` not found in stderr output
expected because of pattern here: tests/fail/stacked_borrows/issue-miri-1050-2.rs:1

There were 1 unmatched diagnostics that occurred outside the testfile and had not pattern
    Error: Undefined Behavior: out-of-bounds pointer use: 0x4[noalloc] is a dangling pointer (it has no provenance)

tests/fail/stacked_borrows/issue-miri-1050-1.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/fail/stacked_borrows/issue-miri-1050-1.rs" "--error-format=json"


actual output differed from expected tests/fail/stacked_borrows/issue-miri-1050-1.stderr
Diff < left / right > :
<error: Undefined Behavior: ALLOC has size 2, so pointer to 4 bytes starting at offset 0 is out-of-bounds
>error: Undefined Behavior: out-of-bounds pointer use: ALLOC has size 2, so pointer to 4 bytes starting at offset 0 is out-of-bounds
    |
 LL |         Box(unsafe { Unique::new_unchecked(raw) }, alloc)
 LL |         Box(unsafe { Unique::new_unchecked(raw) }, alloc)
<   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ALLOC has size 2, so pointer to 4 bytes starting at offset 0 is out-of-bounds
>   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ out-of-bounds pointer use: ALLOC has size 2, so pointer to 4 bytes starting at offset 0 is out-of-bounds
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
    = note: backtrace:
    = note: inside `std::boxed::Box::<u32>::from_raw_in` at RUSTLIB/alloc/src/boxed.rs:LL:CC
---
.......... (60/66)
.....     (66/66)


/checkout/src/test/rustdoc-gui/search-tab-change-title-fn-sig.goml search-tab-change-title-fn-sig... FAILED
[ERROR] (line 6) Error: The following CSS selector "#titles" was not found: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:46
