plain
Receiving objects: 100% (226/226), 21.93 KiB | 21.93 MiB/s, done.
---
Resolving deltas: 100% (195/195), completed with 52 local objects.
---
[00:00:47] configure: rust.quiet-tests     := True
---
[00:05:46] error[E0405]: cannot find trait `Sync` in module `std::sync`
[00:05:46]    --> librustc_data_structures/sync.rs:527:27
[00:05:46]     |
[00:05:46] 527 | unsafe impl<T> std::sync::Sync for OneThread<T> {}
[00:05:46]     |                           ^^^^ not found in `std::sync`
[00:05:46] help: possible candidates are found in other modules, you can import them into scope
[00:05:46]     |
[00:05:46] 32  | use core::prelude::v1::Sync;
[00:05:46]     |
[00:05:46] 32  | use std::marker::Sync;
[00:05:46]     |
[00:05:46] 32  | use std::prelude::v1::Sync;
[00:05:46]     |
[00:05:46] 32  | use sync::Sync;
[00:05:46]     |
[00:05:46]
[00:05:46] error[E0405]: cannot find trait `Send` in module `std::sync`
[00:05:46]    --> librustc_data_structures/sync.rs:528:27
[00:05:46]     |
[00:05:46] 528 | unsafe impl<T> std::sync::Send for OneThread<T> {}
[0x-gnu
