
error[E0596]: cannot borrow `*reader` as mutable, as it is behind a `&` reference
 --> src/main.rs:5:5
  |
3 |     let mut reader = &mut mem_buffer as &dyn std::io::Read;
  |         ---------- consider changing this binding's type to be: `&mut dyn std::io::Read`
4 |     let mut read_buffer = [0u8, 10];
5 |     reader.read(&mut read_buffer);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `reader` is a `&` reference, so the data it refers to cannot be borrowed as mutable
