rust
error[E0080]: evaluation of constant value failed
   --> /home/nilsh/projects/rust/library/core/src/ptr/const_ptr.rs:457:18
    |
457 |         unsafe { intrinsics::offset(self, count) }
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |                  |
    |                  out-of-bounds pointer arithmetic: alloc3 has size 1, so pointer to 3 bytes starting at offset 0 is out-of-bounds
    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /home/nilsh/projects/rust/library/core/src/ptr/const_ptr.rs:457:18
    |
   ::: lol.rs:4:14
    |
4   |     unsafe { ptr.offset(3) }
    |              ------------- inside `demo` at lol.rs:4:14
...
7   | const P: *const u8 = demo();
    |                      ------ inside `P` at lol.rs:7:22
