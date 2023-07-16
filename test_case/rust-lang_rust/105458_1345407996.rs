`
[RUSTC-TIMING] addr2line test:false 0.462
[RUSTC-TIMING] gimli test:false 4.967
[RUSTC-TIMING] object test:false 5.721
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

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
== clock drift check ==
  local time: Sat Dec 10 22:55:48 UTC 2022
  network time: Sat, 10 Dec 2022 22:55:48 GMT
  
  