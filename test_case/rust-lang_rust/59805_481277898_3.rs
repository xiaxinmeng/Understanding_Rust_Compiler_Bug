
Program received signal SIGILL, Illegal instruction.
0x2a0037f4 in rust_panic_with_hook () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/fmt/mod.rs:316
316	/rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/fmt/mod.rs: No such file or directory.
(gdb) bt
#0  0x2a0037f4 in rust_panic_with_hook () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/fmt/mod.rs:316
#1  0x2a003218 in continue_panic_fmt () at src/libstd/panicking.rs:381
#2  0x2a00befc in rust_begin_unwind () at src/libstd/panicking.rs:308
#3  0x2a00f408 in panic_fmt () at src/libcore/panicking.rs:85
#4  0x2a002bbc in unwrap_failed<core::cell::BorrowMutError> () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/macros.rs:18
#5  0x2a002308 in expect<core::cell::RefMut<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,core::cell::BorrowMutError> () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/result.rs:825
#6  borrow_mut<core::option::Option<std::sys_common::thread_info::ThreadInfo>> () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/cell.rs:873
#7  {{closure}}<std::thread::Thread,closure> () at src/libstd/sys_common/thread_info.rs:18
#8  try_with<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,closure,std::thread::Thread> () at src/libstd/thread/local.rs:299
#9  0x2a003600 in with<std::thread::Thread,closure> () at src/libstd/sys_common/thread_info.rs:16
#10 current_thread () at src/libstd/sys_common/thread_info.rs:29
#11 default_hook () at src/libstd/panicking.rs:183
#12 rust_panic_with_hook () at src/libstd/panicking.rs:474
#13 0x2a003218 in continue_panic_fmt () at src/libstd/panicking.rs:381
#14 0x2a00befc in rust_begin_unwind () at src/libstd/panicking.rs:308
#15 0x2a00f408 in panic_fmt () at src/libcore/panicking.rs:85
#16 0x2a002bbc in unwrap_failed<core::cell::BorrowMutError> () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/macros.rs:18
#17 0x2a0022e8 in expect<core::cell::RefMut<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,core::cell::BorrowMutError> () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/result.rs:825
#18 borrow_mut<core::option::Option<std::sys_common::thread_info::ThreadInfo>> () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/cell.rs:873
#19 {{closure}}<std::thread::Thread,closure> () at src/libstd/sys_common/thread_info.rs:23
#20 try_with<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,closure,std::thread::Thread> () at src/libstd/thread/local.rs:299
#21 0x2a003600 in with<std::thread::Thread,closure> () at src/libstd/sys_common/thread_info.rs:16
#22 current_thread () at src/libstd/sys_common/thread_info.rs:29
#23 default_hook () at src/libstd/panicking.rs:183
#24 rust_panic_with_hook () at src/libstd/panicking.rs:474
#25 0x2a003218 in continue_panic_fmt () at src/libstd/panicking.rs:381
#26 0x2a00befc in rust_begin_unwind () at src/libstd/panicking.rs:308
#27 0x2a00f408 in panic_fmt () at src/libcore/panicking.rs:85
#28 0x2a00265c in unwrap_failed<core::cell::BorrowError> () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/macros.rs:18
#29 0x2a00c520 in expect<core::cell::Ref<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,core::cell::BorrowError> () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/result.rs:825
#30 borrow<core::option::Option<std::sys_common::thread_info::ThreadInfo>> () at /rustc/3750348daff89741e3153e0e120aa70a45ff5b68/src/libcore/cell.rs:792
#31 {{closure}} () at src/libstd/sys_common/thread_info.rs:37
#32 try_with<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,closure,()> () at src/libstd/thread/local.rs:299
#33 with<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,closure,()> () at src/libstd/thread/local.rs:245
#34 set () at src/libstd/sys_common/thread_info.rs:37
#35 lang_start_internal () at src/libstd/rt.rs:41
#36 0x2a0137a0 in main ()
(gdb) 
