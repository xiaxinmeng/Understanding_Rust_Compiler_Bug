plain
[RUSTC-TIMING] gimli test:false 6.148
[RUSTC-TIMING] object test:false 6.832
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error[E0599]: no method named `read_buf` found for struct `sgx::net::TcpStream` in the current scope
    |
    |
623 |         self.0.read_buf(buf)
    |
    |
   ::: library/std/src/sys/sgx/net.rs:46:1
46  | pub struct TcpStream {
    | -------------------- method `read_buf` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf`, perhaps you need to implement it
    |
554 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `read_buf` found for struct `sgx::net::TcpStream` in the current scope
    |
    |
661 |         self.0.read_buf(buf)
    |
    |
   ::: library/std/src/sys/sgx/net.rs:46:1
46  | pub struct TcpStream {
    | -------------------- method `read_buf` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf`, perhaps you need to implement it
    |
554 | pub trait Read {
    | ^^^^^^^^^^^^^^

