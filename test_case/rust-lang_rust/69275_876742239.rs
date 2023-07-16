
error[E0382]: use of moved value: `non_copy_value`
  --> src/main.rs:11:10
   |
10 |     let non_copy_value = vec![0u8];
   |         -------------- move occurs because `non_copy_value` has type `Vec<u8>`, which does not implement the `Copy` trait
11 |     foo!(non_copy_value);
   |          ^^^^^^^^^^^^^^
   |          |
   |          value moved here
   |          value used here after move
