bash
$ RUST_BACKTRACE=1 cargo +nightly -Z build-std bolero fuzz --release nodes::behaviors::databases::database::tests::insert_movement_edge_different_source_and_target -t 100sec -s thread

WARNING: ThreadSanitizer: data race (pid=10131)
  Write of size 8 at 0x7b0c00000030 by main thread:
    #0 pthread_cond_destroy /rustc/llvm/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:1212 (bit_network_library-4f9b998d20e451bb+0x1352d75)
    #1 std::sys::unix::condvar::Condvar::destroy /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/sys/unix/condvar.rs:160 (bit_network_library-4f9b998d20e451bb+0x4c4d9aa)
    #2 std::sys_common::condvar::Condvar::destroy /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/sys_common/condvar.rs:70 (bit_network_library-4f9b998d20e451bb+0x4c4d9aa)
    #3 <std::sync::condvar::Condvar as core::ops::drop::Drop>::drop /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/sync/condvar.rs:598 (bit_network_library-4f9b998d20e451bb+0x4c4d9aa)
    #4 core::ptr::drop_in_place /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/core/src/ptr/mod.rs:184 (bit_network_library-4f9b998d20e451bb+0x4c4d9aa)
    #5 core::ptr::drop_in_place /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/core/src/ptr/mod.rs:184 (bit_network_library-4f9b998d20e451bb+0x4c4d9aa)
    #6 alloc::sync::Arc<T>::drop_slow /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/alloc/src/sync.rs:934 (bit_network_library-4f9b998d20e451bb+0x4c4d9aa)

  Previous read of size 8 at 0x7b0c00000030 by thread T1:
    #0 pthread_cond_signal /rustc/llvm/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:1198 (bit_network_library-4f9b998d20e451bb+0x1352a52)
    #1 std::sync::mpsc::blocking::SignalToken::signal /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/sync/mpsc/blocking.rs:41 (bit_network_library-4f9b998d20e451bb+0x4c7435e)

  Location is heap block of size 48 at 0x7b0c00000030 allocated by main thread:
    #0 malloc /rustc/llvm/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:652 (bit_network_library-4f9b998d20e451bb+0x1350184)
    #1 alloc::alloc::alloc /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/alloc/src/alloc.rs:74 (bit_network_library-4f9b998d20e451bb+0x4c588a2)
    #2 alloc::alloc::Global::alloc_impl /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/alloc/src/alloc.rs:153 (bit_network_library-4f9b998d20e451bb+0x4c588a2)
    #3 <alloc::alloc::Global as core::alloc::AllocRef>::alloc /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/alloc/src/alloc.rs:212 (bit_network_library-4f9b998d20e451bb+0x4c588a2)
    #4 alloc::alloc::exchange_malloc /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/alloc/src/alloc.rs:302 (bit_network_library-4f9b998d20e451bb+0x4c588a2)
    #5 std::sync::condvar::Condvar::new /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/sync/condvar.rs:130 (bit_network_library-4f9b998d20e451bb+0x4c588a2)
    #6 std::thread::Thread::new /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/thread/mod.rs:1108 (bit_network_library-4f9b998d20e451bb+0x4c588a2)
    #7 main ??:? (bit_network_library-4f9b998d20e451bb+0x261b579)

  Thread T1 'nodes::behavior' (tid=10133, finished) created by main thread at:
    #0 pthread_create /rustc/llvm/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:966 (bit_network_library-4f9b998d20e451bb+0x1351a5b)
    #1 std::sys::unix::thread::Thread::new /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/sys/unix/thread.rs:66 (bit_network_library-4f9b998d20e451bb+0x4c8a84c)
    #2 core::ops::function::FnOnce::call_once /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/core/src/ops/function.rs:227 (bit_network_library-4f9b998d20e451bb+0x2032cea)
    #3 std::sys_common::backtrace::__rust_begin_short_backtrace /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/sys_common/backtrace.rs:137 (bit_network_library-4f9b998d20e451bb+0x2648fc7)
    #4 std::rt::lang_start::{{closure}} /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/std/src/rt.rs:66 (bit_network_library-4f9b998d20e451bb+0x1fe2aaf)
    #5 core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8/library/core/src/ops/function.rs:259 (bit_network_library-4f9b998d20e451bb+0x4c7ba20)
    #6 std::panicking::try::do_call /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/panicking.rs:381 (bit_network_library-4f9b998d20e451bb+0x4c7ba20)
    #7 std::panicking::try /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/panicking.rs:345 (bit_network_library-4f9b998d20e451bb+0x4c7ba20)
    #8 std::panic::catch_unwind /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/panic.rs:382 (bit_network_library-4f9b998d20e451bb+0x4c7ba20)
    #9 std::rt::lang_start_internal /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/rt.rs:51 (bit_network_library-4f9b998d20e451bb+0x4c7ba20)
    #10 main ??:? (bit_network_library-4f9b998d20e451bb+0x261b579)

SUMMARY: ThreadSanitizer: data race /rustc/d006f5734f49625c34d6fc33bf6b9967243abca8//library/std/src/sys/unix/condvar.rs:160 in std::sys::unix::condvar::Condvar::destroy
==================
ThreadSanitizer: reported 1 warnings
error: test failed, to rerun pass '--lib'
error: process exited with status 66
Command exited with non-zero status 1
1530.03user 28.28system 4:39.38elapsed 557%CPU (0avgtext+0avgdata 4674340maxresident)k
0inputs+4731856outputs (683major+8137732minor)pagefaults 0swaps
