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

stderr:
thread 'main' panicked at 'attempt to shift left with overflow', /checkout/library/core/src/num/mod.rs:261:5
stack backtrace:
   0:     0x56047322b9b2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8978151eef092d7d
   1:     0x5604732481b8 - core::fmt::write::h6d6e96066401bc0f
   2:     0x560473229cd1 - std::io::Write::write_fmt::h2ed111527b564c68
   3:     0x56047322b775 - std::sys_common::backtrace::print::he092bdc28d453324
   4:     0x56047322d1b7 - std::panicking::default_hook::{{closure}}::h9a78745f44f61a8c
   5:     0x56047322cf15 - std::panicking::default_hook::h7e5963dff20d1f30
   6:     0x56047322d9b2 - std::panicking::rust_panic_with_hook::hed0ba72ef8e79c54
   7:     0x56047322d6c1 - std::panicking::begin_panic_handler::{{closure}}::haa09fca95c2acddd
   8:     0x56047322bf5c - std::sys_common::backtrace::__rust_end_short_backtrace::h3a2229ee29684132
   9:     0x56047322d3d2 - rust_begin_unwind
  10:     0x56047320aaf3 - core::panicking::panic_fmt::h3a5b5d72039ab650
  11:     0x56047320abcd - core::panicking::panic::h12e64a76e23b19bf
  12:     0x56047320b5a0 - rust_out::main::_doctest_main_src_num_mod_rs_272_5::h32f302eb29edfec0
  13:     0x56047320b296 - rust_out::main::h4e32373a768026d2
  14:     0x56047320b1a3 - core::ops::function::FnOnce::call_once::h1c0c2d9b110d98b1
  15:     0x56047320af29 - std::sys_common::backtrace::__rust_begin_short_backtrace::h16510240dccdd97b
  16:     0x56047320af79 - std::rt::lang_start::{{closure}}::hcf05db6635b504a2
  17:     0x560473227b62 - std::rt::lang_start_internal::h83cc3d5a7bc1c2ee
  18:     0x56047320af57 - std::rt::lang_start::hdc464374a674c57a
  19:     0x56047320c178 - main
  20:     0x7ff1799efc87 - __libc_start_main
  21:     0x56047320ae3a - _start
  22:                0x0 - <unknown>

---- src/num/mod.rs - num::i16::saturating_shl (line 254) stdout ----
Test executable failed (exit status: 101).


stderr:
thread 'main' panicked at 'attempt to shift left with overflow', /checkout/library/core/src/num/mod.rs:243:5
stack backtrace:
   0:     0x561bf382b4d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8978151eef092d7d
   1:     0x561bf38472a8 - core::fmt::write::h6d6e96066401bc0f
   2:     0x561bf38297f1 - std::io::Write::write_fmt::h2ed111527b564c68
   3:     0x561bf382b295 - std::sys_common::backtrace::print::he092bdc28d453324
   4:     0x561bf382ccd7 - std::panicking::default_hook::{{closure}}::h9a78745f44f61a8c
   5:     0x561bf382ca35 - std::panicking::default_hook::h7e5963dff20d1f30
   6:     0x561bf382d4d2 - std::panicking::rust_panic_with_hook::hed0ba72ef8e79c54
   7:     0x561bf382d1e1 - std::panicking::begin_panic_handler::{{closure}}::haa09fca95c2acddd
   8:     0x561bf382ba7c - std::sys_common::backtrace::__rust_end_short_backtrace::h3a2229ee29684132
   9:     0x561bf382cef2 - rust_begin_unwind
  10:     0x561bf380aa33 - core::panicking::panic_fmt::h3a5b5d72039ab650
  11:     0x561bf380ab0d - core::panicking::panic::h12e64a76e23b19bf
  12:     0x561bf380b3f5 - rust_out::main::_doctest_main_src_num_mod_rs_254_0::h1945c3c1c70b6455
  13:     0x561bf380b1d6 - rust_out::main::h4e32373a768026d2
  14:     0x561bf380b0e3 - core::ops::function::FnOnce::call_once::h1c0c2d9b110d98b1
  15:     0x561bf380ae69 - std::sys_common::backtrace::__rust_begin_short_backtrace::h16510240dccdd97b
  16:     0x561bf380aeb9 - std::rt::lang_start::{{closure}}::hcf05db6635b504a2
  17:     0x561bf3827682 - std::rt::lang_start_internal::h83cc3d5a7bc1c2ee
  18:     0x561bf380ae97 - std::rt::lang_start::hdc464374a674c57a
  19:     0x561bf380bc98 - main
  20:     0x7f25bec6fc87 - __libc_start_main
  21:     0x561bf380ad7a - _start
  22:                0x0 - <unknown>

---- src/num/mod.rs - num::i32::saturating_shl (line 259) stdout ----
Test executable failed (exit status: 101).


