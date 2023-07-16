
error[E0432]: unresolved import `std::sync::atomic::AtomicU64`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/quanta-0.3.1/src/lib.rs:51:14
   |
51 |     atomic::{AtomicU64, Ordering},
   |              ^^^^^^^^^
   |              |
   |              no `AtomicU64` in `sync::atomic`
   |              help: a similar name exists in the module: `AtomicU8`

error[E0432]: unresolved import `std::sync::atomic::AtomicU64`
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/quanta-0.3.1/src/mock.rs:5:18
  |
5 |         atomic::{AtomicU64, Ordering},
  |                  ^^^^^^^^^
  |                  |
  |                  no `AtomicU64` in `sync::atomic`
  |                  help: a similar name exists in the module: `AtomicU8`
