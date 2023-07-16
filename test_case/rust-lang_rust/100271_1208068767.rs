plain
running 55 tests
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
failures:

thread 'main' panicked at 'I/O failure during tests: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }', src/tools/compiletest/src/main.rs:430:13
---- [rustdoc-json] src/test/rustdoc-json/fn_pointer/generics.rs stdout ----
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: rustdoc failed!
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fn_pointer/generics/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fn_pointer/generics" "--deny" "warnings" "/checkout/src/test/rustdoc-json/fn_pointer/generics.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
   0:     0x7f642583df9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7f64258a49f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7f642582e651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   2:     0x7f642582e651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7f6425840f5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7f6425840c1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7f64261fa044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6425841712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7f64258414f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7f642583e514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7f6425841202 - rust_begin_unwind
  10:     0x7f64257f0e13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7f64257f0cdd - core::panicking::panic::h6647217039e322be
  12:     0x56545ed068a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x56545edf5707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x56545ed1c96e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x56545ee2c378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x56545edc0672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x56545eca6d53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x56545ef2da1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x56545ed1f441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x56545ed2a769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x56545ee3a609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x56545ef872ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x56545ed939f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f642584c9e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7f6425747609 - start_thread
  26:     0x7f642551b133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/generic-associated-types/gats.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/generic-associated-types/gats/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/generic-associated-types/gats" "--deny" "warnings" "/checkout/src/test/rustdoc-json/generic-associated-types/gats.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
stack backtrace:
   0:     0x7f2d0d768f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7f2d0d7cf9f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7f2d0d759651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7f2d0d76bf5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7f2d0d76bc1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7f2d0e125044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2d0d76c712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7f2d0d76c4f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7f2d0d769514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7f2d0d76c202 - rust_begin_unwind
  10:     0x7f2d0d71be13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7f2d0d71bcdd - core::panicking::panic::h6647217039e322be
  12:     0x562b086e98a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x562b087d8707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x562b086ff96e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x562b0880f378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x562b087a3672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x562b08689d53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x562b08910a1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x562b08702441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x562b0870d769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x562b0881d609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x562b0896a2ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x562b087769f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f2d0d7779e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7f2d0d672609 - start_thread
  26:     0x7f2d0d446133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/fns/generic_args.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_args/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_args" "--deny" "warnings" "/checkout/src/test/rustdoc-json/fns/generic_args.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
   0:     0x7f92630daf9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7f92631419f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7f92630cb651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   2:     0x7f92630cb651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7f92630ddf5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7f92630ddc1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7f9263a97044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f92630de712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7f92630de4f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7f92630db514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7f92630de202 - rust_begin_unwind
  10:     0x7f926308de13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7f926308dcdd - core::panicking::panic::h6647217039e322be
  12:     0x55af6e2688a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x55af6e357707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x55af6e27e96e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x55af6e38e378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x55af6e322672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x55af6e208d53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x55af6e48fa1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x55af6e281441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x55af6e28c769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x55af6e39c609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x55af6e4e92ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x55af6e2f59f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f92630e99e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7f9262fe4609 - start_thread
  26:     0x7f9262db8133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/doc_hidden_failure.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/doc_hidden_failure/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/doc_hidden_failure" "--deny" "warnings" "/checkout/src/test/rustdoc-json/doc_hidden_failure.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
