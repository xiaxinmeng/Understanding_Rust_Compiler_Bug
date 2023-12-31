
warning: reference to packed field is unaligned
   --> programs/stocks/src/lib.rs:370:5
    |
370 |     &s.c
    |     ^^^^
    |
    = note: `#[warn(unaligned_references)]` on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
    = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
