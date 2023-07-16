plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/library/std/src/os/fd/owned.rs at line 388:
 }
 
 #[unstable(feature = "unbound_socket", issue = "99999")]
- impl AsFd for crate::net::UnboundUdpSocket {
-     #[inline]
+impl AsFd for crate::net::UnboundUdpSocket {
+    #[inline]
     fn as_fd(&self) -> BorrowedFd<'_> {
         self.as_inner().socket().as_fd()
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/fd/owned.rs" "/checkout/library/std/src/os/fd/mod.rs" "/checkout/library/std/src/os/windows/ffi.rs" "/checkout/library/std/src/os/solid/mod.rs" "/checkout/library/std/src/os/solid/io.rs" "/checkout/library/std/src/os/redox/mod.rs" "/checkout/library/std/src/os/wasi/ffi.rs" "/checkout/library/std/src/os/solid/ffi.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
