
error[E0080]: evaluation of constant value failed
    --> $HOME\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\ptr\mod.rs:1139:9
     |
929  |       unsafe { swap_nonoverlapping_simple_untyped(x, y, count) }
     |                ----------------------------------------------- inside `swap_nonoverlapping::<MaybeUninit<u8>>` at $HOME\.rustup\toolchains\nightly-x86948  |           mem::swap_simple::<MaybeUninit<T>>(x, y);
     |           ---------------------------------------- inside `ptr::swap_nonoverlapping_simple_untyped::<MaybeUninit<u8>>` at $HOME\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\ptr\mod.rs:948:9
...
1139 |           copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |           |
     |           unable to copy parts of a pointer from memory at alloc6+0x1
     |           inside `std::ptr::read::<MaybeUninit<MaybeUninit<u8>>>` at $HOME\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\ptr\mod.rs:1139:9
     |
    ::: $HOME\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\mem\mod.rs:776:17
     |
776  |           let a = ptr::read(x);
     |                   ------------ inside `mem::swap_simple::<MaybeUninit<MaybeUninit<u8>>>` at $HOME\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\mem\mod.rs:776:17
     |
    ::: src\main.rs:14:9
     |
14   | /         ptr::swap_nonoverlapping(
15   | |             &mut ptr1 as *mut _ as *mut MaybeUninit<u8>,
16   | |             &mut ptr2 as *mut _ as *mut MaybeUninit<u8>,
17   | |             mem::size_of::<&i32>(),
18   | |         );
     | |_________- inside `X` at src\main.rs:14:9
     |
     = help: this code performed an operation that depends on the underlying bytes representing a pointer
     = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
