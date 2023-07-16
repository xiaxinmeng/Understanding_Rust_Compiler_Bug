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
    Checking object v0.26.1
    Checking hashbrown v0.11.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error[E0425]: cannot find function `setsockopt` in this scope
   --> library/std/src/sys/windows/net.rs:455:9
    |
455 |         setsockopt(self, c::SOL_SOCKET, c::SO_LINGER, linger)
    |
    |
help: try calling `setsockopt` as a method
    |
455 |         self.setsockopt(c::SOL_SOCKET, c::SO_LINGER, linger)
help: consider importing one of these items
    |
    |
3   | use crate::sys::c::setsockopt;
3   | use crate::sys_common::net::setsockopt;
    |
3   | use libc::setsockopt;
    |
    |

error[E0308]: mismatched types
   --> library/std/src/sys/windows/net.rs:451:22
    |
451 |             l_onoff: linger.is_some() as c_int,
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u16`, found `i32`
error[E0308]: mismatched types
   --> library/std/src/sys/windows/net.rs:452:23
    |
    |
452 |             l_linger: linger.map(|dur| dur.as_secs() as c_int).unwrap_or_default(),
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u16`, found `i32`
error[E0308]: mismatched types
   --> library/std/src/sys/windows/net.rs:459:30
    |
    |
459 |         let val: c::linger = net::getsockopt(self, c::SOL_SOCKET, c::SO_LINGER);
    |                  ---------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `linger`, found enum `core::result::Result`
    |                  expected due to this
    |
    = note: expected struct `linger`
                 found enum `core::result::Result<_, io::error::Error>`
                 found enum `core::result::Result<_, io::error::Error>`

error[E0616]: field `l_onoff` of struct `linger` is private
   --> library/std/src/sys/windows/net.rs:461:17
    |
461 |         Ok((val.l_onoff != 0).then(|| Duration::from_secs(val.l_linger as u64)))


error[E0616]: field `l_linger` of struct `linger` is private
   --> library/std/src/sys/windows/net.rs:461:63
    |
461 |         Ok((val.l_onoff != 0).then(|| Duration::from_secs(val.l_linger as u64)))

Some errors have detailed explanations: E0308, E0425, E0616.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `std` due to 6 previous errors
