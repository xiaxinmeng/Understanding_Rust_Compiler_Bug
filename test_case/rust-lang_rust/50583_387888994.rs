
(gdb) bt
#0  printf_core (f=f@entry=0x0, fmt=fmt@entry=0x11a1d1 "Symbol not found: %s", ap=ap@entry=0x7effe6ec, nl_arg=nl_arg@entry=0x7effe718, nl_type=nl_type@entry=0x7effe6f0) at src/stdio/vfprintf.c:618
#1  0x0010179c in vfprintf (f=f@entry=0x7effe7f0, fmt=fmt@entry=0x11a1d1 "Symbol not found: %s", ap=ap@entry=...) at src/stdio/vfprintf.c:666
#2  0x001019e8 in vsnprintf (s=s@entry=0x0, n=n@entry=0, fmt=fmt@entry=0x11a1d1 "Symbol not found: %s", ap=...) at src/stdio/vsnprintf.c:54
#3  0x00105894 in __dl_vseterr (fmt=0x11a1d1 "Symbol not found: %s", fmt@entry=0x7efff55c "<\365\377~\354\365\377~\344\365\377~\001", ap=..., ap@entry=...) at src/ldso/dlerror.c:34
#4  0x001058ec in __dl_seterr (fmt=0x11a1d1 "Symbol not found: %s") at src/ldso/dlerror.c:51
#5  0x0010581c in stub_dlsym (p=<optimized out>, s=<optimized out>, ra=<optimized out>) at src/ldso/__dlsym.c:9
#6  0x000c8dc4 in mio::sys::unix::dlsym::fetch (name=...) at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.6.14/src/sys/unix/dlsym.rs:43
#7  0x000c8b60 in <mio::sys::unix::dlsym::DlSym<F>>::get (self=0x14506c <mio::sys::unix::epoll::Selector::new::epoll_create1>) at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.6.14/src/sys/unix/dlsym.rs:30
#8  0x000c8e7c in mio::sys::unix::epoll::Selector::new () at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.6.14/src/sys/unix/epoll.rs:38
#9  0x000bd148 in mio::poll::Poll::new () at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.6.14/src/poll.rs:655
#10 0x000a934c in tokio_reactor::Reactor::new () at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-reactor-0.1.1/src/lib.rs:215
#11 0x00053b78 in tokio::runtime::builder::Builder::build (self=0x7effeff0) at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.1.5/src/runtime/builder.rs:94
#12 0x00058ccc in tokio::runtime::Runtime::new () at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.1.5/src/runtime/mod.rs:260
#13 0x0004c72c in tokio_core::reactor::Core::new () at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-core-0.1.17/src/reactor/mod.rs:129
#14 0x000129a8 in tokio_segfault_test::main () at src/main.rs:5
