
thread '<unnamed>' panicked at 'use of std::thread::current() is not possible after the thread's local data has been destroyed', library/std/src/thread/mod.rs:681:35
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/panicking.rs:142:14
   2: core::panicking::panic_display
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/panicking.rs:72:5
   3: core::panicking::panic_str
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/panicking.rs:56:5
   4: core::option::expect_failed
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/option.rs:1874:5
   5: core::option::Option<T>::expect
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/option.rs:738:21
   6: std::thread::current
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/std/src/thread/mod.rs:681:5
   7: std::thread::park_timeout
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/std/src/thread/mod.rs:997:9
   8: futex::Mutex<T>::lock
             at /Users/garypen/dev/futex/src/lib.rs:58:21
   9: <dhat::Alloc as core::alloc::global::GlobalAlloc>::dealloc
             at ./src/lib.rs:1287:51
  10: alloc::alloc::dealloc
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/alloc/src/alloc.rs:107:14
  11: <alloc::alloc::Global as core::alloc::Allocator>::deallocate
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/alloc/src/alloc.rs:244:22
  12: <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/alloc/src/raw_vec.rs:478:22
  13: core::ptr::drop_in_place<alloc::raw_vec::RawVec<(*mut u8,unsafe extern "C" fn(*mut u8))>>
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/ptr/mod.rs:487:1
  14: <<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<T,A> as core::ops::drop::Drop>::drop
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/alloc/src/vec/into_iter.rs:324:94
  15: core::ptr::drop_in_place<<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<(*mut u8,unsafe extern "C" fn(*mut u8)),alloc::alloc::Global>>
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/ptr/mod.rs:487:1
  16: <alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/alloc/src/vec/into_iter.rs:335:5
  17: core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<(*mut u8,unsafe extern "C" fn(*mut u8))>>
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/ptr/mod.rs:487:1
  18: std::sys::unix::thread_local_dtor::register_dtor::run_dtors
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/std/src/sys/unix/thread_local_dtor.rs:88:13
