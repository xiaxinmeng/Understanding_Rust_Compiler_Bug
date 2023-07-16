
error[E0382]: use of moved value: `file`
  --> src/main.rs:10:19
   |
8  |     let file = File::create("hello.txt").unwrap();
   |         ---- move occurs because `file` has type `File`, which does not implement the `Copy` trait
9  |     write_to_file(file, String::from("Hello File!"));
   |                   ---- value moved here
10 |     write_to_file(file, String::from("Hello File!"));
   |                   ^^^^ value used here after move
   |
note: consider changing this parameter type in function `write_to_file` to borrow instead if owning the value isn't necessary
  --> src/main.rs:14:24
   |
14 | fn write_to_file(file: File, string_buffer: String) -> Result<usize, io::Error> {
   |    -------------       ^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
