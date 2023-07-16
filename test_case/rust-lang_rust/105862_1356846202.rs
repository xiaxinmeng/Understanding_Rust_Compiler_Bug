plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
Test executable failed (exit status: 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"this an old"`,
 right: `"than an old"`', src/str.rs:7:1
stack backtrace:
   0:     0x55bfbba32542 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h62d61b912b6c1e0a
   1:     0x55bfbba4e428 - core::fmt::write::h0cd9f5419c66d611
   2:     0x55bfbba30851 - std::io::Write::write_fmt::hdf8da66e4a05be95
   3:     0x55bfbba32305 - std::sys_common::backtrace::print::h11723a34e1ab4b18
   4:     0x55bfbba33cc7 - std::panicking::default_hook::{{closure}}::h5d3bed7a6b7b491a
   5:     0x55bfbba33a25 - std::panicking::default_hook::h4fdd417709624f4e
   6:     0x55bfbba344c2 - std::panicking::rust_panic_with_hook::h42b5aa8dbb667dc4
   7:     0x55bfbba34217 - std::panicking::begin_panic_handler::{{closure}}::h84cf78d9ad8c5dc1
   8:     0x55bfbba32aec - std::sys_common::backtrace::__rust_end_short_backtrace::hbeb7f776584da27b
   9:     0x55bfbba33ee2 - rust_begin_unwind
  10:     0x55bfbba0b643 - core::panicking::panic_fmt::ha29375e42f9e82fb
  11:     0x55bfbba0b89b - core::panicking::assert_failed_inner::hc36543c12f11956e
error: doctest failed, to rerun pass `-p alloc --doc`
  12:     0x55bfbba0f5ca - core::panicking::assert_failed::ha03885c532c68a6c
  13:     0x55bfbba11e5d - rust_out::main::_doctest_main_src_str_rs_252_0::hbfd7d4edf603e01d
  14:     0x55bfbba11c76 - rust_out::main::h0bc7401e89f1786c
  15:     0x55bfbba0c6e3 - core::ops::function::FnOnce::call_once::hb4cba86133d25887
  16:     0x55bfbba0bcb9 - std::sys_common::backtrace::__rust_begin_short_backtrace::hdded411ff16f8e62
  17:     0x55bfbba0bd09 - std::rt::lang_start::{{closure}}::hf4702bad861fc642
  18:     0x55bfbba2e534 - std::rt::lang_start_internal::h3498b0692dde0103
  19:     0x55bfbba0bce7 - std::rt::lang_start::hbc90175682df88da
  20:     0x55bfbba11e85 - main
  21:     0x7f3dc58adc87 - __libc_start_main
  22:     0x55bfbba0bb5a - _start
  23:                0x0 - <unknown>


failures:
    src/str.rs - str::str::replace (line 252)
