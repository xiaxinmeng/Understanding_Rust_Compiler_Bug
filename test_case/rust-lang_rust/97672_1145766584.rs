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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys_common/net.rs at line 672:
 
         let inner = Socket::new(&addr_family, c::SOCK_DGRAM)?;
         let addr_family = *addr_family;
-        let new_self = Self { inner,  addr_family };
+        let new_self = Self { inner, addr_family };
         Ok(new_self)
 
Diff in /checkout/library/std/src/net/udp.rs at line 3:
 
 use crate::fmt;
 use crate::fmt;
 use crate::io::{self, ErrorKind};
-use crate::net::{Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs, SocketAddrFamily};
+use crate::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrFamily, ToSocketAddrs};
 use crate::sys_common::net as net_imp;
 use crate::sys_common::{AsInner, FromInner, IntoInner};
 use crate::time::Duration;
Diff in /checkout/library/std/src/net/mod.rs at line 24:
 use crate::io::{self, ErrorKind};
 
 #[stable(feature = "rust1", since = "1.0.0")]
-pub use self::addr::{SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs, SocketAddrFamily};
+pub use self::addr::{SocketAddr, SocketAddrFamily, SocketAddrV4, SocketAddrV6, ToSocketAddrs};
 #[stable(feature = "rust1", since = "1.0.0")]
 pub use self::ip::{IpAddr, Ipv4Addr, Ipv6Addr, Ipv6MulticastScope};
 #[stable(feature = "rust1", since = "1.0.0")]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys_common/thread_parker/mod.rs" "/checkout/library/std/src/sys_common/thread_parker/generic.rs" "/checkout/library/std/src/sys_common/thread_parker/futex.rs" "/checkout/library/std/src/sys_common/thread_local_key.rs" "/checkout/library/std/src/sys_common/rwlock.rs" "/checkout/library/std/src/sys_common/net.rs" "/checkout/library/std/src/sys_common/thread.rs" "/checkout/library/std/src/sys_common/wtf8/tests.rs"` failed.
Build completed unsuccessfully in 0:00:14
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
