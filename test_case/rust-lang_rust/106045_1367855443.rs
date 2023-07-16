rust
#0  __GI___libc_malloc (bytes=1) at malloc.c:3031
#1  0x000055555556d5f9 in alloc::alloc::alloc () at library/alloc/src/alloc.rs:99
#2  alloc::alloc::Global::alloc_impl () at library/alloc/src/alloc.rs:181
#3  alloc::alloc::{impl#1}::allocate () at library/alloc/src/alloc.rs:241
#4  alloc::raw_vec::RawVec::allocate_in<u8, alloc::alloc::Global> () at library/alloc/src/raw_vec.rs:185
#5  alloc::raw_vec::RawVec::with_capacity_in<u8, alloc::alloc::Global> () at library/alloc/src/raw_vec.rs:131
#6  alloc::vec::Vec::with_capacity_in<u8, alloc::alloc::Global> () at library/alloc/src/vec/mod.rs:673
#7  alloc::slice::hack::{impl#1}::to_vec<u8, alloc::alloc::Global> () at library/alloc/src/slice.rs:157
#8  alloc::slice::hack::to_vec<u8, alloc::alloc::Global> () at library/alloc/src/slice.rs:106
#9  alloc::slice::{impl#0}::to_vec_in<u8, alloc::alloc::Global> () at library/alloc/src/slice.rs:436
#10 alloc::slice::{impl#0}::to_vec<u8> () at library/alloc/src/slice.rs:411
#11 std::sys::unix::os::getenv () at library/std/src/sys/unix/os.rs:557
#12 std::env::_var_os () at library/std/src/env.rs:273
#13 0x000055555556e50f in std::env::var_os<&str> () at library/std/src/env.rs:269
#14 std::panic::get_backtrace_style () at library/std/src/panic.rs:298
#15 0x0000555555570d63 in std::panicking::default_hook () at library/std/src/panicking.rs:241
#16 0x0000555555571948 in std::panicking::rust_panic_with_hook () at library/std/src/panicking.rs:688
#17 0x000055555555aa34 in std::panicking::begin_panic::{{closure}} ()
#18 0x000055555555b2a5 in std::sys_common::backtrace::__rust_end_short_backtrace ()
#19 0x000055555555a97f in std::panicking::begin_panic ()
#20 0x000055555555a929 in main ()
