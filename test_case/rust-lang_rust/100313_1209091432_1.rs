
error[[E0080]](https://doc.rust-lang.org/nightly/error-index.html#E0080): evaluation of constant value failed
  --> src/main.rs:9:13
   |
9  |             *(B as *const u64 as *mut u64) -= 1;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |             |
   |             writing to alloc7 which is read-only
   |             inside `T::<{0x1 as &u64}>::set_false` at src/main.rs:9:13
...
16 |     x.set_false();
   |     ------------- inside `_` at src/main.rs:16:5
