
[--snip--]$ export RUST_BACKTRACE=full
[--snip--]$ cargo build --release
thread 'main' panicked at 'supplied instant is later than self', library/std/src/time.rs:311:48
stack backtrace:
   0:     0x559af5b0ca0c - std::backtrace_rs::backtrace::libunwind::trace::hf6a6dfd7da937cb0
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x559af5b0ca0c - std::backtrace_rs::backtrace::trace_unsynchronized::hc596a19e4891f7f3
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x559af5b0ca0c - std::sys_common::backtrace::_print_fmt::hb16700db31584325
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x559af5b0ca0c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h231c4190cfa75162
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x559af5b35ecc - core::fmt::write::h2a1462b5f8eea807
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/fmt/mod.rs:1163:17
   5:     0x559af5b038c5 - std::io::Write::write_fmt::h71ddfebc68685972
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/io/mod.rs:1696:15
   6:     0x559af5b0f1d0 - std::sys_common::backtrace::_print::hcc197d4bebf2b369
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x559af5b0f1d0 - std::sys_common::backtrace::print::h335a66af06738c7c
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x559af5b0f1d0 - std::panicking::default_hook::{{closure}}::h6fac9ac9c8b79e52
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:210:50
   9:     0x559af5b0ed85 - std::panicking::default_hook::h341c1030c6a1161b
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:227:9
  10:     0x559af5b0f884 - std::panicking::rust_panic_with_hook::h50680ff4b44510c6
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:624:17
  11:     0x559af5b0f360 - std::panicking::begin_panic_handler::{{closure}}::h9371c0fbb1e8465a
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:521:13
  12:     0x559af5b0ceb4 - std::sys_common::backtrace::__rust_end_short_backtrace::h9b3efa22a5768c0f
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:139:18
  13:     0x559af5b0f2c9 - rust_begin_unwind
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:517:5
  14:     0x559af5179941 - core::panicking::panic_fmt::h23b9203e89cc61cf
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:100:14
  15:     0x559af5b34091 - core::panicking::panic_display::h0b2b4cc4e8e79e2e
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:64:5
  16:     0x559af517983b - core::option::expect_failed::h111c62e88059319b
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/option.rs:1638:5
  17:     0x559af5b0beab - core::option::Option<T>::expect::hc706a79e52c15325
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/option.rs:709:21
  18:     0x559af5b0beab - std::time::Instant::duration_since::hb291e035d51e9a9f
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/time.rs:311:9
  19:     0x559af5b0beab - <std::time::Instant as core::ops::arith::Sub>::sub::h1895890ba2463a9a
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/time.rs:436:9
  20:     0x559af5b0beab - std::time::Instant::elapsed::he89b003a6bece5e1
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/time.rs:375:9
  21:     0x559af558f22a - cargo::core::resolver::activate::he3073cf48b38292d
  22:     0x559af558c47d - cargo::core::resolver::activate_deps_loop::h30f2ed39d81e30b1
  23:     0x559af5588c27 - cargo::core::resolver::resolve::ha4518cd4a67a1003
  24:     0x559af5601c12 - cargo::ops::resolve::resolve_with_previous::h0569bc1629ee2886
  25:     0x559af55ff17d - cargo::ops::resolve::resolve_with_registry::ha8c1db2d89dabd63
  26:     0x559af55fde93 - cargo::ops::resolve::resolve_ws_with_opts::h54d7552e5eb48a42
  27:     0x559af5546b7e - cargo::ops::cargo_compile::create_bcx::he669e2bf2a090bf8
  28:     0x559af5545dae - cargo::ops::cargo_compile::compile_ws::h1d30b9665da49d50
  29:     0x559af5545ce6 - cargo::ops::cargo_compile::compile::h2fa80f04f6323616
  30:     0x559af51ec1c1 - cargo::commands::build::exec::h97328af9497e368f
  31:     0x559af51ab13f - cargo::cli::main::hd285ea351ad27140
  32:     0x559af51edab4 - cargo::main::h8ccca51d449430d5
  33:     0x559af51c2ba3 - std::sys_common::backtrace::__rust_begin_short_backtrace::h0157dc230135dd33
  34:     0x559af51e5549 - std::rt::lang_start::{{closure}}::h7f7be114acd6d212
  35:     0x559af5b0c6eb - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::hc56adab7a77ec6e3
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/ops/function.rs:259:13
  36:     0x559af5b0c6eb - std::panicking::try::do_call::h29f013120c5abc65
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:403:40
  37:     0x559af5b0c6eb - std::panicking::try::h86d5b2b66caec4cf
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:367:19
  38:     0x559af5b0c6eb - std::panic::catch_unwind::h7dd136d787f51397
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panic.rs:133:14
  39:     0x559af5b0c6eb - std::rt::lang_start_internal::{{closure}}::h4a199351e630a8a5
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/rt.rs:128:48
  40:     0x559af5b0c6eb - std::panicking::try::do_call::h20ceb9e5dff838c6
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:403:40
  41:     0x559af5b0c6eb - std::panicking::try::hc2abb46a5e41bd43
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:367:19
  42:     0x559af5b0c6eb - std::panic::catch_unwind::h52c3eb4408ad6dfb
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panic.rs:133:14
  43:     0x559af5b0c6eb - std::rt::lang_start_internal::hd15a47be08101c28
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/rt.rs:128:20
  44:     0x559af51efd22 - main
  45:     0x7f3a01cd413a - __libc_start_main
  46:     0x559af517a161 - <unknown>
