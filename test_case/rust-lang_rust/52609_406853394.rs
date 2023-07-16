
thread 'main' panicked at 'unexpected getrandom error: 14', libstd/sys/unix/rand.rs:56:21
stack backtrace:
   0:   0x7dcf83 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h04ab691e2d5a2a25
                       at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:   0x7e2e23 - std::sys_common::backtrace::print::hac888a06b71fcb05
                       at libstd/sys_common/backtrace.rs:71
                       at libstd/sys_common/backtrace.rs:59
   2:   0x7c84d3 - std::panicking::default_hook::{{closure}}::h726fe9dee04cfca4
                       at libstd/panicking.rs:211
   3:   0x7c817b - std::panicking::default_hook::h50a9ec73820d3541
                       at libstd/panicking.rs:227
   4:   0x7c8b97 - std::panicking::rust_panic_with_hook::ha5eff468dabed44e
                       at libstd/panicking.rs:463
   5:   0x7c8723 - std::panicking::begin_panic_fmt::h127c95263e9d91e1
                       at libstd/panicking.rs:350
   6:   0x7d2323 - std::sys::unix::rand::imp::fill_bytes::h107c175968024ac6
                       at libstd/sys/unix/rand.rs:56
                       at libstd/sys/unix/rand.rs:101
   7:   0x7daaaf - std::sys::unix::rand::hashmap_random_keys::h7d2ea6e457897731
                       at libstd/sys/unix/rand.rs:19
   8:   0x2a99bb - <clap::args::arg_matches::ArgMatches<'a> as core::default::Default>::default::hdc9a1ee68179ea39
   9:   0x2a8d17 - clap::app::App::get_matches_from_safe_borrow::h9dd85a73ec350cce
  10:   0x2a867f - clap::app::App::get_matches::hd0be77d101c2233a
  11:    0xe2c03 - lanlord::main::hf5a0300982924cc3
  12:   0x17d6ff - std::rt::lang_start::{{closure}}::hf87301db5ac10996
  13:   0x7c862b - std::panicking::try::do_call::hbdb89cc4df2089c8
                       at libstd/rt.rs:59
                       at libstd/panicking.rs:310
  14:   0x7ea81f - __rust_maybe_catch_panic
                       at libpanic_unwind/lib.rs:105
  15:   0x7ca263 - std::rt::lang_start_internal::hbbf5e767c9c6dfac
                       at libstd/panicking.rs:289
                       at libstd/panic.rs:374
                       at libstd/rt.rs:58
  16:    0xe7d13 - main
