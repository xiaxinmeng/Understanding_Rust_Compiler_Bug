
 Documenting std v0.0.0 (/home/imperio/rust/rust/src/libstd)
error[E0599]: no method named `into_handle` found for struct `sys::unix::thread::Thread` in the current scope
  --> src/libstd/sys/windows/ext/thread.rs:19:27
   |
19 |         self.into_inner().into_handle().into_raw() as *mut _
   |                           ^^^^^^^^^^^ method not found in `sys::unix::thread::Thread`
   | 
  ::: src/libstd/sys/unix/thread.rs:14:1
   |
14 | pub struct Thread {
   | ----------------- method `into_handle` not found for this

error: internal compiler error: src/librustc_passes/dead.rs:122:13: no type-dependent def for method
