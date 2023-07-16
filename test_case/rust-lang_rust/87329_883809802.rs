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
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error[E0433]: failed to resolve: could not find `unix` in `os`
 --> library/std/src/net/udp/tests.rs:4:16
4 | use crate::os::unix::io::AsRawFd;
4 | use crate::os::unix::io::AsRawFd;
  |                ^^^^ could not find `unix` in `os`

error[E0599]: no method named `as_raw_fd` found for reference `&Socket` in the current scope
   --> library/std/src/net/udp/tests.rs:176:44
    |
176 |     let udpsock_inner = udpsock.0.socket().as_raw_fd();
    |                                            ^^^^^^^^^ help: there is an associated function with a similar name: `as_raw`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