stack backtrace:
   0:     0x7f034a278f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7f034a2df9f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7f034a269651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7f034a27bf5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7f034a27bc1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7f034ac35044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f034a27c712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7f034a27c4f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7f034a279514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7f034a27c202 - rust_begin_unwind
  10:     0x7f034a22be13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7f034a22bcdd - core::panicking::panic::h6647217039e322be
  12:     0x5556fa3c88a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x5556fa4b7707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x5556fa3de96e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x5556fa4ee378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x5556fa482672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x5556fa368d53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x5556fa5efa1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x5556fa3e1441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x5556fa3ec769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x5556fa4fc609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x5556fa6492ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x5556fa4559f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f034a2879e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7f034a182609 - start_thread
  26:     0x7f0349f56133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/fns/generics.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generics/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generics" "--deny" "warnings" "/checkout/src/test/rustdoc-json/fns/generics.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
   0:     0x7fae15fd1f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7fae160389f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7fae15fc2651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   2:     0x7fae15fc2651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7fae15fd4f5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7fae15fd4c1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7fae1698e044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fae15fd5712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7fae15fd54f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7fae15fd2514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7fae15fd5202 - rust_begin_unwind
  10:     0x7fae15f84e13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7fae15f84cdd - core::panicking::panic::h6647217039e322be
  12:     0x555ef18e78a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x555ef19d6707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x555ef18fd96e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x555ef1a0d378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x555ef19a1672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x555ef1887d53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x555ef1b0ea1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x555ef1900441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x555ef190b769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x555ef1a1b609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x555ef1b682ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x555ef19749f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7fae15fe09e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7fae15edb609 - start_thread
  26:     0x7fae15caf133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/fns/generic_returns.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_returns/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/generic_returns" "--deny" "warnings" "/checkout/src/test/rustdoc-json/fns/generic_returns.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
   0:     0x7f4f56f26f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7f4f56f8d9f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7f4f56f17651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   2:     0x7f4f56f17651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7f4f56f29f5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7f4f56f29c1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7f4f578e3044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4f56f2a712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7f4f56f2a4f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7f4f56f27514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7f4f56f2a202 - rust_begin_unwind
  10:     0x7f4f56ed9e13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7f4f56ed9cdd - core::panicking::panic::h6647217039e322be
  12:     0x55d2f91438a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x55d2f9232707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x55d2f915996e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x55d2f9269378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x55d2f91fd672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x55d2f90e3d53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x55d2f936aa1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x55d2f915c441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x55d2f9167769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x55d2f9277609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x55d2f93c42ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x55d2f91d09f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f4f56f359e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7f4f56e30609 - start_thread
  26:     0x7f4f56c04133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/glob_import.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/glob_import/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/glob_import" "--deny" "warnings" "/checkout/src/test/rustdoc-json/glob_import.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
   0:     0x7f926968df9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7f92696f49f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7f926967e651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   2:     0x7f926967e651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7f9269690f5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7f9269690c1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7f926a04a044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f9269691712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7f92696914f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7f926968e514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7f9269691202 - rust_begin_unwind
  10:     0x7f9269640e13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7f9269640cdd - core::panicking::panic::h6647217039e322be
  12:     0x557b7fcad8a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x557b7fd9c707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x557b7fcc396e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x557b7fdd3378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x557b7fd67672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x557b7fc4dd53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x557b7fed4a1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x557b7fcc6441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x557b7fcd1769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x557b7fde1609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x557b7ff2e2ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x557b7fd3a9f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f926969c9e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7f9269597609 - start_thread
  26:     0x7f926936b133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/lifetime/outlives.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/lifetime/outlives/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/lifetime/outlives" "--deny" "warnings" "/checkout/src/test/rustdoc-json/lifetime/outlives.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
stack backtrace:
   0:     0x7fb029701f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7fb0297689f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7fb0296f2651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7fb029704f5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7fb029704c1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7fb02a0be044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb029705712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7fb0297054f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7fb029702514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7fb029705202 - rust_begin_unwind
  10:     0x7fb0296b4e13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7fb0296b4cdd - core::panicking::panic::h6647217039e322be
  12:     0x559a159798a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x559a15a68707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x559a1598f96e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x559a15a9f378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x559a15a33672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x559a15919d53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x559a15ba0a1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x559a15992441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x559a1599d769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x559a15aad609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x559a15bfa2ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x559a15a069f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7fb0297109e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7fb02960b609 - start_thread
  26:     0x7fb0293df133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/lifetime/longest.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/lifetime/longest/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/lifetime/longest" "--deny" "warnings" "/checkout/src/test/rustdoc-json/lifetime/longest.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
stack backtrace:
   0:     0x7ffbef726f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7ffbef78d9f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7ffbef717651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7ffbef729f5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7ffbef729c1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7ffbf00e3044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ffbef72a712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7ffbef72a4f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7ffbef727514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7ffbef72a202 - rust_begin_unwind
  10:     0x7ffbef6d9e13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7ffbef6d9cdd - core::panicking::panic::h6647217039e322be
  12:     0x558fb142a8a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x558fb1519707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x558fb144096e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x558fb1550378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x558fb14e4672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x558fb13cad53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x558fb1651a1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x558fb1443441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x558fb144e769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x558fb155e609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x558fb16ab2ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x558fb14b79f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7ffbef7359e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7ffbef630609 - start_thread
  26:     0x7ffbef404133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/methods/abi.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/methods/abi/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/methods/abi" "--deny" "warnings" "/checkout/src/test/rustdoc-json/methods/abi.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
stack backtrace:
   0:     0x7f231a433f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7f231a49a9f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7f231a424651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7f231a436f5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7f231a436c1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7f231adf0044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f231a437712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7f231a4374f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7f231a434514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7f231a437202 - rust_begin_unwind
  10:     0x7f231a3e6e13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7f231a3e6cdd - core::panicking::panic::h6647217039e322be
  12:     0x5648ecc3b8a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x5648ecd2a707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x5648ecc5196e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x5648ecd61378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x5648eccf5672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x5648ecbdbd53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x5648ece62a1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x5648ecc54441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x5648ecc5f769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x5648ecd6f609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x5648ecebc2ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x5648eccc89f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f231a4429e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7f231a33d609 - start_thread
  26:     0x7f231a111133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/assoc_items.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/assoc_items/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/assoc_items" "--deny" "warnings" "/checkout/src/test/rustdoc-json/assoc_items.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
