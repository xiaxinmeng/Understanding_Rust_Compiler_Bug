
error: any use of this value will cause an error
    --> /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1096:9
     |
1096 |           copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |           |
     |           unable to turn pointer into raw bytes
     |           inside `std::ptr::read::<MaybeUninit<u8>>` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1096:9
     |           inside `mem::swap_simple::<MaybeUninit<u8>>` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/mem/mod.rs:762:17
     |           inside `ptr::swap_nonoverlapping_simple::<MaybeUninit<u8>>` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:913:9
     |           inside `swap_nonoverlapping::<Demo>` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:861:33
     |           inside `std::mem::swap::<Demo>` at /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/library/core/src/mem/mod.rs:726:29
     |           inside `C` at swap.rs:10:5
     |
    ::: swap.rs:7:1
     |
7    | / const C: (Demo, Demo) = {
8    | |     let mut x = Demo(0, &1, 2, -1, -1);
9    | |     let mut y = Demo(3, &4, 5, -1, -1);
10   | |     std::mem::swap(&mut x, &mut y);
11   | |     (x, y)
12   | | };
     | |__-
     |
     = note: `#[deny(const_err)]` on by default
     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
     = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
