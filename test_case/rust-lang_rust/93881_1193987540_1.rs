rust
error[E0080]: evaluation of constant value failed
    |
   ::: src/lib.rs:4:14
    |
4   |     unsafe { ptr.offset(3) }
    |              ------------- inside `demo` at src/lib.rs:4:14
...
7   | pub const P: *const u8 = demo();
    |                          ------ inside `P` at src/lib.rs:7:26
