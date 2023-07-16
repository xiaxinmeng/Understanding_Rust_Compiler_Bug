
#0  std::panicking::rust_panic_with_hook (msg=..., file_line=<optimized out>) at /root/rust-nightly/src/libstd/panicking.rs:536
#1  0xb6e63924 in std::panicking::begin_panic<collections::string::String> (msg=..., file_line=0xbeffeb94) at /root/rust-nightly/src/libstd/panicking.rs:511
#2  0xb6e638d4 in std::panicking::begin_panic_fmt (msg=<optimized out>, file_line=0x0) at /root/rust-nightly/src/libstd/panicking.rs:495
#3  0xb6e63880 in std::panicking::rust_begin_panic (msg=..., file=..., line=859) at /root/rust-nightly/src/libstd/panicking.rs:471
#4  0xb6e7b910 in core::panicking::panic_fmt (fmt=..., file_line=<optimized out>) at /root/rust-nightly/src/libcore/panicking.rs:69
#5  0xb6e3cafc in core::result::unwrap_failed<core::cell::BorrowMutError> (msg=..., error=...) at /root/rust-nightly/src/libcore/macros.rs:29
#6  0xb6e58340 in core::result::Result<core::cell::RefMut<core::option::Option<std::sys_common::thread_info::ThreadInfo>>, core::cell::BorrowMutError>::expect<core::cell::RefMut<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,core::cell::BorrowMutError> (msg=..., self=...) at /root/rust-nightly/src/libcore/result.rs:761
#7  core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>::borrow_mut<core::option::Option<std::sys_common::thread_info::ThreadInfo>> (self=<optimized out>) at /root/rust-nightly/src/libcore/cell.rs:736
#8  std::sys_common::thread_info::{{impl}}::with::{{closure}}<std::thread::Thread,closure> (c=<optimized out>) at /root/rust-nightly/src/libstd/sys_common/thread_info.rs:32
#9  0xb6e6332c in std::thread::local::LocalKey<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>>::with<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,closure,core::option::Option<std::thread::Thread>> (self=<optimized out>, f=...) at /root/rust-nightly/src/libstd/thread/local.rs:253
#10 std::sys_common::thread_info::ThreadInfo::with<std::thread::Thread,closure> (f=...) at /root/rust-nightly/src/libstd/sys_common/thread_info.rs:30
#11 std::sys_common::thread_info::current_thread () at /root/rust-nightly/src/libstd/sys_common/thread_info.rs:43
#12 std::panicking::default_hook (info=<optimized out>) at /root/rust-nightly/src/libstd/panicking.rs:341
#13 0xb6e63ab8 in std::panicking::rust_panic_with_hook (msg=..., file_line=<optimized out>) at /root/rust-nightly/src/libstd/panicking.rs:549
#14 0xb6e63924 in std::panicking::begin_panic<collections::string::String> (msg=..., file_line=0xbeffed84) at /root/rust-nightly/src/libstd/panicking.rs:511
#15 0xb6e638d4 in std::panicking::begin_panic_fmt (msg=<optimized out>, file_line=0x0) at /root/rust-nightly/src/libstd/panicking.rs:495
#16 0xb6e63880 in std::panicking::rust_begin_panic (msg=..., file=..., line=859) at /root/rust-nightly/src/libstd/panicking.rs:471
#17 0xb6e7b910 in core::panicking::panic_fmt (fmt=..., file_line=<optimized out>) at /root/rust-nightly/src/libcore/panicking.rs:69
#18 0xb6e3cafc in core::result::unwrap_failed<core::cell::BorrowMutError> (msg=..., error=...) at /root/rust-nightly/src/libcore/macros.rs:29
#19 0xb6e58340 in core::result::Result<core::cell::RefMut<core::option::Option<std::sys_common::thread_info::ThreadInfo>>, core::cell::BorrowMutError>::expect<core::cell::RefMut<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,core::cell::BorrowMutError> (msg=..., self=...) at /root/rust-nightly/src/libcore/result.rs:761
#20 core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>::borrow_mut<core::option::Option<std::sys_common::thread_info::ThreadInfo>> (self=<optimized out>) at /root/rust-nightly/src/libcore/cell.rs:736
#21 std::sys_common::thread_info::{{impl}}::with::{{closure}}<std::thread::Thread,closure> (c=<optimized out>) at /root/rust-nightly/src/libstd/sys_common/thread_info.rs:32
#22 0xb6e6332c in std::thread::local::LocalKey<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>>::with<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,closure,core::option::Option<std::thread::Thread>> (self=<optimized out>, f=...) at /root/rust-nightly/src/libstd/thread/local.rs:253
#23 std::sys_common::thread_info::ThreadInfo::with<std::thread::Thread,closure> (f=...) at /root/rust-nightly/src/libstd/sys_common/thread_info.rs:30
#24 std::sys_common::thread_info::current_thread () at /root/rust-nightly/src/libstd/sys_common/thread_info.rs:43
#25 std::panicking::default_hook (info=<optimized out>) at /root/rust-nightly/src/libstd/panicking.rs:341
#26 0xb6e63ab8 in std::panicking::rust_panic_with_hook (msg=..., file_line=<optimized out>) at /root/rust-nightly/src/libstd/panicking.rs:549
#27 0xb6e63924 in std::panicking::begin_panic<collections::string::String> (msg=..., file_line=0xbeffef74) at /root/rust-nightly/src/libstd/panicking.rs:511
#28 0xb6e638d4 in std::panicking::begin_panic_fmt (msg=<optimized out>, file_line=0x0) at /root/rust-nightly/src/libstd/panicking.rs:495
#29 0xb6e63880 in std::panicking::rust_begin_panic (msg=..., file=..., line=859) at /root/rust-nightly/src/libstd/panicking.rs:471
#30 0xb6e7b910 in core::panicking::panic_fmt (fmt=..., file_line=<optimized out>) at /root/rust-nightly/src/libcore/panicking.rs:69
#31 0xb6e3cafc in core::result::unwrap_failed<core::cell::BorrowMutError> (msg=..., error=...) at /root/rust-nightly/src/libcore/macros.rs:29
#32 0xb6e64678 in core::result::Result<core::cell::RefMut<core::option::Option<std::sys_common::thread_info::ThreadInfo>>, core::cell::BorrowMutError>::expect<core::cell::RefMut<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,core::cell::BorrowMutError> (self=..., msg=...) at /root/rust-nightly/src/libcore/result.rs:761
#33 core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>::borrow_mut<core::option::Option<std::sys_common::thread_info::ThreadInfo>> (self=<optimized out>) at /root/rust-nightly/src/libcore/cell.rs:736
#34 std::sys_common::thread_info::set::{{closure}} (c=<optimized out>) at /root/rust-nightly/src/libstd/sys_common/thread_info.rs:52
#35 std::thread::local::LocalKey<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>>::with<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,closure,()> (self=<optimized out>, f=...) at /root/rust-nightly/src/libstd/thread/local.rs:253
#36 std::sys_common::thread_info::set (stack_guard=..., thread=...) at /root/rust-nightly/src/libstd/sys_common/thread_info.rs:52
#37 std::rt::lang_start (main=<optimized out>, argc=<optimized out>, argv=<optimized out>) at /root/rust-nightly/src/libstd/rt.rs:51
#38 0xb6c6a73c in __libc_start_main () from /lib/libc.so.6
#39 0x7f5556f8 in _start ()