stack backtrace:
   0:     0x7f36b9458f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7f36b94bf9f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7f36b9449651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7f36b945bf5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7f36b945bc1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7f36b9e15044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f36b945c712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7f36b945c4f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7f36b9459514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7f36b945c202 - rust_begin_unwind
  10:     0x7f36b940be13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7f36b940bcdd - core::panicking::panic::h6647217039e322be
  12:     0x560f5f5c38a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x560f5f6b2707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x560f5f5d996e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x560f5f6e9378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x560f5f67d672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x560f5f563d53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x560f5f7eaa1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x560f5f5dc441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x560f5f5e7769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x560f5f6f7609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  22:     0x560f5f8442ee - std[35caa551f2bd943b]::panicking::try::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, core[11457ffc0d08deb2]::panic::unwind_safe::AssertUnwindSafe<<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x560f5f6509f2 - <<std[35caa551f2bd943b]::thread::Builder>::spawn_unchecked_<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#1} as core[11457ffc0d08deb2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f36b94679e5 - std::sys::unix::thread::Thread::new::thread_start::hd79f73c4cf20808a
  25:     0x7f36b9362609 - start_thread
  26:     0x7f36b9136133 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc-json] src/test/rustdoc-json/blanket_impls.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/blanket_impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/blanket_impls" "--deny" "warnings" "/checkout/src/test/rustdoc-json/blanket_impls.rs" "--output-format" "json" "-Zunstable-options"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:311:16
   0:     0x7fc1a3848f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h60ee82f7913f042c
   1:     0x7fc1a38af9f8 - core::fmt::write::hea2bca18d4c7e8ed
   2:     0x7fc1a3839651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   2:     0x7fc1a3839651 - std::io::Write::write_fmt::h8dac55ee3be476d7
   3:     0x7fc1a384bf5e - std::panicking::default_hook::{{closure}}::h6b633c4c8fca9f4f
   4:     0x7fc1a384bc1f - std::panicking::default_hook::h87db6d77693ccfe9
   5:     0x7fc1a4205044 - rustc_driver[8bafddba79f7d41c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc1a384c712 - std::panicking::rust_panic_with_hook::h685f84a1718c5070
   7:     0x7fc1a384c4f9 - std::panicking::begin_panic_handler::{{closure}}::hb39cb4dcc38b9fc9
   8:     0x7fc1a3849514 - std::sys_common::backtrace::__rust_end_short_backtrace::h5bbbcc259388a57b
   9:     0x7fc1a384c202 - rust_begin_unwind
  10:     0x7fc1a37fbe13 - core::panicking::panic_fmt::hd4d2c8cf7ca7398f
  11:     0x7fc1a37fbcdd - core::panicking::panic::h6647217039e322be
  12:     0x55ea15f8d8a5 - <rustdoc[5e348fabbf908524]::json::JsonRenderer as rustdoc[5e348fabbf908524]::formats::renderer::FormatRenderer>::after_krate
  13:     0x55ea1607c707 - rustdoc[5e348fabbf908524]::formats::renderer::run_format::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  14:     0x55ea15fa396e - rustdoc[5e348fabbf908524]::run_renderer::<rustdoc[5e348fabbf908524]::json::JsonRenderer>
  15:     0x55ea160b3378 - <rustc_session[90272e15b8c85301]::session::Session>::time::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#2}>
  16:     0x55ea16047672 - <rustc_interface[ed8c02c5e442a77]::passes::QueryContext>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}::{closure#1}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  17:     0x55ea15f2dd53 - <rustc_interface[ed8c02c5e442a77]::interface::Compiler>::enter::<rustdoc[5e348fabbf908524]::main_options::{closure#0}::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  18:     0x55ea161b4a1b - rustc_span[65855187f00bea3d]::with_source_map::<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustc_interface[ed8c02c5e442a77]::interface::create_compiler_and_run<core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>, rustdoc[5e348fabbf908524]::main_options::{closure#0}>::{closure#1}>
  19:     0x55ea15fa6441 - rustdoc[5e348fabbf908524]::main_options
  20:     0x55ea15fb1769 - <scoped_tls[c5b47b3bf6710703]::ScopedKey<rustc_span[65855187f00bea3d]::SessionGlobals>>::set::<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
  21:     0x55ea160c1609 - std[35caa551f2bd943b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ed8c02c5e442a77]::util::run_in_thread_pool_with_globals<rustdoc[5e348fabbf908524]::main_args::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>::{closure#0}, core[11457ffc0d08deb2]::result::Result<(), rustc_errors[a5a14b99e1b00533]::ErrorGuaranteed>>
Build completed unsuccessfully in 0:32:06
