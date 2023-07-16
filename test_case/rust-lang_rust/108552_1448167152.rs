plain
[RUSTC-TIMING] gimli test:false 4.056
[RUSTC-TIMING] object test:false 4.702
warning: dropping unsupported crate type `dylib` for target `wasm32-wasi`

error[E0599]: no method named `read_buf` found for struct `sys::wasi::net::TcpStream` in the current scope
    |
    |
623 |         self.0.read_buf(buf)
    |
   ::: library/std/src/sys/wasi/net.rs:15:1
    |
15  | pub struct TcpStream {
15  | pub struct TcpStream {
    | -------------------- method `read_buf` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf`, perhaps you need to implement it
    |
554 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `read_buf` found for struct `sys::wasi::net::TcpStream` in the current scope
    |
    |
661 |         self.0.read_buf(buf)
    |
   ::: library/std/src/sys/wasi/net.rs:15:1
    |
15  | pub struct TcpStream {
15  | pub struct TcpStream {
    | -------------------- method `read_buf` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf`, perhaps you need to implement it
    |
554 | pub trait Read {
    | ^^^^^^^^^^^^^^

