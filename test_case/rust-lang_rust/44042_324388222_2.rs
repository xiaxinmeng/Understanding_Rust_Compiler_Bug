
error[E0152]: duplicate lang item found: `slice`.
    --> src/liballoc/slice.rs:176:1
     |
176  | / impl<T> [T] {
177  | |     /// Returns the number of elements in the slice.
178  | |     ///
179  | |     /// # Example
...    |
1506 | |     }
1507 | | }
     | |_^
     |
note: first defined here.
    --> src/liballoc/slice.rs:170:1
     |
170  | / impl [u8] {
171  | |     fn foobar(&self) {}
172  | | }
     | |_^
