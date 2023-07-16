plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
Successfully built 40800a060c64
Successfully tagged rust-ci:latest
Built container sha256:40800a060c64d323dcf7f1febc441e740fc5a71c7184e9906d10f7d6f310b131
Uploading finished image to https://ci-caches.rust-lang.org/docker/2022246eea0fceb8dd4e4dc9e56d028016fd8dd73ce83342144a1d278bb5faf6c136af0476a9cbe3f734e6749b99181e6738ca41b0f413c1eb4f076dd7a84292
upload failed: - to s3://rust-lang-ci-sccache2/docker/2022246eea0fceb8dd4e4dc9e56d028016fd8dd73ce83342144a1d278bb5faf6c136af0476a9cbe3f734e6749b99181e6738ca41b0f413c1eb4f076dd7a84292 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---

actual output differed from expected
--- tests/pass/miri-alloc.stderr
+++ <stderr output>
+thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_const_eval/src/interpret/memory.rs:LL:CC
+stack backtrace:
+   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h857c485298a35cfc
+   1: core::fmt::write::h1692ea36034ef62d
+   2: std::io::Write::write_fmt::h5275431ba0b03075
+   3: std::sys_common::backtrace::print::hbaebd0b6a97b2bdc
+   4: std::panicking::default_hook::{{closure}}::h62c36ca7853dc291
+   5: std::panicking::default_hook::h63b797dbd5303937
+   6: rustc_driver_impl[30900129d6928322]::DEFAULT_HOOK::{closure#0}::{closure#0}
+   7: std::panicking::rust_panic_with_hook::h34c6253620675739
+   8: std::panicking::begin_panic_handler::{{closure}}::h9e41015fba81e59e
+   9: std::sys_common::backtrace::__rust_end_short_backtrace::h8f739472cbbe64ed
+  11: core::panicking::panic_fmt::hec968b555ef4d9c1
+  11: core::panicking::panic_fmt::hec968b555ef4d9c1
+  12: core::panicking::panic::h7639f9e87c66325a
+  13: <rustc_const_eval[15cf6ae4845e40ca]::interpret::eval_context::InterpCx<miri[bae29999596f13bc]::machine::MiriMachine>>::allocate_raw_ptr
+  14: <rustc_const_eval[15cf6ae4845e40ca]::interpret::eval_context::InterpCx<miri[bae29999596f13bc]::machine::MiriMachine> as miri[bae29999596f13bc]::shims::env::EvalContextExt>::update_environ
+  15: <miri[bae29999596f13bc]::machine::MiriMachine>::late_init
+  16: miri[bae29999596f13bc]::eval::create_ecx
+  17: miri[bae29999596f13bc]::eval::eval_entry
+  18: <rustc_middle[e73d1a046df07a29]::ty::context::GlobalCtxt>::enter
+  19: <miri[b413ac633ed0375a]::MiriCompilerCalls as rustc_driver_impl[30900129d6928322]::Callbacks>::after_analysis
+  20: <rustc_interface[a21a6190ce08c83a]::interface::Compiler>::enter
+  21: rustc_span[cdbb012465937b99]::with_source_map
+  22: <scoped_tls[8c44f6ff39b30028]::ScopedKey<rustc_span[cdbb012465937b99]::SessionGlobals>>::set
+  23: std[9bff0da3a391513c]::sys_common::backtrace::__rust_begin_short_backtrace
+  24: <<std[9bff0da3a391513c]::thread::Builder>::spawn_unchecked_<rustc_interface[a21a6190ce08c83a]::util::run_in_thread_pool_with_globals<rustc_interface[a21a6190ce08c83a]::interface::run_compiler<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}>::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>::{closure#1} as core[763cfd3286f44cff]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+  25: std::sys::PLATFORM::thread::Thread::new::thread_start::h158a9891bfff6346
+  26: <unknown>
+  27: <unknown>
+  28: <unknown>
+error: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+
---
+


full stderr:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_const_eval/src/interpret/memory.rs:236:74
stack backtrace:
   0:     0x7fa6d6b9eac5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h857c485298a35cfc
   1:     0x7fa6d6c0b0f8 - core::fmt::write::h1692ea36034ef62d
   2:     0x7fa6d6b93101 - std::io::Write::write_fmt::h5275431ba0b03075
   3:     0x7fa6d6b9e8d5 - std::sys_common::backtrace::print::hbaebd0b6a97b2bdc
   4:     0x7fa6d6ba1a84 - std::panicking::default_hook::{{closure}}::h62c36ca7853dc291
   5:     0x7fa6d6ba1772 - std::panicking::default_hook::h63b797dbd5303937
   6:     0x7fa6d761a525 - rustc_driver_impl[30900129d6928322]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa6d6ba21b9 - std::panicking::rust_panic_with_hook::h34c6253620675739
   8:     0x7fa6d6ba1ef2 - std::panicking::begin_panic_handler::{{closure}}::h9e41015fba81e59e
   9:     0x7fa6d6b9ef76 - std::sys_common::backtrace::__rust_end_short_backtrace::h8f739472cbbe64ed
  10:     0x7fa6d6ba1c02 - rust_begin_unwind
  11:     0x7fa6d6b5c0a3 - core::panicking::panic_fmt::hec968b555ef4d9c1
  12:     0x7fa6d6b5c13d - core::panicking::panic::h7639f9e87c66325a
  13:     0x5644e77a84a2 - <rustc_const_eval[15cf6ae4845e40ca]::interpret::eval_context::InterpCx<miri[bae29999596f13bc]::machine::MiriMachine>>::allocate_raw_ptr
  14:     0x5644e78752d1 - <rustc_const_eval[15cf6ae4845e40ca]::interpret::eval_context::InterpCx<miri[bae29999596f13bc]::machine::MiriMachine> as miri[bae29999596f13bc]::shims::env::EvalContextExt>::update_environ
  15:     0x5644e78f3a39 - <miri[bae29999596f13bc]::machine::MiriMachine>::late_init
  16:     0x5644e7723324 - miri[bae29999596f13bc]::eval::create_ecx
  17:     0x5644e7727191 - miri[bae29999596f13bc]::eval::eval_entry
  18:     0x5644e767d195 - <rustc_middle[e73d1a046df07a29]::ty::context::GlobalCtxt>::enter::<<miri[b413ac633ed0375a]::MiriCompilerCalls as rustc_driver_impl[30900129d6928322]::Callbacks>::after_analysis::{closure#0}, ()>
  19:     0x5644e7679229 - <miri[b413ac633ed0375a]::MiriCompilerCalls as rustc_driver_impl[30900129d6928322]::Callbacks>::after_analysis
  20:     0x7fa6d766ce93 - <rustc_interface[a21a6190ce08c83a]::interface::Compiler>::enter::<rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}::{closure#2}, core[763cfd3286f44cff]::result::Result<core[763cfd3286f44cff]::option::Option<rustc_interface[a21a6190ce08c83a]::queries::Linker>, rustc_span[cdbb012465937b99]::ErrorGuaranteed>>
  21:     0x7fa6d761b830 - rustc_span[cdbb012465937b99]::with_source_map::<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_interface[a21a6190ce08c83a]::interface::run_compiler<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  22:     0x7fa6d765d498 - <scoped_tls[8c44f6ff39b30028]::ScopedKey<rustc_span[cdbb012465937b99]::SessionGlobals>>::set::<rustc_interface[a21a6190ce08c83a]::interface::run_compiler<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}>::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>
  23:     0x7fa6d7638970 - std[9bff0da3a391513c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a21a6190ce08c83a]::util::run_in_thread_pool_with_globals<rustc_interface[a21a6190ce08c83a]::interface::run_compiler<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}>::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>
  24:     0x7fa6d762c663 - <<std[9bff0da3a391513c]::thread::Builder>::spawn_unchecked_<rustc_interface[a21a6190ce08c83a]::util::run_in_thread_pool_with_globals<rustc_interface[a21a6190ce08c83a]::interface::run_compiler<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}>::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>::{closure#1} as core[763cfd3286f44cff]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7fa6d6bae42e - std::sys::unix::thread::Thread::new::thread_start::h158a9891bfff6346
  26:     0x7fa6d6843b43 - <unknown>
  27:     0x7fa6d68d5a00 - <unknown>
  28:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

---

actual output differed from expected
--- tests/pass/no_std.stderr
+++ <stderr output>
+thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_const_eval/src/interpret/memory.rs:LL:CC
+stack backtrace:
+   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h857c485298a35cfc
+   1: core::fmt::write::h1692ea36034ef62d
+   2: std::io::Write::write_fmt::h5275431ba0b03075
+   3: std::sys_common::backtrace::print::hbaebd0b6a97b2bdc
+   4: std::panicking::default_hook::{{closure}}::h62c36ca7853dc291
+   5: std::panicking::default_hook::h63b797dbd5303937
+   6: rustc_driver_impl[30900129d6928322]::DEFAULT_HOOK::{closure#0}::{closure#0}
+   7: std::panicking::rust_panic_with_hook::h34c6253620675739
+   8: std::panicking::begin_panic_handler::{{closure}}::h9e41015fba81e59e
+   9: std::sys_common::backtrace::__rust_end_short_backtrace::h8f739472cbbe64ed
+  11: core::panicking::panic_fmt::hec968b555ef4d9c1
+  11: core::panicking::panic_fmt::hec968b555ef4d9c1
+  12: core::panicking::panic::h7639f9e87c66325a
+  13: <rustc_const_eval[15cf6ae4845e40ca]::interpret::eval_context::InterpCx<miri[bae29999596f13bc]::machine::MiriMachine>>::allocate_raw_ptr
+  14: <rustc_const_eval[15cf6ae4845e40ca]::interpret::eval_context::InterpCx<miri[bae29999596f13bc]::machine::MiriMachine> as miri[bae29999596f13bc]::shims::env::EvalContextExt>::update_environ
+  15: <miri[bae29999596f13bc]::machine::MiriMachine>::late_init
+  16: miri[bae29999596f13bc]::eval::create_ecx
+  17: miri[bae29999596f13bc]::eval::eval_entry
+  18: <rustc_middle[e73d1a046df07a29]::ty::context::GlobalCtxt>::enter
+  19: <miri[b413ac633ed0375a]::MiriCompilerCalls as rustc_driver_impl[30900129d6928322]::Callbacks>::after_analysis
+  20: <rustc_interface[a21a6190ce08c83a]::interface::Compiler>::enter
+  21: rustc_span[cdbb012465937b99]::with_source_map
+  22: <scoped_tls[8c44f6ff39b30028]::ScopedKey<rustc_span[cdbb012465937b99]::SessionGlobals>>::set
+  23: std[9bff0da3a391513c]::sys_common::backtrace::__rust_begin_short_backtrace
+  24: <<std[9bff0da3a391513c]::thread::Builder>::spawn_unchecked_<rustc_interface[a21a6190ce08c83a]::util::run_in_thread_pool_with_globals<rustc_interface[a21a6190ce08c83a]::interface::run_compiler<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}>::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>::{closure#1} as core[763cfd3286f44cff]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+  25: std::sys::PLATFORM::thread::Thread::new::thread_start::h158a9891bfff6346
+  26: <unknown>
+  27: <unknown>
+  28: <unknown>
+error: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+
---
-hello, world!


full stderr:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_const_eval/src/interpret/memory.rs:236:74
stack backtrace:
   0:     0x7fa8327c8ac5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h857c485298a35cfc
   1:     0x7fa8328350f8 - core::fmt::write::h1692ea36034ef62d
   2:     0x7fa8327bd101 - std::io::Write::write_fmt::h5275431ba0b03075
   3:     0x7fa8327c88d5 - std::sys_common::backtrace::print::hbaebd0b6a97b2bdc
   4:     0x7fa8327cba84 - std::panicking::default_hook::{{closure}}::h62c36ca7853dc291
   5:     0x7fa8327cb772 - std::panicking::default_hook::h63b797dbd5303937
   6:     0x7fa833244525 - rustc_driver_impl[30900129d6928322]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa8327cc1b9 - std::panicking::rust_panic_with_hook::h34c6253620675739
   8:     0x7fa8327cbef2 - std::panicking::begin_panic_handler::{{closure}}::h9e41015fba81e59e
   9:     0x7fa8327c8f76 - std::sys_common::backtrace::__rust_end_short_backtrace::h8f739472cbbe64ed
  10:     0x7fa8327cbc02 - rust_begin_unwind
  11:     0x7fa8327860a3 - core::panicking::panic_fmt::hec968b555ef4d9c1
  12:     0x7fa83278613d - core::panicking::panic::h7639f9e87c66325a
  13:     0x55ff4abd24a2 - <rustc_const_eval[15cf6ae4845e40ca]::interpret::eval_context::InterpCx<miri[bae29999596f13bc]::machine::MiriMachine>>::allocate_raw_ptr
  14:     0x55ff4ac9f2d1 - <rustc_const_eval[15cf6ae4845e40ca]::interpret::eval_context::InterpCx<miri[bae29999596f13bc]::machine::MiriMachine> as miri[bae29999596f13bc]::shims::env::EvalContextExt>::update_environ
  15:     0x55ff4ad1da39 - <miri[bae29999596f13bc]::machine::MiriMachine>::late_init
  16:     0x55ff4ab4d324 - miri[bae29999596f13bc]::eval::create_ecx
  17:     0x55ff4ab51191 - miri[bae29999596f13bc]::eval::eval_entry
  18:     0x55ff4aaa7195 - <rustc_middle[e73d1a046df07a29]::ty::context::GlobalCtxt>::enter::<<miri[b413ac633ed0375a]::MiriCompilerCalls as rustc_driver_impl[30900129d6928322]::Callbacks>::after_analysis::{closure#0}, ()>
  19:     0x55ff4aaa3229 - <miri[b413ac633ed0375a]::MiriCompilerCalls as rustc_driver_impl[30900129d6928322]::Callbacks>::after_analysis
  20:     0x7fa833296e93 - <rustc_interface[a21a6190ce08c83a]::interface::Compiler>::enter::<rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}::{closure#2}, core[763cfd3286f44cff]::result::Result<core[763cfd3286f44cff]::option::Option<rustc_interface[a21a6190ce08c83a]::queries::Linker>, rustc_span[cdbb012465937b99]::ErrorGuaranteed>>
  21:     0x7fa833245830 - rustc_span[cdbb012465937b99]::with_source_map::<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_interface[a21a6190ce08c83a]::interface::run_compiler<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  22:     0x7fa833287498 - <scoped_tls[8c44f6ff39b30028]::ScopedKey<rustc_span[cdbb012465937b99]::SessionGlobals>>::set::<rustc_interface[a21a6190ce08c83a]::interface::run_compiler<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}>::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>
  23:     0x7fa833262970 - std[9bff0da3a391513c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a21a6190ce08c83a]::util::run_in_thread_pool_with_globals<rustc_interface[a21a6190ce08c83a]::interface::run_compiler<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}>::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>
  24:     0x7fa833256663 - <<std[9bff0da3a391513c]::thread::Builder>::spawn_unchecked_<rustc_interface[a21a6190ce08c83a]::util::run_in_thread_pool_with_globals<rustc_interface[a21a6190ce08c83a]::interface::run_compiler<core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>, rustc_driver_impl[30900129d6928322]::run_compiler::{closure#1}>::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[763cfd3286f44cff]::result::Result<(), rustc_span[cdbb012465937b99]::ErrorGuaranteed>>::{closure#1} as core[763cfd3286f44cff]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7fa8327d842e - std::sys::unix::thread::Thread::new::thread_start::h158a9891bfff6346
  26:     0x7fa83246db43 - <unknown>
  27:     0x7fa8324ffa00 - <unknown>
  28:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

