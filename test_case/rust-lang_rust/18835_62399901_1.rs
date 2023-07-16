
lib.rs:5:1: 9:2 error: conflicting implementations for trait `core::ops::FnOnce` [E0119]
lib.rs:5 impl<T> FnOnce<(), T> for Chunk<T> {
lib.rs:6     extern "rust-call" fn call_once(self, arg: ()) -> T {
lib.rs:7         self.0.call_once(arg)
lib.rs:8     }
lib.rs:9 }
lib.rs:5:1: 9:2 note: conflicting implementation in crate `core`
lib.rs:5 impl<T> FnOnce<(), T> for Chunk<T> {
lib.rs:6     extern "rust-call" fn call_once(self, arg: ()) -> T {
lib.rs:7         self.0.call_once(arg)
lib.rs:8     }
lib.rs:9 }
error: aborting due to previous error
