plain
   Compiling lock_api v0.4.1
   Compiling tracing-core v0.1.17
   Compiling thread_local v1.0.1
   Compiling sharded-slab v0.1.1
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/lock_api-0.4.1/src/mutex.rs:143:6
    |
143 | impl<R: RawMutex, T> Mutex<R, T> {
    |
    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
    = help: add `#![feature(const_fn_trait_bounds)]` to the crate attributes to enable


   Compiling itertools v0.9.0
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/lock_api-0.4.1/src/remutex.rs:217:6
    |
217 | impl<R: RawMutex, G: GetThreadId, T> ReentrantMutex<R, G, T> {
    |
    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
    = help: add `#![feature(const_fn_trait_bounds)]` to the crate attributes to enable


error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/lock_api-0.4.1/src/remutex.rs:217:19
    |
217 | impl<R: RawMutex, G: GetThreadId, T> ReentrantMutex<R, G, T> {
    |
    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
    = help: add `#![feature(const_fn_trait_bounds)]` to the crate attributes to enable


error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/lock_api-0.4.1/src/rwlock.rs:348:6
    |
348 | impl<R: RawRwLock, T> RwLock<R, T> {
    |
    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
    = help: add `#![feature(const_fn_trait_bounds)]` to the crate attributes to enable

