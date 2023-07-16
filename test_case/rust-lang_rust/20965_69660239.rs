
godot2:~ jdm$ rustc /tmp/sync.rs
/tmp/sync.rs:8:1: 9:2 error: the trait `core::kinds::Sync` is not implemented for the type `core::cell::UnsafeCell<std::comm::Flavor<()>>`
/tmp/sync.rs:8 impl Flow for BaseFlow {
/tmp/sync.rs:9 }
/tmp/sync.rs:8:1: 9:2 note: the type `core::cell::UnsafeCell<std::comm::Flavor<()>>` must implement `core::kinds::Sync` because it appears within the type `std::comm::Sender<()>`
/tmp/sync.rs:8 impl Flow for BaseFlow {
/tmp/sync.rs:9 }
/tmp/sync.rs:8:1: 9:2 note: the type `std::comm::Sender<()>` must implement `core::kinds::Sync` because it appears within the type `BaseFlow`
/tmp/sync.rs:8 impl Flow for BaseFlow {
/tmp/sync.rs:9 }
/tmp/sync.rs:8:1: 9:2 note: the trait `core::kinds::Sync` must be implemented because it is required by `Flow`
/tmp/sync.rs:8 impl Flow for BaseFlow {
/tmp/sync.rs:9 }