stderr:
thread 'main' panicked at 'attempt to shift left with overflow', /checkout/library/core/src/num/mod.rs:248:5
stack backtrace:
   0:     0x5641dfa2b492 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8978151eef092d7d
   1:     0x5641dfa47268 - core::fmt::write::h6d6e96066401bc0f
   2:     0x5641dfa297b1 - std::io::Write::write_fmt::h2ed111527b564c68
   3:     0x5641dfa2b255 - std::sys_common::backtrace::print::he092bdc28d453324
   4:     0x5641dfa2cc97 - std::panicking::default_hook::{{closure}}::h9a78745f44f61a8c
   5:     0x5641dfa2c9f5 - std::panicking::default_hook::h7e5963dff20d1f30
   6:     0x5641dfa2d492 - std::panicking::rust_panic_with_hook::hed0ba72ef8e79c54
   7:     0x5641dfa2d1a1 - std::panicking::begin_panic_handler::{{closure}}::haa09fca95c2acddd
   8:     0x5641dfa2ba3c - std::sys_common::backtrace::__rust_end_short_backtrace::h3a2229ee29684132
   9:     0x5641dfa2ceb2 - rust_begin_unwind
  10:     0x5641dfa0a9f3 - core::panicking::panic_fmt::h3a5b5d72039ab650
  11:     0x5641dfa0aacd - core::panicking::panic::h12e64a76e23b19bf
  12:     0x5641dfa0b3ba - rust_out::main::_doctest_main_src_num_mod_rs_259_0::hf48d0d6b103afa6b
  13:     0x5641dfa0b196 - rust_out::main::h4e32373a768026d2
  14:     0x5641dfa0b0a3 - core::ops::function::FnOnce::call_once::h1c0c2d9b110d98b1
  15:     0x5641dfa0ae29 - std::sys_common::backtrace::__rust_begin_short_backtrace::h16510240dccdd97b
  16:     0x5641dfa0ae79 - std::rt::lang_start::{{closure}}::hcf05db6635b504a2
  17:     0x5641dfa27642 - std::rt::lang_start_internal::h83cc3d5a7bc1c2ee
  18:     0x5641dfa0ae57 - std::rt::lang_start::hdc464374a674c57a
  19:     0x5641dfa0bc58 - main
  20:     0x7f873e9cec87 - __libc_start_main
  21:     0x5641dfa0ad3a - _start
  22:                0x0 - <unknown>

---- src/num/mod.rs - num::i64::saturating_shl (line 265) stdout ----
Test executable failed (exit status: 101).


stderr:
thread 'main' panicked at 'attempt to shift left with overflow', /checkout/library/core/src/num/mod.rs:254:5
stack backtrace:
   0:     0x557090e2b5c2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8978151eef092d7d
   1:     0x557090e47398 - core::fmt::write::h6d6e96066401bc0f
   2:     0x557090e298e1 - std::io::Write::write_fmt::h2ed111527b564c68
   3:     0x557090e2b385 - std::sys_common::backtrace::print::he092bdc28d453324
   4:     0x557090e2cdc7 - std::panicking::default_hook::{{closure}}::h9a78745f44f61a8c
   5:     0x557090e2cb25 - std::panicking::default_hook::h7e5963dff20d1f30
   6:     0x557090e2d5c2 - std::panicking::rust_panic_with_hook::hed0ba72ef8e79c54
   7:     0x557090e2d2d1 - std::panicking::begin_panic_handler::{{closure}}::haa09fca95c2acddd
   8:     0x557090e2bb6c - std::sys_common::backtrace::__rust_end_short_backtrace::h3a2229ee29684132
   9:     0x557090e2cfe2 - rust_begin_unwind
  10:     0x557090e0aa33 - core::panicking::panic_fmt::h3a5b5d72039ab650
  11:     0x557090e0ab0d - core::panicking::panic::h12e64a76e23b19bf
  12:     0x557090e0b423 - rust_out::main::_doctest_main_src_num_mod_rs_265_0::hdab4f8097f9d0653
  13:     0x557090e0b1d6 - rust_out::main::h4e32373a768026d2
  14:     0x557090e0b0e3 - core::ops::function::FnOnce::call_once::h1c0c2d9b110d98b1
  15:     0x557090e0ae69 - std::sys_common::backtrace::__rust_begin_short_backtrace::h16510240dccdd97b
  16:     0x557090e0aeb9 - std::rt::lang_start::{{closure}}::hcf05db6635b504a2
  17:     0x557090e27772 - std::rt::lang_start_internal::h83cc3d5a7bc1c2ee
  18:     0x557090e0ae97 - std::rt::lang_start::hdc464374a674c57a
  20:     0x7fd554c46c87 - __libc_start_main
  21:     0x557090e0ad7a - _start
  22:                0x0 - <unknown>



---- src/num/mod.rs - num::i8::saturating_shl (line 249) stdout ----
Test executable failed (exit status: 101).

