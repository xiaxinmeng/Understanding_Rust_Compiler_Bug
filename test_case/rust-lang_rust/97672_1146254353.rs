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
    Checking addr2line v0.16.0
error[E0308]: mismatched types
   --> library/std/src/sys_common/net.rs:461:32
    |
461 |         let sock = Socket::new(addr.family(), c::SOCK_DGRAM)?;
    |                    |           |
    |                    |           |
    |                    |           expected `&SocketAddrFamily`, found enum `SocketAddrFamily`
    |                    |           help: consider borrowing here: `&addr.family()`
    |
note: associated function defined here
   --> library/std/src/sys/windows/net.rs:103:12
    |
    |
103 |     pub fn new(addr: &SocketAddrFamily, ty: c_int) -> io::Result<Socket> {

error[E0308]: mismatched types
   --> library/std/src/sys_common/net.rs:673:33
    |
    |
673 |         let inner = Socket::new(addr_family, c::SOCK_DGRAM)?;
    |                     |           |
    |                     |           |
    |                     |           expected `&SocketAddrFamily`, found enum `SocketAddrFamily`
    |                     |           help: consider borrowing here: `&addr_family`
    |
note: associated function defined here
   --> library/std/src/sys/windows/net.rs:103:12
    |
    |
103 |     pub fn new(addr: &SocketAddrFamily, ty: c_int) -> io::Result<Socket> {

error[E0308]: mismatched types
   --> library/std/src/sys_common/net.rs:735:32
    |
    |
735 |         let sock = Socket::new(addr_family, c::SOCK_STREAM)?;
    |                    |           |
    |                    |           |
    |                    |           expected `&SocketAddrFamily`, found enum `SocketAddrFamily`
    |                    |           help: consider borrowing here: `&addr_family`
    |
note: associated function defined here
   --> library/std/src/sys/windows/net.rs:103:12
    |
    |
103 |     pub fn new(addr: &SocketAddrFamily, ty: c_int) -> io::Result<Socket> {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to 3 previous errors
Build completed unsuccessfully in 0:00:21
