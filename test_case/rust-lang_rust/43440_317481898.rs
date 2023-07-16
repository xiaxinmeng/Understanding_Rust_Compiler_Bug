
$ rustc src/main.rs
error: method `new` is private
 --> src/main.rs:4:22
  |
4 |     println!("{:?}", test::Test::new(String::from("test")));
  |                      ^^^^^^^^^^^^^^^

error: aborting due to previous error
