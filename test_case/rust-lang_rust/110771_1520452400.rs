
(gdb) bt
#0  syscall () at ../sysdeps/unix/sysv/linux/x86_64/syscall.S:38
#1  0x00007fafc1163de5 in std::sys::unix::futex::futex_wait () at library/std/src/sys/unix/futex.rs:62
#2  0x00007fafc115c3a0 in std::sys::unix::locks::futex_mutex::Mutex::lock_contended () at library/std/src/sys/unix/locks/futex_mutex.rs:56
#3  0x00007fafc1161986 in std::sys::unix::locks::futex_mutex::Mutex::lock () at library/std/src/sys/unix/locks/futex_mutex.rs:28
#4  std::sync::mutex::Mutex::lock<()> () at library/std/src/sync/mutex.rs:273
#5  0x00007fafc1174d65 in std::sys_common::backtrace::lock () at library/std/src/sys_common/backtrace.rs:17
#6  std::sys_common::backtrace::print () at library/std/src/sys_common/backtrace.rs:33
#7  0x00007fafc114a8f6 in std::panicking::default_hook::{closure#1} () at library/std/src/panicking.rs:274
#8  0x00007fafc114a52f in std::panicking::default_hook () at library/std/src/panicking.rs:293
#9  0x00007fafc4cbe7ff in rustc_driver_impl::DEFAULT_HOOK::{closure#0}::{closure#0} () at compiler/rustc_driver_impl/src/lib.rs:1208
#10 core::ops::function::FnOnce::call_once<rustc_driver_impl::DEFAULT_HOOK::{closure#0}::{closure_env#0}, (&core::panic::panic_info::PanicInfo)> () at library/core/src/ops/function.rs:250
#11 core::ops::function::FnOnce::call_once<rustc_driver_impl::DEFAULT_HOOK::{closure#0}::{closure_env#0}, (&core::panic::panic_info::PanicInfo)> () at library/core/src/ops/function.rs:250
#12 0x00007fafc114b0d3 in std::panicking::rust_panic_with_hook () at library/std/src/panicking.rs:704
#13 0x00007fafc117558f in std::panicking::begin_panic_handler::{closure#0} () at library/std/src/panicking.rs:586
#14 0x00007fafc1175086 in std::sys_common::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::{closure_env#0}, !> () at library/std/src/sys_common/backtrace.rs:150
#15 0x00007fafc114aa92 in std::panicking::begin_panic_handler () at library/std/src/panicking.rs:584
#16 0x00007fafc11c3843 in core::panicking::panic_fmt () at library/core/src/panicking.rs:67
#17 0x00007fafc11c38dd in core::panicking::panic () at library/core/src/panicking.rs:117
#18 0x00007fafc1152dd9 in alloc::vec::Vec::extend_desugared<std::backtrace_rs::symbolize::gimli::elf::ParsedSym, alloc::alloc::Global, core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<object::elf::Sym64<object::endian::LittleEndian>>, std::backtrace_rs::symbolize::gimli::elf::{impl#1}::parse::{closure_env#0}>, std::backtrace_rs::symbolize::gimli::elf::{impl#1}::parse::{closure_env#1}>, std::backtrace_rs::symbolize::gimli::elf::{impl#1}::parse::{closure_env#2}>> () at library/alloc/src/vec/mod.rs:2827
#19 alloc::vec::spec_extend::{impl#0}::spec_extend<std::backtrace_rs::symbolize::gimli::elf::ParsedSym, core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter::Filter<core::slice::iter::Iter<object::elf::Sym64<object::endian::LittleEndian>>, std::backtrace_rs::symbolize::gimli::elf::{impl#1}::parse::{closure_env#0}>, std::backtrace_rs::symbolize::gimli::elf::{impl#1}::parse::{closure_env#1}>, std::backtrace_rs::symbolize::gimli::elf::{impl#1}::parse::{closure_env#2}>, alloc::alloc::Global> () at library/alloc/src/vec/spec_extend.rs:17
