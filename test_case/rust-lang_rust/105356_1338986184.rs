plain
........................................................................................ 176/3985
........................................................................................ 264/3985
.............................ii......................................iii.......i........ 352/3985
........................................................................................ 440/3985
........................................F..F........i................................... 528/3985
.......................................................................................i 704/3985
iii..................................................................................... 792/3985
........................................................................................ 880/3985
........................................................................................ 968/3985
---
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---- src/intrinsics/mir.rs - intrinsics::mir::Field (line 271) stdout ----
error[E0107]: this function takes 1 generic argument but 2 generic arguments were supplied
    |
    |
12  |         RET = *Field::<_, &i32>(Variant(opt, 1), 0);
    |                ^^^^^      ---- help: remove this generic argument
    |                expected 1 generic argument
    |
    |
note: function defined here, with 1 generic parameter: `F`
    |
    |
295 |     fn Field<F>(place: (), field: u32) -> F

error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
---
    src/intrinsics/mir.rs - intrinsics::mir::Field (line 271)

test result: FAILED. 3947 passed; 2 failed; 36 ignored; 0 measured; 0 filtered out; finished in 69.47s

error: doctest failed, to rerun pass `-p core --doc`
