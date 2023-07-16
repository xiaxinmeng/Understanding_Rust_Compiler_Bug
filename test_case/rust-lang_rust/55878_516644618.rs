
error: reaching this expression at runtime will panic or abort
   --> rust/src/libcore/mem/mod.rs:243:5
    |
243 |     intrinsics::size_of::<T>()
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ rustc layout computation failed: SizeOverflow([u8; 18446744073709551615])
    |
   ::: file7.rs:2:26
    |
2   |     println!("Size: {}", std::mem::size_of::<[u8; std::u64::MAX as usize]>());
    |                          ---------------------------------------------------
    |
    = note: `#[deny(const_err)]` on by default

error: aborting due to previous error
