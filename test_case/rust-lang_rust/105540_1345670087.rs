plain

error: unused variable: `buf`
  --> library/std/src/sys/wasm/../unsupported/pipe.rs:18:31
   |
18 |     pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
   |                               ^^^ help: if this is intentional, prefix it with an underscore: `_buf`
   |
   = note: `-D unused-variables` implied by `-D warnings`
[RUSTC-TIMING] std test:false 2.677
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
Build completed unsuccessfully in 0:12:08
