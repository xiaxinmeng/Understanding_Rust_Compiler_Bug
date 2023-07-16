plain
    
    --- stdout
    
    running 8 tests
    test /tmp/mdbook-xawLyJ/c-tips/index.md - Tips_for_embedded_C_developers::Iterators_vs_Array_Access (line 183) ... ignored
    test /tmp/mdbook-xawLyJ/c-tips/index.md - Tips_for_embedded_C_developers::Volatile_Access (line 262) ... ignored
    test /tmp/mdbook-xawLyJ/c-tips/index.md - Tips_for_embedded_C_developers::Packed_and_Aligned_Types (line 350) ... FAILED
    test /tmp/mdbook-xawLyJ/c-tips/index.md - Tips_for_embedded_C_developers::Preprocessor::Compile_Time_Sizes_and_Computation (line 78) ... ok
    test /tmp/mdbook-xawLyJ/c-tips/index.md - Tips_for_embedded_C_developers::Preprocessor::Compile_Time_Code_Selection (line 44) ... ok
    test /tmp/mdbook-xawLyJ/c-tips/index.md - Tips_for_embedded_C_developers::Packed_and_Aligned_Types (line 312) ... ok
    test /tmp/mdbook-xawLyJ/c-tips/index.md - Tips_for_embedded_C_developers::Packed_and_Aligned_Types (line 330) ... ok
    test /tmp/mdbook-xawLyJ/c-tips/index.md - Tips_for_embedded_C_developers::Packed_and_Aligned_Types (line 373) ... ok
    failures:
    
    
    ---- /tmp/mdbook-xawLyJ/c-tips/index.md - Tips_for_embedded_C_developers::Packed_and_Aligned_Types (line 350) stdout ----
    error: reference to packed field is unaligned
      --> /tmp/mdbook-xawLyJ/c-tips/index.md:361:41
    12 |     unsafe { println!("{:p} {:p} {:p}", &v.x, &v.y, &v.z) };
       |                                         ^^^^
       |
       = note: `#[deny(unaligned_references)]` on by default
       = note: `#[deny(unaligned_references)]` on by default
       = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
       = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
       = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
       = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
    
    error: reference to packed field is unaligned
      --> /tmp/mdbook-xawLyJ/c-tips/index.md:361:53
    12 |     unsafe { println!("{:p} {:p} {:p}", &v.x, &v.y, &v.z) };
       |                                                     ^^^^
       |
       = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
---
    
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-xawLyJ/c-tips/index.md - Tips_for_embedded_C_developers::Packed_and_Aligned_Types (line 350)
    test result: FAILED. 5 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.20s
    
    
    --- stderr
---
Verifying status of miri...
Verifying status of embedded-book...
This PR updated 'src/doc/embedded-book', verifying if status is 'test-pass'...

We detected that this PR updated 'embedded-book', but its tests failed.

If you do intend to update 'embedded-book', please check the error messages above and
commit another update.

If you do NOT intend to update 'embedded-book', please ensure you did not accidentally
change the submodule at 'src/doc/embedded-book'. You may ask your reviewer for the
