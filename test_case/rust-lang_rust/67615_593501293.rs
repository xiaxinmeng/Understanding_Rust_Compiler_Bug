
error: internal compiler error: src/librustc/ty/instance.rs:85: Instance.ty called for type fn() -> u32 {closures::unused::<T>} with params in substs: [T]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:885:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/david/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /home/david/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1053
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1428
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
fish: Process 25930, “rustc” “rustc +rust0-stage1 src/test/co…” terminated by signal SIGSEGV (Address boundary error)
