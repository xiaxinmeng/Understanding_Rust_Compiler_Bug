plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking hashbrown v0.12.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:331:1
    |
331 | / impl AsSocket for crate::net::UnboundUdpSocket {
332 | |     #[inline]
333 | |     fn as_socket(&self) -> BorrowedSocket<'_> {
334 | |         unsafe { BorrowedSocket::borrow_raw(self.as_raw_socket()) }
336 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:338:1
    |
338 | / impl From<crate::net::UnboundUdpSocket> for OwnedSocket {
339 | |     #[inline]
340 | |     fn from(unbound_udp_socket: crate::net::UnboundUdpSocket) -> OwnedSocket {
341 | |         unsafe { OwnedSocket::from_raw_socket(unbound_udp_socket.into_raw_socket()) }
343 | | }
    | |_^

error: could not compile `std` due to 2 previous errors
