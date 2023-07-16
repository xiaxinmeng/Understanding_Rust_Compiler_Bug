plain
[RUSTC-TIMING] gimli test:false 5.831
[RUSTC-TIMING] object test:false 6.971
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

error[E0599]: no method named `read_buf` found for struct `sys::wasm::net::TcpStream` in the current scope
    |
    |
623 |         self.0.read_buf(buf)
    |
    |
   ::: library/std/src/sys/wasm/../unsupported/net.rs:7:1
    |
7   | pub struct TcpStream(!);
    | -------------------- method `read_buf` not found for this struct
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf`, perhaps you need to implement it
    |
554 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `read_buf` found for struct `sys::wasm::net::TcpStream` in the current scope
    |
    |
661 |         self.0.read_buf(buf)
    |
    |
   ::: library/std/src/sys/wasm/../unsupported/net.rs:7:1
    |
7   | pub struct TcpStream(!);
    | -------------------- method `read_buf` not found for this struct
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf`, perhaps you need to implement it
    |
554 | pub trait Read {
    | ^^^^^^^^^^^^^^

