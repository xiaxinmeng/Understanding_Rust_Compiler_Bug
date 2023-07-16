 rust
rustc: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/libstd
../src/libstd\sys/windows\ext\thread.rs:19:1: 23:2 error: This node does not have a stability attribute
../src/libstd\sys/windows\ext\thread.rs:19 impl<T> AsRawHandle for thread::JoinHandle<T> {
../src/libstd\sys/windows\ext\thread.rs:20     fn as_raw_handle(&self) -> RawHandle {
../src/libstd\sys/windows\ext\thread.rs:21         self.as_inner().handle().raw() as *mut _
../src/libstd\sys/windows\ext\thread.rs:22     }
../src/libstd\sys/windows\ext\thread.rs:23 }
../src/libstd\sys/windows\ext\thread.rs:25:1: 29:2 error: This node does not have a stability attribute
../src/libstd\sys/windows\ext\thread.rs:25 impl<T> IntoRawHandle for thread::JoinHandle<T>  {
../src/libstd\sys/windows\ext\thread.rs:26     fn into_raw_handle(self) -> RawHandle {
../src/libstd\sys/windows\ext\thread.rs:27         self.into_inner().into_handle().into_raw() as *mut _
../src/libstd\sys/windows\ext\thread.rs:28     }
../src/libstd\sys/windows\ext\thread.rs:29 }
error: aborting due to 2 previous errors
