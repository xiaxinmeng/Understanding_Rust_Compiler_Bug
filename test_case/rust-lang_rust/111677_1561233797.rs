plain
tests/fail/tokio/sleep.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/invalid_bool.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "tests/fail/invalid_bool.rs" "-Zmiri-disable-alignment-check" "-Zmiri-disable-stacked-borrows" "-Zmiri-disable-validation" "--edition" "2021"


fail test got exit status: 101, but expected 1


A bug in `ui_test` occurred: test panicked: stderr:
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: failed while formatting fluent string `const_eval_invalid_bool`: 
the fluent string has an argument `val` that was not found.
help: the argument `value` is available
stack backtrace:
   0:     0x7f7ad3b7bdc1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha33519be807b9e90
   1:     0x7f7ad3be1638 - core::fmt::write::hcfc1b5d3630d46bc
   2:     0x7f7ad3b70541 - std::io::Write::write_fmt::he7e802acde0956aa
   2:     0x7f7ad3b70541 - std::io::Write::write_fmt::he7e802acde0956aa
   3:     0x7f7ad3b7bbd5 - std::sys_common::backtrace::print::ha21e12769b20174d
   4:     0x7f7ad3b7ec2a - std::panicking::default_hook::{{closure}}::h26455d42676e6f25
   5:     0x7f7ad3b7e990 - std::panicking::default_hook::h8f02a0b1db0e167d
   6:     0x7f7ad45b4e0b - rustc_driver_impl[535bac85d4800bad]::install_ice_hook::{closure#0}
   7:     0x7f7ad3b7f302 - std::panicking::rust_panic_with_hook::h82e1d3a9af9af973
   8:     0x7f7ad3b7f087 - std::panicking::begin_panic_handler::{{closure}}::h02e34929e337d44b
   9:     0x7f7ad3b7c276 - std::sys_common::backtrace::__rust_end_short_backtrace::h3efbd8a9c363a9bb
  10:     0x7f7ad3b7edb2 - rust_begin_unwind
  11:     0x7f7ad3b38c13 - core::panicking::panic_fmt::h9c09764f6d49b506
  12:     0x7f7ad3b391f3 - core::result::unwrap_failed::ha56d85afefb220bb
  13:     0x55d11866386d - <rustc_errors[4c78f55d9cc5f46]::Handler>::eagerly_translate_to_string::<std[ebb9df25de86cedd]::collections::hash::map::Iter<alloc[32a38855e109692a]::borrow::Cow<str>, rustc_errors[4c78f55d9cc5f46]::diagnostic::DiagnosticArgValue>>
  14:     0x55d118674e21 - miri[71447080387b09f8]::diagnostics::report_error
  15:     0x55d118498fae - miri[71447080387b09f8]::eval::eval_entry
  16:     0x55d1183e5670 - <rustc_interface[a290b752cfd746b9]::queries::QueryResult<&rustc_middle[f8e0cb549d93f4d2]::ty::context::GlobalCtxt>>::enter::<(), <miri[f52b30c07fedc22a]::MiriCompilerCalls as rustc_driver_impl[535bac85d4800bad]::Callbacks>::after_analysis::{closure#0}>
  17:     0x55d1183eda3a - <miri[f52b30c07fedc22a]::MiriCompilerCalls as rustc_driver_impl[535bac85d4800bad]::Callbacks>::after_analysis
  18:     0x7f7ad4607ee5 - <rustc_interface[a290b752cfd746b9]::interface::Compiler>::enter::<rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}::{closure#2}, core[b3712eec32c24bf4]::result::Result<core[b3712eec32c24bf4]::option::Option<rustc_interface[a290b752cfd746b9]::queries::Linker>, rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>
  19:     0x7f7ad45c9fff - rustc_span[b7d86bfc037a248f]::set_source_map::<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  20:     0x7f7ad45c0025 - <scoped_tls[5a5d5be53a0448ef]::ScopedKey<rustc_span[b7d86bfc037a248f]::SessionGlobals>>::set::<rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>
  21:     0x7f7ad45d6679 - std[ebb9df25de86cedd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a290b752cfd746b9]::util::run_in_thread_pool_with_globals<rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>
  22:     0x7f7ad45bcd95 - <<std[ebb9df25de86cedd]::thread::Builder>::spawn_unchecked_<rustc_interface[a290b752cfd746b9]::util::run_in_thread_pool_with_globals<rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>::{closure#1} as core[b3712eec32c24bf4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  23:     0x7f7ad3b8ba4e - std::sys::unix::thread::Thread::new::thread_start::h95b4a232dcf07ef4
  24:     0x7f7ad3824b43 - <unknown>
  25:     0x7f7ad38b6a00 - <unknown>
  26:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/miri/issues/new


note: rustc 1.71.0-nightly (740a937fe 2023-05-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z ui-testing -Z miri-disable-alignment-check -Z miri-disable-stacked-borrows -Z miri-disable-validation
query stack during panic:
end of query stack

stdout:
stdout:


full stderr:



tests/fail/invalid_char.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "tests/fail/invalid_char.rs" "-Zmiri-disable-alignment-check" "-Zmiri-disable-stacked-borrows" "-Zmiri-disable-validation" "--edition" "2021"


fail test got exit status: 101, but expected 1


A bug in `ui_test` occurred: test panicked: stderr:
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: failed while formatting fluent string `const_eval_invalid_char`: 
the fluent string has an argument `val` that was not found.
help: the argument `value` is available
stack backtrace:
   0:     0x7ff2fff7edc1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha33519be807b9e90
   1:     0x7ff2fffe4638 - core::fmt::write::hcfc1b5d3630d46bc
   2:     0x7ff2fff73541 - std::io::Write::write_fmt::he7e802acde0956aa
   2:     0x7ff2fff73541 - std::io::Write::write_fmt::he7e802acde0956aa
   3:     0x7ff2fff7ebd5 - std::sys_common::backtrace::print::ha21e12769b20174d
   4:     0x7ff2fff81c2a - std::panicking::default_hook::{{closure}}::h26455d42676e6f25
   5:     0x7ff2fff81990 - std::panicking::default_hook::h8f02a0b1db0e167d
   6:     0x7ff3009b7e0b - rustc_driver_impl[535bac85d4800bad]::install_ice_hook::{closure#0}
   7:     0x7ff2fff82302 - std::panicking::rust_panic_with_hook::h82e1d3a9af9af973
   8:     0x7ff2fff82087 - std::panicking::begin_panic_handler::{{closure}}::h02e34929e337d44b
   9:     0x7ff2fff7f276 - std::sys_common::backtrace::__rust_end_short_backtrace::h3efbd8a9c363a9bb
  10:     0x7ff2fff81db2 - rust_begin_unwind
  11:     0x7ff2fff3bc13 - core::panicking::panic_fmt::h9c09764f6d49b506
  12:     0x7ff2fff3c1f3 - core::result::unwrap_failed::ha56d85afefb220bb
  13:     0x5578d531e86d - <rustc_errors[4c78f55d9cc5f46]::Handler>::eagerly_translate_to_string::<std[ebb9df25de86cedd]::collections::hash::map::Iter<alloc[32a38855e109692a]::borrow::Cow<str>, rustc_errors[4c78f55d9cc5f46]::diagnostic::DiagnosticArgValue>>
  14:     0x5578d532fe21 - miri[71447080387b09f8]::diagnostics::report_error
  15:     0x5578d5153fae - miri[71447080387b09f8]::eval::eval_entry
  16:     0x5578d50a0670 - <rustc_interface[a290b752cfd746b9]::queries::QueryResult<&rustc_middle[f8e0cb549d93f4d2]::ty::context::GlobalCtxt>>::enter::<(), <miri[f52b30c07fedc22a]::MiriCompilerCalls as rustc_driver_impl[535bac85d4800bad]::Callbacks>::after_analysis::{closure#0}>
  17:     0x5578d50a8a3a - <miri[f52b30c07fedc22a]::MiriCompilerCalls as rustc_driver_impl[535bac85d4800bad]::Callbacks>::after_analysis
  18:     0x7ff300a0aee5 - <rustc_interface[a290b752cfd746b9]::interface::Compiler>::enter::<rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}::{closure#2}, core[b3712eec32c24bf4]::result::Result<core[b3712eec32c24bf4]::option::Option<rustc_interface[a290b752cfd746b9]::queries::Linker>, rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>
  19:     0x7ff3009ccfff - rustc_span[b7d86bfc037a248f]::set_source_map::<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  20:     0x7ff3009c3025 - <scoped_tls[5a5d5be53a0448ef]::ScopedKey<rustc_span[b7d86bfc037a248f]::SessionGlobals>>::set::<rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>
  21:     0x7ff3009d9679 - std[ebb9df25de86cedd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a290b752cfd746b9]::util::run_in_thread_pool_with_globals<rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>
  22:     0x7ff3009bfd95 - <<std[ebb9df25de86cedd]::thread::Builder>::spawn_unchecked_<rustc_interface[a290b752cfd746b9]::util::run_in_thread_pool_with_globals<rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>::{closure#1} as core[b3712eec32c24bf4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  23:     0x7ff2fff8ea4e - std::sys::unix::thread::Thread::new::thread_start::h95b4a232dcf07ef4
  24:     0x7ff2ffc27b43 - <unknown>
  25:     0x7ff2ffcb9a00 - <unknown>
  26:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/miri/issues/new


note: rustc 1.71.0-nightly (740a937fe 2023-05-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z ui-testing -Z miri-disable-alignment-check -Z miri-disable-stacked-borrows -Z miri-disable-validation
query stack during panic:
end of query stack

stdout:
stdout:


full stderr:



tests/fail/invalid_enum_tag.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "tests/fail/invalid_enum_tag.rs" "-Zmiri-disable-alignment-check" "-Zmiri-disable-stacked-borrows" "-Zmiri-disable-validation" "--edition" "2021"


fail test got exit status: 101, but expected 1


A bug in `ui_test` occurred: test panicked: stderr:
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: failed while formatting fluent string `const_eval_invalid_tag`: 
the fluent string has an argument `val` that was not found.
help: the argument `tag` is available
stack backtrace:
   0:     0x7f4e9a01fdc1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha33519be807b9e90
   1:     0x7f4e9a085638 - core::fmt::write::hcfc1b5d3630d46bc
   2:     0x7f4e9a014541 - std::io::Write::write_fmt::he7e802acde0956aa
   2:     0x7f4e9a014541 - std::io::Write::write_fmt::he7e802acde0956aa
   3:     0x7f4e9a01fbd5 - std::sys_common::backtrace::print::ha21e12769b20174d
   4:     0x7f4e9a022c2a - std::panicking::default_hook::{{closure}}::h26455d42676e6f25
   5:     0x7f4e9a022990 - std::panicking::default_hook::h8f02a0b1db0e167d
   6:     0x7f4e9aa58e0b - rustc_driver_impl[535bac85d4800bad]::install_ice_hook::{closure#0}
   7:     0x7f4e9a023302 - std::panicking::rust_panic_with_hook::h82e1d3a9af9af973
   8:     0x7f4e9a023087 - std::panicking::begin_panic_handler::{{closure}}::h02e34929e337d44b
   9:     0x7f4e9a020276 - std::sys_common::backtrace::__rust_end_short_backtrace::h3efbd8a9c363a9bb
  10:     0x7f4e9a022db2 - rust_begin_unwind
  11:     0x7f4e99fdcc13 - core::panicking::panic_fmt::h9c09764f6d49b506
  12:     0x7f4e99fdd1f3 - core::result::unwrap_failed::ha56d85afefb220bb
  13:     0x55c0de66586d - <rustc_errors[4c78f55d9cc5f46]::Handler>::eagerly_translate_to_string::<std[ebb9df25de86cedd]::collections::hash::map::Iter<alloc[32a38855e109692a]::borrow::Cow<str>, rustc_errors[4c78f55d9cc5f46]::diagnostic::DiagnosticArgValue>>
  14:     0x55c0de676e21 - miri[71447080387b09f8]::diagnostics::report_error
  15:     0x55c0de49afae - miri[71447080387b09f8]::eval::eval_entry
  16:     0x55c0de3e7670 - <rustc_interface[a290b752cfd746b9]::queries::QueryResult<&rustc_middle[f8e0cb549d93f4d2]::ty::context::GlobalCtxt>>::enter::<(), <miri[f52b30c07fedc22a]::MiriCompilerCalls as rustc_driver_impl[535bac85d4800bad]::Callbacks>::after_analysis::{closure#0}>
  17:     0x55c0de3efa3a - <miri[f52b30c07fedc22a]::MiriCompilerCalls as rustc_driver_impl[535bac85d4800bad]::Callbacks>::after_analysis
  18:     0x7f4e9aaabee5 - <rustc_interface[a290b752cfd746b9]::interface::Compiler>::enter::<rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}::{closure#2}, core[b3712eec32c24bf4]::result::Result<core[b3712eec32c24bf4]::option::Option<rustc_interface[a290b752cfd746b9]::queries::Linker>, rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>
  19:     0x7f4e9aa6dfff - rustc_span[b7d86bfc037a248f]::set_source_map::<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  20:     0x7f4e9aa64025 - <scoped_tls[5a5d5be53a0448ef]::ScopedKey<rustc_span[b7d86bfc037a248f]::SessionGlobals>>::set::<rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>
  21:     0x7f4e9aa7a679 - std[ebb9df25de86cedd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a290b752cfd746b9]::util::run_in_thread_pool_with_globals<rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>
  22:     0x7f4e9aa60d95 - <<std[ebb9df25de86cedd]::thread::Builder>::spawn_unchecked_<rustc_interface[a290b752cfd746b9]::util::run_in_thread_pool_with_globals<rustc_interface[a290b752cfd746b9]::interface::run_compiler<core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>, rustc_driver_impl[535bac85d4800bad]::run_compiler::{closure#1}>::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b3712eec32c24bf4]::result::Result<(), rustc_span[b7d86bfc037a248f]::ErrorGuaranteed>>::{closure#1} as core[b3712eec32c24bf4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  23:     0x7f4e9a02fa4e - std::sys::unix::thread::Thread::new::thread_start::h95b4a232dcf07ef4
  24:     0x7f4e99cc8b43 - <unknown>
  25:     0x7f4e99d5aa00 - <unknown>
  26:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/miri/issues/new


note: rustc 1.71.0-nightly (740a937fe 2023-05-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z ui-testing -Z miri-disable-alignment-check -Z miri-disable-stacked-borrows -Z miri-disable-validation
query stack during panic:
end of query stack

stdout:
stdout:


full stderr:



tests/fail/function_pointers/cast_fn_ptr3.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "tests/fail/function_pointers/cast_fn_ptr3.rs" "--edition" "2021"



actual output differed from expected
---
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...


substring `calling a function with fewer arguments than it requires` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/function_pointers/cast_fn_ptr3.rs:6
    Error: Undefined Behavior: calling a function with more arguments than it expected

---



tests/fail/validity/dangling_ref3.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rlib" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rlib" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rlib" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rlib" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rlib" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rlib" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "--extern" "miri_test_deps=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/miri-test-deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "tests/fail/validity/dangling_ref3.rs" "-Zmiri-disable-stacked-borrows" "--edition" "2021"



actual output differed from expected
actual output differed from expected
--- tests/fail/validity/dangling_ref3.stderr
+++ <stderr output>
-error: Undefined Behavior: constructing invalid value: encountered a dangling reference (use-after-free)
+error: Undefined Behavior: constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
   --> $DIR/dangling_ref3.rs:LL:CC
    |
 LL |     let _x: &i32 = unsafe { mem::transmute(dangling()) };
-   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (use-after-free)
+   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `dangling reference (use-after-free)` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/validity/dangling_ref3.rs:11
    Error: Undefined Behavior: constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)


full stderr:
error: Undefined Behavior: constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
  --> tests/fail/validity/dangling_ref3.rs:11:29
   |
LL |     let _x: &i32 = unsafe { mem::transmute(dangling()) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/validity/dangling_ref3.rs:11:29: 11:55
---
test result: FAIL. 5 tests failed, 377 tests passed, 3 ignored, 0 filtered out
Error: 
   0: tests failed

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-b7edc18be2be8615 --quiet` (exit status: 1)
Build completed unsuccessfully in 0:02:48
