
rustc 1.68.0-dev running on x86_64-unknown-linux-gnu at 2023-01-10 02:37:25
compiler flags:
    -Z
    treat-err-as-bug

panicked at compiler/rustc_errors/src/lib.rs:1663:30:
aborting due to `-Z treat-err-as-bug=1`

   0: rustc_driver::write_ice_to_disk
             at ./compiler/rustc_driver/src/lib.rs:1206:19
   1: rustc_driver::DEFAULT_HOOK::{closure#0}::{closure#0}
             at ./compiler/rustc_driver/src/lib.rs:1304:44
   2: <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call
             at ./library/alloc/src/boxed.rs:2002:9
   3: std::panicking::rust_panic_with_hook
             at ./library/std/src/panicking.rs:692:13
8<8<8<8<8<8<8<8<8<8<8<8<8<8<8<8<
 113: std::sys::unix::thread::Thread::new::thread_start
             at ./library/std/src/sys/unix/thread.rs:108:17
 114: start_thread
             at ./nptl/./nptl/pthread_create.c:442:8
 115: clone3
             at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81


query stack during panic:
#0 [mir_borrowck] borrow-checking `<impl at f6.rs:45:1: 45:16>::foo`
#1 [analysis] running analysis passes on this crate
end of query stack
