plain
test src/unit.rs - unit::() (line 8) ... ok

failures:

---- src/intrinsics/mir.rs - intrinsics::mir (line 62) stdout ----
error: Could not parse terminator, found: "Match { scrutinee: e1, arms: [a0, a1] }"
   |
13 | /             match c {
14 | |                 true => t,
15 | |                 _ => f,
15 | |                 _ => f,
16 | |             }
   | |_____________^

error: Could not parse local, found: "Call { ty: fn((), u32) -> T {std::intrinsics::mir::Field::<T>}, fun: e5, args: [e13, e15], from_hir_call: true, fn_span: src/intrinsics/mir.rs:39:20: 39:45 (#0) }"
   |
   |
39 |         RET = Move(Field(Variant(opt, 1), 0));

error: aborting due to 2 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- src/intrinsics/mir.rs - intrinsics::mir::Field (line 258) stdout ----
error: Could not parse local, found: "Call { ty: fn((), u32) -> &i32 {std::intrinsics::mir::Field::<&i32>}, fun: e3, args: [e11, e13], from_hir_call: true, fn_span: src/intrinsics/mir.rs:12:16: 12:49 (#0) }"
   |
   |
12 |         RET = *Field::<&i32>(Variant(opt, 1), 0);


error: Could not parse local, found: "Call { ty: fn(i32) -> *mut i32 {std::intrinsics::mir::__internal_make_place::<i32>}, fun: e1, args: [e17], from_hir_call: true, fn_span: /checkout/library/core/src/intrinsics/mir.rs:357:7: 357:57 (#28) }"
   |
   |
20 |         place!(Field(Variant(*opt, 1), 0)) = 5;
   |
   = note: this error originates in the macro `place` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
---
    src/intrinsics/mir.rs - intrinsics::mir::Field (line 258)

test result: FAILED. 3949 passed; 2 failed; 36 ignored; 0 measured; 0 filtered out; finished in 55.87s

error: doctest failed, to rerun pass `-p core --doc`
