
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /build/rustc-1.32.0-src/src/libcore/slice/mod.rs:2463:10
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::panicking::default_hook::{{closure}}
   2: std::panicking::rust_panic_with_hook
   3: std::panicking::continue_panic_fmt
   4: rust_begin_unwind
   5: core::panicking::panic_fmt
   6: core::panicking::panic_bounds_check
   7: zoneinfo_compiled::CompiledData::from_file
   8: exa::options::view::<impl exa::output::View>::deduce
   9: exa::options::Options::parse
  10: exa::main
  11: std::rt::lang_start::{{closure}}
  12: main
  13: __libc_start_main
  14: _start
czsh: abort (core dumped)  exa -l --group-directories-first --sort=extension
