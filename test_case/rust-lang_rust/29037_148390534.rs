
../rust/src/libstd/thread/mod.rs:283:27: 283:32 warning: the parameter type `T` may not live long enough [E0310]
../rust/src/libstd/thread/mod.rs:283             *their_packet.get() = Some(try_result.map(|()| {
                                                               ^~~~~
../rust/src/libstd/thread/mod.rs:283:27: 283:32 help: run `rustc --explain E0310` to see a detailed explanation
../rust/src/libstd/thread/mod.rs:283:27: 283:32 help: consider adding an explicit lifetime bound `T: 'static`...
../rust/src/libstd/thread/mod.rs:283:27: 283:32 note: this warning results from recent bug fixes and clarifications; it will become a HARD ERROR in the next release. See RFC 1214 for details.
../rust/src/libstd/thread/mod.rs:283             *their_packet.get() = Some(try_result.map(|()| {
                                                               ^~~~~
../rust/src/libstd/thread/mod.rs:283:27: 283:32 note: ...so that the reference type `&alloc::arc::Arc<core::cell::UnsafeCell<core::option::Option<core::result::Result<T, Box<core::any::Any + Send>>>>>` does not outlive the data it points at
../rust/src/libstd/thread/mod.rs:283             *their_packet.get() = Some(try_result.map(|()| {
                                                               ^~~~~
../rust/src/libstd/thread/mod.rs:283:27: 283:32 warning: the parameter type `T` may not live long enough [E0310]
../rust/src/libstd/thread/mod.rs:283             *their_packet.get() = Some(try_result.map(|()| {
                                                               ^~~~~
../rust/src/libstd/thread/mod.rs:283:27: 283:32 help: run `rustc --explain E0310` to see a detailed explanation
../rust/src/libstd/thread/mod.rs:283:27: 283:32 help: consider adding an explicit lifetime bound `T: 'static`...
../rust/src/libstd/thread/mod.rs:283:27: 283:32 note: this warning results from recent bug fixes and clarifications; it will become a HARD ERROR in the next release. See RFC 1214 for details.
../rust/src/libstd/thread/mod.rs:283             *their_packet.get() = Some(try_result.map(|()| {
                                                               ^~~~~
../rust/src/libstd/thread/mod.rs:283:27: 283:32 note: ...so that the reference type `&core::cell::UnsafeCell<core::option::Option<core::result::Result<T, Box<core::any::Any + Send>>>>` does not outlive the data it points at
../rust/src/libstd/thread/mod.rs:283             *their_packet.get() = Some(try_result.map(|()| {
                                                               ^~~~~