stderr:
thread 'main' panicked at 'attempt to shift left with overflow', /checkout/library/core/src/num/mod.rs:238:5
stack backtrace:
   0:     0x560fe642b452 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8978151eef092d7d
   1:     0x560fe6447228 - core::fmt::write::h6d6e96066401bc0f
   2:     0x560fe6429771 - std::io::Write::write_fmt::h2ed111527b564c68
   3:     0x560fe642b215 - std::sys_common::backtrace::print::he092bdc28d453324
   4:     0x560fe642cc57 - std::panicking::default_hook::{{closure}}::h9a78745f44f61a8c
   5:     0x560fe642c9b5 - std::panicking::default_hook::h7e5963dff20d1f30
   6:     0x560fe642d452 - std::panicking::rust_panic_with_hook::hed0ba72ef8e79c54
   7:     0x560fe642d161 - std::panicking::begin_panic_handler::{{closure}}::haa09fca95c2acddd
   8:     0x560fe642b9fc - std::sys_common::backtrace::__rust_end_short_backtrace::h3a2229ee29684132
   9:     0x560fe642ce72 - rust_begin_unwind
  10:     0x560fe640aa33 - core::panicking::panic_fmt::h3a5b5d72039ab650
  11:     0x560fe640ab0d - core::panicking::panic::h12e64a76e23b19bf
error: doctest failed, to rerun pass `-p core --doc`
  12:     0x560fe640b3d9 - rust_out::main::_doctest_main_src_num_mod_rs_249_0::h7ac12ec6d5a80c1c
  13:     0x560fe640b1d6 - rust_out::main::h4e32373a768026d2
  14:     0x560fe640b0e3 - core::ops::function::FnOnce::call_once::h1c0c2d9b110d98b1
  15:     0x560fe640ae69 - std::sys_common::backtrace::__rust_begin_short_backtrace::h16510240dccdd97b
  16:     0x560fe640aeb9 - std::rt::lang_start::{{closure}}::hcf05db6635b504a2
  17:     0x560fe6427602 - std::rt::lang_start_internal::h83cc3d5a7bc1c2ee
  18:     0x560fe640ae97 - std::rt::lang_start::hdc464374a674c57a
  19:     0x560fe640bc18 - main
  20:     0x7f5cb7c3ec87 - __libc_start_main
  21:     0x560fe640ad7a - _start
  22:                0x0 - <unknown>

---- src/num/mod.rs - num::isize::saturating_shl (line 301) stdout ----
Test executable failed (exit status: 101).


stderr:
thread 'main' panicked at 'attempt to shift left with overflow', /checkout/library/core/src/num/mod.rs:290:5
stack backtrace:
   0:     0x562c74e2b5c2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8978151eef092d7d
   1:     0x562c74e47398 - core::fmt::write::h6d6e96066401bc0f
   2:     0x562c74e298e1 - std::io::Write::write_fmt::h2ed111527b564c68
   3:     0x562c74e2b385 - std::sys_common::backtrace::print::he092bdc28d453324
   4:     0x562c74e2cdc7 - std::panicking::default_hook::{{closure}}::h9a78745f44f61a8c
   5:     0x562c74e2cb25 - std::panicking::default_hook::h7e5963dff20d1f30
   6:     0x562c74e2d5c2 - std::panicking::rust_panic_with_hook::hed0ba72ef8e79c54
   7:     0x562c74e2d2d1 - std::panicking::begin_panic_handler::{{closure}}::haa09fca95c2acddd
   8:     0x562c74e2bb6c - std::sys_common::backtrace::__rust_end_short_backtrace::h3a2229ee29684132
   9:     0x562c74e2cfe2 - rust_begin_unwind
  10:     0x562c74e0aa33 - core::panicking::panic_fmt::h3a5b5d72039ab650
  11:     0x562c74e0ab0d - core::panicking::panic::h12e64a76e23b19bf
  12:     0x562c74e0b423 - rust_out::main::_doctest_main_src_num_mod_rs_301_0::h5cd7f0f6a17a9815
  13:     0x562c74e0b1d6 - rust_out::main::h4e32373a768026d2
  14:     0x562c74e0b0e3 - core::ops::function::FnOnce::call_once::h1c0c2d9b110d98b1
  15:     0x562c74e0ae69 - std::sys_common::backtrace::__rust_begin_short_backtrace::h16510240dccdd97b
  16:     0x562c74e0aeb9 - std::rt::lang_start::{{closure}}::hcf05db6635b504a2
  17:     0x562c74e27772 - std::rt::lang_start_internal::h83cc3d5a7bc1c2ee
  18:     0x562c74e0ae97 - std::rt::lang_start::hdc464374a674c57a
  19:     0x562c74e0bd88 - main
  20:     0x7f6a6c2c9c87 - __libc_start_main
  21:     0x562c74e0ad7a - _start
  22:                0x0 - <unknown>


failures:
    src/num/mod.rs - num::i128::saturating_shl (line 272)
