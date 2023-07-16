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
Diff in /checkout/library/std/src/os/unix/net/addr.rs at line 285:
                 ));
             }
 
-            ptr::copy_nonoverlapping(namespace.as_ptr(), addr.sun_path.as_mut_ptr().offset(1) as *mut u8, namespace.len());
+                namespace.as_ptr(),
+                namespace.as_ptr(),
+                addr.sun_path.as_mut_ptr().offset(1) as *mut u8,
+                namespace.len(),
+            );
             let len = (sun_path_offset(&addr) + 1 + namespace.len()) as libc::socklen_t;
             SocketAddr::from_parts(addr, len)
Diff in /checkout/library/std/src/os/unix/net/stream.rs at line 130:
Diff in /checkout/library/std/src/os/unix/net/stream.rs at line 130:
     pub fn connect_addr(socket_addr: &SocketAddr) -> io::Result<UnixStream> {
         unsafe {
             let inner = Socket::new_raw(libc::AF_UNIX, libc::SOCK_STREAM)?;
-            cvt(libc::connect(*inner.as_inner(), &socket_addr.addr as *const _ as *const _, socket_addr.len))?;
+            cvt(libc::connect(
+                *inner.as_inner(),
+                &socket_addr.addr as *const _ as *const _,
+                socket_addr.len,
+            ))?;
             Ok(UnixStream(inner))
     }
Diff in /checkout/library/std/src/os/unix/net/listener.rs at line 107:
Diff in /checkout/library/std/src/os/unix/net/listener.rs at line 107:
     pub fn bind_addr(socket_addr: &SocketAddr) -> io::Result<UnixListener> {
         unsafe {
             let inner = Socket::new_raw(libc::AF_UNIX, libc::SOCK_STREAM)?;
-            cvt(libc::bind(*inner.as_inner(), &socket_addr.addr as *const _ as *const _, socket_addr.len as _))?;
+            cvt(libc::bind(
+                *inner.as_inner(),
+                &socket_addr.addr as *const _ as *const _,
+                socket_addr.len as _,
+            ))?;
             cvt(libc::listen(*inner.as_inner(), 128))?;
 
             Ok(UnixListener(inner))
Diff in /checkout/library/std/src/os/unix/net/listener.rs at line 114:
     }
-
 
 
     /// Accepts a new incoming connection to this listener.
Diff in /checkout/library/std/src/os/unix/net/datagram.rs at line 136:
Diff in /checkout/library/std/src/os/unix/net/datagram.rs at line 136:
     pub fn bind_addr(socket_addr: &SocketAddr) -> io::Result<UnixDatagram> {
         unsafe {
             let socket = UnixDatagram::unbound()?;
-            cvt(libc::bind(*socket.0.as_inner(), &socket_addr.addr as *const _ as *const _, socket_addr.len as _))?;
+            cvt(libc::bind(
+                *socket.0.as_inner(),
+                &socket_addr.addr as *const _ as *const _,
+                socket_addr.len as _,
+            ))?;
             Ok(socket)
     }
Diff in /checkout/library/std/src/os/unix/net/datagram.rs at line 245:
Diff in /checkout/library/std/src/os/unix/net/datagram.rs at line 245:
     #[unstable(feature = "unix_socket_abstract", issue = "42048")]
     pub fn connect_addr(&self, socket_addr: &SocketAddr) -> io::Result<()> {
         unsafe {
-            cvt(libc::connect(*self.0.as_inner(), &socket_addr.addr as *const _ as *const _, socket_addr.len))?;
+            cvt(libc::connect(
+                *self.0.as_inner(),
+                &socket_addr.addr as *const _ as *const _,
+                socket_addr.len,
+            ))?;
         Ok(())
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/unix/fs.rs" "/checkout/library/std/src/os/unix/thread.rs" "/checkout/library/std/src/os/unix/process.rs" "/checkout/library/std/src/os/unix/ucred/tests.rs" "/checkout/library/std/src/os/unix/raw.rs" "/checkout/library/std/src/os/unix/net/ancillary.rs" "/checkout/library/std/src/os/unix/net/addr.rs" "/checkout/library/std/src/os/unix/ucred.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
