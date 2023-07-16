plain
---- [rustdoc] rustdoc/comment-in-doctest.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/comment-in-doctest/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/comment-in-doctest" "--deny" "warnings" "/checkout/src/test/rustdoc/comment-in-doctest.rs" "--test"
running 1 test
test /checkout/src/test/rustdoc/comment-in-doctest.rs - (line 10) ... FAILED

failures:
failures:

---- /checkout/src/test/rustdoc/comment-in-doctest.rs - (line 10) stdout ----
thread '/checkout/src/test/rustdoc/comment-in-doctest.rs - (line 10)' panicked at 'explicit panic', compiler/rustc_errors/src/diagnostic_builder.rs:553:21
stack backtrace:
   0:     0x7f368ed1914c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb31841a35de7ae44
   1:     0x7f368ed7f50f - core::fmt::write::hdf0ec0802ad45e4b
   2:     0x7f368ed089f1 - std::io::Write::write_fmt::h2524cb9ed65713f9
   3:     0x7f368ed18f6b - std::sys_common::backtrace::print::hd98fe85c86c28bd8
   4:     0x7f368ed1c984 - std::panicking::default_hook::{{closure}}::hc73b9ffe54393b8e
   5:     0x7f368ed1c578 - std::panicking::default_hook::hb1253ab9ee6e621c
   6:     0x7f368f7ac0c1 - rustc_driver[8ecb87e1eb3aa8f3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f368ed1d06b - std::panicking::rust_panic_with_hook::h6757e7fd135fa48c
   8:     0x7f368ed1ce59 - std::panicking::begin_panic_handler::{{closure}}::h79d1d89385d63c41
   9:     0x7f368ed19664 - std::sys_common::backtrace::__rust_end_short_backtrace::h7fa9c914986083cb
  10:     0x7f368ed1cb79 - rust_begin_unwind
  11:     0x7f368eccbca3 - core::panicking::panic_fmt::h38a44dcfbb3a04c8
  12:     0x7f368eccbb6d - core::panicking::panic::hc3fe1be39c8433c6
  13:     0x7f36921db6ce - <rustc_errors[8887b8da7163ea9f]::diagnostic_builder::DiagnosticBuilderInner as core[68e790c1a639eca5]::ops::drop::Drop>::drop
  14:     0x559bcc5d4a5c - <scoped_tls[f351b9b0045ca033]::ScopedKey<rustc_span[3810166f75dd4edb]::SessionGlobals>>::with::<rustdoc[a8a656e4f4865ca9]::doctest::check_if_attr_is_complete::{closure#0}, bool>
  15:     0x559bcc5d2f10 - <scoped_tls[f351b9b0045ca033]::ScopedKey<rustc_span[3810166f75dd4edb]::SessionGlobals>>::set::<rustc_span[3810166f75dd4edb]::create_session_if_not_set_then<bool, rustdoc[a8a656e4f4865ca9]::doctest::check_if_attr_is_complete::{closure#0}>::{closure#0}, bool>
  16:     0x559bcc7e3d30 - rustdoc[a8a656e4f4865ca9]::doctest::make_test
  17:     0x559bcc7e084f - rustdoc[a8a656e4f4865ca9]::doctest::run_test::<<rustdoc[a8a656e4f4865ca9]::doctest::Collector as rustdoc[a8a656e4f4865ca9]::doctest::Tester>::add_test::{closure#2}::{closure#0}>
  18:     0x559bcc7c6ed3 - <<rustdoc[a8a656e4f4865ca9]::doctest::Collector as rustdoc[a8a656e4f4865ca9]::doctest::Tester>::add_test::{closure#2} as core[68e790c1a639eca5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7f36935ce7ae - test::__rust_begin_short_backtrace::hd7bd01bd55c086c0
  20:     0x7f36935ccf6f - test::run_test::run_test_inner::{{closure}}::h2446c47ff9f6bd85
  21:     0x7f3693587211 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1ef205fbd44f2d54
  22:     0x7f369358e318 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hae996b1e9457a7d2
  23:     0x7f368ed288e3 - std::sys::unix::thread::Thread::new::thread_start::hc88ec2c1f0dd94c0
  24:     0x7f368ec25609 - start_thread
  25:     0x7f368e9fb163 - clone
  26:                0x0 - <unknown>
query stack during panic:
end of query stack


