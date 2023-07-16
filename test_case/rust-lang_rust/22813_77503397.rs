
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/lib.rs:116:12: 116:26 warning: feature is deprecated and will only be available for a limited time, please rewrite code that relies on it
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/lib.rs:116 #![feature(old_impl_check)]
                                                                                                         ^~~~~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/lib.rs:116:12: 116:26 warning: feature is deprecated and will only be available for a limited time, please rewrite code that relies on it
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/lib.rs:116 #![feature(old_impl_check)]
                                                                                                         ^~~~~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/old_io/mod.rs:1618:10: 1618:11 warning: the type parameter `T` is not constrained by the impl trait, self type, or predicates
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/old_io/mod.rs:1618 impl<'a, T, A: ?Sized + Acceptor<T>> Iterator for IncomingConnections<'a, A> {
                                                                                                               ^
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:76:18: 76:47 error: use of deprecated item: will be removed to be reintroduced at a later date; in the meantime consider using the `unix_socket` crate for unix sockets; there is currently no replacement for named pipes, #[deny(deprecated)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:76 impl AsRawFd for old_io::net::pipe::UnixStream {
                                                                                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:82:18: 82:49 error: use of deprecated item: will be removed to be reintroduced at a later date; in the meantime consider using the `unix_socket` crate for unix sockets; there is currently no replacement for named pipes, #[deny(deprecated)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:82 impl AsRawFd for old_io::net::pipe::UnixListener {
                                                                                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:88:18: 88:49 error: use of deprecated item: will be removed to be reintroduced at a later date; in the meantime consider using the `unix_socket` crate for unix sockets; there is currently no replacement for named pipes, #[deny(deprecated)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:88 impl AsRawFd for old_io::net::pipe::UnixAcceptor {
                                                                                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:94:18: 94:45 error: use of deprecated item: replaced with new I/O primitives in `std::net`, #[deny(deprecated)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:94 impl AsRawFd for old_io::net::tcp::TcpStream {
                                                                                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:100:18: 100:47 error: use of deprecated item: replaced with new I/O primitives in `std::net`, #[deny(deprecated)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:100 impl AsRawFd for old_io::net::tcp::TcpListener {
                                                                                                                        ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:106:18: 106:47 error: use of deprecated item: replaced with new I/O primitives in `std::net`, #[deny(deprecated)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:106 impl AsRawFd for old_io::net::tcp::TcpAcceptor {
                                                                                                                        ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:112:18: 112:45 error: use of deprecated item: replaced with new I/O primitives in `std::net`, #[deny(deprecated)] on by default
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-x-android-t/build/src/libstd/sys/unix/ext.rs:112 impl AsRawFd for old_io::net::udp::UdpSocket {
                                                                                                                        ^~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 7 previous errors
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.std] Error 101
