
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:335
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:365
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:511
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:495
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:471
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic
             at /checkout/src/libcore/panicking.rs:51
  10: rustdoc::html::format::resolved_path
  11: core::fmt::write
             at /checkout/src/libcore/fmt/mod.rs:954
  12: core::fmt::Formatter::write_fmt
             at /checkout/src/libcore/fmt/mod.rs:1262
  13: rustdoc::html::format::<impl core::fmt::Display for rustdoc::clean::Import>::fmt
  14: core::fmt::write
             at /checkout/src/libcore/fmt/mod.rs:954
  15: core::fmt::Formatter::write_fmt
             at /checkout/src/libcore/fmt/mod.rs:1262
  16: <rustdoc::html::render::Item<'a> as core::fmt::Display>::fmt
  17: core::fmt::write
             at /checkout/src/libcore/fmt/mod.rs:994
             at /checkout/src/libcore/fmt/mod.rs:962
  18: std::io::Write::write_fmt
  19: rustdoc::html::render::Context::render_item
  20: rustdoc::html::render::Context::item::{{closure}}
  21: rustdoc::html::render::run
