plain
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.36
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unnecessary `unsafe` block
    |
    |
338 |     unsafe { intrinsics::size_of_val(val) }
    |     ^^^^^^ unnecessary `unsafe` block
    |
    = note: `-D unused-unsafe` implied by `-D warnings`

error: unnecessary `unsafe` block
    |
    |
387 |     unsafe { intrinsics::size_of_val(val) }
    |     ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
    |
    |
432 |     unsafe { intrinsics::min_align_of_val(val) }
    |     ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
    |
    |
477 |     unsafe { intrinsics::min_align_of_val(val) }
    |     ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
    |
    |
522 |     unsafe { intrinsics::min_align_of_val(val) }
    |     ^^^^^^ unnecessary `unsafe` block
error: aborting due to 5 previous errors

error: could not compile `core`

