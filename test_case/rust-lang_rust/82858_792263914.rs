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
    Checking addr2line v0.14.0
error[E0432]: unresolved import `crate::sys::ext::net`
 --> library/std/src/net/udp.rs:7:22
  |
7 | use crate::sys::ext::net::SocketAncillary;
  |                      ^^^ could not find `net` in `ext`
error[E0432]: unresolved import `crate::sys::ext::net`
  --> library/std/src/sys_common/net.rs:12:22
   |
12 | use crate::sys::ext::net::SocketAncillary;
12 | use crate::sys::ext::net::SocketAncillary;
   |                      ^^^ could not find `net` in `ext`

error: unused imports: `IoSliceMut`, `IoSlice`
  |
  |
5 | use crate::io::{self, Error, ErrorKind, IoSlice, IoSliceMut};
  |                                         ^^^^^^^  ^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`

error[E0599]: no method named `set_recvttl` found for struct `Socket` in the current scope
   --> library/std/src/sys_common/net.rs:502:20
    |
502 |         self.inner.set_recvttl(recvttl)
    |                    ^^^^^^^^^^^ method not found in `Socket`
   ::: library/std/src/sys/windows/net.rs:27:1
    |
    |
27  | pub struct Socket(c::SOCKET);
    | ----------------------------- method `set_recvttl` not found for this

error[E0599]: no method named `recvttl` found for struct `Socket` in the current scope
   --> library/std/src/sys_common/net.rs:506:20
    |
506 |         self.inner.recvttl()
    |                    ^^^^^^^ method not found in `Socket`
   ::: library/std/src/sys/windows/net.rs:27:1
    |
    |
27  | pub struct Socket(c::SOCKET);
    | ----------------------------- method `recvttl` not found for this
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0432, E0599.
For more information about an error, try `rustc --explain E0432`.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:31
