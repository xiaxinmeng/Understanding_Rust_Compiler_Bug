console
error[E0046]: not all trait items implemented, missing: `read`
 --> src/main.rs:3:1
  |
3 | impl std::io::Read for MyRead {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `read` in implementation
  |
  = help: implement the missing item: `fn read(&mut self, _: &mut [u8]) -> Result<usize, std::io::Error> { todo!() }`
