plain
[00:05:57]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:05:58]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:05:58]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:06:02]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:06:05] error[E0277]: can't compare `libc::sockaddr_in` with `_`
[00:06:05]    |
[00:06:05]    |
[00:06:05] 86 | pub struct SocketAddrV4 { inner: c::sockaddr_in }
[00:06:05]    |                           ^^^^^^^^^^^^^^^^^^^^^ no implementation for `libc::sockaddr_in < _` and `libc::sockaddr_in > _`
[00:06:05]    |
[00:06:05]    = help: the trait `core::cmp::PartialOrd<_>` is not implemented for `libc::sockaddr_in`
[00:06:05]    = note: required by `core::cmp::PartialOrd::partial_cmp`
[00:06:05] 
[00:06:05] error[E0277]: the trait bound `libc::sockaddr_in: core::cmp::Ord` is not satisfied
[00:06:05]    |
[00:06:05]    |
[00:06:05] 86 | pub struct SocketAddrV4 { inner: c::sockaddr_in }
[00:06:05]    |                           ^^^^^^^^^^^^^^^^^^^^^ the trait `core::cmp::Ord` is not implemented for `libc::sockaddr_in`
[00:06:05]    = note: required by `test/tools
31632 ./.git/modules/src/tools/lld/objects
31624 ./.git/modules/src/tools/lld/objects/pack
30300 ./src/llvm/test/Transforms
