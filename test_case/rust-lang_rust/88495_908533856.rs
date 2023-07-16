plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.11.0
    Checking addr2line v0.16.0
error[E0277]: the trait bound `linger: core::marker::Copy` is not satisfied
   --> library/std/src/sys/windows/net.rs:459:30
    |
459 |         let val: c::linger = net::getsockopt(self, c::SOL_SOCKET, c::SO_LINGER)?;
    |                              ^^^^^^^^^^^^^^^ the trait `core::marker::Copy` is not implemented for `linger`
   ::: library/std/src/sys_common/net.rs:69:22
    |
    |
69  | pub fn getsockopt<T: Copy>(sock: &Socket, opt: c_int, val: c_int) -> io::Result<T> {
    |                      ---- required by this bound in `sys_common::net::getsockopt`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:21
