plain
    Checking addr2line v0.16.0
error: implementation has missing stability attribute
   --> library/std/src/net/tcp.rs:978:1
    |
978 | / impl Iterator for IntoIncoming {
979 | |     type Item = io::Result<TcpStream>;
980 | |     fn next(&mut self) -> Option<io::Result<TcpStream>> {
981 | |         Some(self.listener.accept().map(|p| p.0))
983 | | }
    | |_^

error: struct has missing stability attribute
error: struct has missing stability attribute
   --> library/std/src/net/tcp.rs:106:1
    |
106 | / pub struct IntoIncoming {
107 | |     listener: TcpListener,
    | |_^

error: implementation has missing stability attribute
   --> library/std/src/net/tcp.rs:105:10
   --> library/std/src/net/tcp.rs:105:10
    |
105 |   #[derive(Debug)]
    |            ^^^^^ in this derive macro expansion
    | 
   ::: /checkout/library/core/src/fmt/mod.rs:650:5
    |
650 | /     pub macro Debug($item:item) {
652 | |     }
    | |_____- in this expansion of `#[derive(Debug)]`

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/net/tcp.rs:837:5
    |
837 | /     pub fn into_incoming(self) -> IntoIncoming {
838 | |         IntoIncoming { listener: self }
    | |_____^

error: could not compile `std` due to 4 previous errors
Build completed unsuccessfully in 0:01:38
