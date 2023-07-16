 rust
rustc 1.12.1 (d4f39402a 2016-10-19)
error[E0382]: use of moved value: `self`
 --> <anon>:8:41
  |
8 |         some_function(self.some_string, self.some_num)
  |                       ----------------  ^^^^^^^^^^^^^ value used here after move
  |                       |
  |                       value moved here
  |
  = note: move occurs because `self.some_string` has type `std::string::String`, which does not implement the `Copy` trait

error: aborting due to previous error
