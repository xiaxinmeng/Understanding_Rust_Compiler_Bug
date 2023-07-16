plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 68 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..................................F.FF.FF..FFFF.FF.........FFF......

---- [ui] src/test/ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----

error: test compilation failed although it shouldn't!
error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-load.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "crate-attr=plugin(lint_plugin_test)" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> <crate attribute>:1:1
   |
LL | plugin(lint_plugin_test)
   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread 'rustc' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7fce2077897c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h21bd9c7cdc48d0a4
   1:     0x7fce207defd8 - core::fmt::write::hc460522e84e320df
   2:     0x7fce20768801 - std::io::Write::write_fmt::h3fa3fb7cbe034a52
   3:     0x7fce2077b9ce - std::panicking::default_hook::{{closure}}::h8acdb61d396c8dfb
   4:     0x7fce2077b5fc - std::panicking::default_hook::hf0275d4850e0aea6
   5:     0x5614233398e1 - rustc_driver[3f2533b0400fee3f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fce2077c22e - std::panicking::rust_panic_with_hook::h23a09bd700c5dc51
   7:     0x7fce12d343ab - std[15814acf61565875]::panicking::begin_panic::<&str>::{closure#0}
   8:     0x7fce12d33f24 - std[15814acf61565875]::sys_common::backtrace::__rust_end_short_backtrace::<std[15814acf61565875]::panicking::begin_panic<&str>::{closure#0}, !>
   9:     0x7fce0ff00f2c - std[15814acf61565875]::panicking::begin_panic::<&str>
  10:     0x7fce12d3dcc1 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::with::<<rustc_span[5d78bee91e5c8a17]::symbol::Symbol>::as_str::{closure#0}, &str>
  11:     0x7fce0ff0b01d - <lint_plugin_test::Pass as rustc_lint::passes::EarlyLintPass>::check_item::h0786d59c57c13ae5
  12:     0x5614255d0470 - <rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects as rustc_lint[462294dbdfc0c258]::passes::EarlyLintPass>::check_item
  13:     0x561423473847 - <rustc_lint[462294dbdfc0c258]::early::EarlyContextAndPass<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects> as rustc_ast[ca3c63eecea61c62]::visit::Visitor>::visit_item
  14:     0x5614234c79ef - rustc_ast[ca3c63eecea61c62]::visit::walk_crate::<rustc_lint[462294dbdfc0c258]::early::EarlyContextAndPass<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects>>
  15:     0x5614234719b0 - rustc_lint[462294dbdfc0c258]::early::early_lint_node::<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects, &rustc_ast[ca3c63eecea61c62]::ast::Crate>
  16:     0x56142347029d - rustc_lint[462294dbdfc0c258]::early::check_ast_node::<rustc_lint[462294dbdfc0c258]::BuiltinCombinedEarlyLintPass, &rustc_ast[ca3c63eecea61c62]::ast::Crate>
  17:     0x56142349da1a - <rustc_session[c42791251753e321]::session::Session>::time::<(), rustc_interface[a5b41241d2768600]::passes::configure_and_expand::{closure#8}>
  18:     0x56142347ea37 - rustc_interface[a5b41241d2768600]::passes::configure_and_expand
  19:     0x5614234aaeef - <rustc_interface[a5b41241d2768600]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[a5b41241d2768600]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[6397b3f443c99aab]::result::Result<rustc_ast[ca3c63eecea61c62]::ast::Crate, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  20:     0x56142346cab2 - <rustc_interface[a5b41241d2768600]::queries::Queries>::expansion
  21:     0x56142336348c - <rustc_interface[a5b41241d2768600]::interface::Compiler>::enter::<rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}::{closure#2}, core[6397b3f443c99aab]::result::Result<core[6397b3f443c99aab]::option::Option<rustc_interface[a5b41241d2768600]::queries::Linker>, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  22:     0x56142334752b - rustc_span[5d78bee91e5c8a17]::with_source_map::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_interface[a5b41241d2768600]::interface::create_compiler_and_run<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#1}>
  23:     0x561423364709 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::set::<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  24:     0x5614233bf789 - std[15814acf61565875]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  25:     0x561423365691 - std[15814acf61565875]::panicking::try::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  26:     0x5614233ba792 - <<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1} as core[6397b3f443c99aab]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7fce20787603 - std::sys::unix::thread::Thread::new::thread_start::h95090dc407d71b16
  28:     0x7fce1aecd609 - start_thread
  29:     0x7fce1aca1133 - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (2df5a508b 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z crate-attr=plugin(lint_plugin_test)
query stack during panic:
end of query stack
warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread 'rustc' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7fafcd1a497c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h21bd9c7cdc48d0a4
   1:     0x7fafcd20afd8 - core::fmt::write::hc460522e84e320df
   2:     0x7fafcd194801 - std::io::Write::write_fmt::h3fa3fb7cbe034a52
   3:     0x7fafcd1a79ce - std::panicking::default_hook::{{closure}}::h8acdb61d396c8dfb
   4:     0x7fafcd1a75fc - std::panicking::default_hook::hf0275d4850e0aea6
   5:     0x55d0f87278e1 - rustc_driver[3f2533b0400fee3f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fafcd1a822e - std::panicking::rust_panic_with_hook::h23a09bd700c5dc51
   7:     0x7fafbed343ab - std[15814acf61565875]::panicking::begin_panic::<&str>::{closure#0}
   8:     0x7fafbed33f24 - std[15814acf61565875]::sys_common::backtrace::__rust_end_short_backtrace::<std[15814acf61565875]::panicking::begin_panic<&str>::{closure#0}, !>
   9:     0x7fafbbf00f2c - std[15814acf61565875]::panicking::begin_panic::<&str>
  10:     0x7fafbed3dcc1 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::with::<<rustc_span[5d78bee91e5c8a17]::symbol::Symbol>::as_str::{closure#0}, &str>
  11:     0x7fafbbf0b01d - <lint_plugin_test::Pass as rustc_lint::passes::EarlyLintPass>::check_item::h0786d59c57c13ae5
  12:     0x55d0fa9be470 - <rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects as rustc_lint[462294dbdfc0c258]::passes::EarlyLintPass>::check_item
  13:     0x55d0f8861847 - <rustc_lint[462294dbdfc0c258]::early::EarlyContextAndPass<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects> as rustc_ast[ca3c63eecea61c62]::visit::Visitor>::visit_item
  14:     0x55d0f88b59ef - rustc_ast[ca3c63eecea61c62]::visit::walk_crate::<rustc_lint[462294dbdfc0c258]::early::EarlyContextAndPass<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects>>
  15:     0x55d0f885f9b0 - rustc_lint[462294dbdfc0c258]::early::early_lint_node::<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects, &rustc_ast[ca3c63eecea61c62]::ast::Crate>
  16:     0x55d0f885e29d - rustc_lint[462294dbdfc0c258]::early::check_ast_node::<rustc_lint[462294dbdfc0c258]::BuiltinCombinedEarlyLintPass, &rustc_ast[ca3c63eecea61c62]::ast::Crate>
  17:     0x55d0f888ba1a - <rustc_session[c42791251753e321]::session::Session>::time::<(), rustc_interface[a5b41241d2768600]::passes::configure_and_expand::{closure#8}>
  18:     0x55d0f886ca37 - rustc_interface[a5b41241d2768600]::passes::configure_and_expand
  19:     0x55d0f8898eef - <rustc_interface[a5b41241d2768600]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[a5b41241d2768600]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[6397b3f443c99aab]::result::Result<rustc_ast[ca3c63eecea61c62]::ast::Crate, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  20:     0x55d0f885aab2 - <rustc_interface[a5b41241d2768600]::queries::Queries>::expansion
  21:     0x55d0f875148c - <rustc_interface[a5b41241d2768600]::interface::Compiler>::enter::<rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}::{closure#2}, core[6397b3f443c99aab]::result::Result<core[6397b3f443c99aab]::option::Option<rustc_interface[a5b41241d2768600]::queries::Linker>, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  22:     0x55d0f873552b - rustc_span[5d78bee91e5c8a17]::with_source_map::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_interface[a5b41241d2768600]::interface::create_compiler_and_run<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#1}>
  23:     0x55d0f8752709 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::set::<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  24:     0x55d0f87ad789 - std[15814acf61565875]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  25:     0x55d0f8753691 - std[15814acf61565875]::panicking::try::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  26:     0x55d0f87a8792 - <<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1} as core[6397b3f443c99aab]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7fafcd1b3603 - std::sys::unix::thread::Thread::new::thread_start::h95090dc407d71b16
  28:     0x7fafc78f9609 - start_thread
  29:     0x7fafc76cd133 - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (2df5a508b 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread 'rustc' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7fc898e4a97c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h21bd9c7cdc48d0a4
   1:     0x7fc898eb0fd8 - core::fmt::write::hc460522e84e320df
   2:     0x7fc898e3a801 - std::io::Write::write_fmt::h3fa3fb7cbe034a52
   3:     0x7fc898e4d9ce - std::panicking::default_hook::{{closure}}::h8acdb61d396c8dfb
   4:     0x7fc898e4d5fc - std::panicking::default_hook::hf0275d4850e0aea6
   5:     0x55f9f36428e1 - rustc_driver[3f2533b0400fee3f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc898e4e22e - std::panicking::rust_panic_with_hook::h23a09bd700c5dc51
   7:     0x7fc88ad343ab - std[15814acf61565875]::panicking::begin_panic::<&str>::{closure#0}
   8:     0x7fc88ad33f24 - std[15814acf61565875]::sys_common::backtrace::__rust_end_short_backtrace::<std[15814acf61565875]::panicking::begin_panic<&str>::{closure#0}, !>
   9:     0x7fc887f00f2c - std[15814acf61565875]::panicking::begin_panic::<&str>
  10:     0x7fc88ad3dcc1 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::with::<<rustc_span[5d78bee91e5c8a17]::symbol::Symbol>::as_str::{closure#0}, &str>
  11:     0x7fc887f0b01d - <lint_plugin_test::Pass as rustc_lint::passes::EarlyLintPass>::check_item::h0786d59c57c13ae5
  12:     0x55f9f58d9470 - <rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects as rustc_lint[462294dbdfc0c258]::passes::EarlyLintPass>::check_item
  13:     0x55f9f377c847 - <rustc_lint[462294dbdfc0c258]::early::EarlyContextAndPass<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects> as rustc_ast[ca3c63eecea61c62]::visit::Visitor>::visit_item
  14:     0x55f9f37d09ef - rustc_ast[ca3c63eecea61c62]::visit::walk_crate::<rustc_lint[462294dbdfc0c258]::early::EarlyContextAndPass<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects>>
  15:     0x55f9f377a9b0 - rustc_lint[462294dbdfc0c258]::early::early_lint_node::<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects, &rustc_ast[ca3c63eecea61c62]::ast::Crate>
  16:     0x55f9f377929d - rustc_lint[462294dbdfc0c258]::early::check_ast_node::<rustc_lint[462294dbdfc0c258]::BuiltinCombinedEarlyLintPass, &rustc_ast[ca3c63eecea61c62]::ast::Crate>
  17:     0x55f9f37a6a1a - <rustc_session[c42791251753e321]::session::Session>::time::<(), rustc_interface[a5b41241d2768600]::passes::configure_and_expand::{closure#8}>
  18:     0x55f9f3787a37 - rustc_interface[a5b41241d2768600]::passes::configure_and_expand
  19:     0x55f9f37b3eef - <rustc_interface[a5b41241d2768600]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[a5b41241d2768600]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[6397b3f443c99aab]::result::Result<rustc_ast[ca3c63eecea61c62]::ast::Crate, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  20:     0x55f9f3775ab2 - <rustc_interface[a5b41241d2768600]::queries::Queries>::expansion
  21:     0x55f9f366c48c - <rustc_interface[a5b41241d2768600]::interface::Compiler>::enter::<rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}::{closure#2}, core[6397b3f443c99aab]::result::Result<core[6397b3f443c99aab]::option::Option<rustc_interface[a5b41241d2768600]::queries::Linker>, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  22:     0x55f9f365052b - rustc_span[5d78bee91e5c8a17]::with_source_map::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_interface[a5b41241d2768600]::interface::create_compiler_and_run<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#1}>
  23:     0x55f9f366d709 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::set::<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  24:     0x55f9f36c8789 - std[15814acf61565875]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  25:     0x55f9f366e691 - std[15814acf61565875]::panicking::try::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  26:     0x55f9f36c3792 - <<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1} as core[6397b3f443c99aab]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7fc898e59603 - std::sys::unix::thread::Thread::new::thread_start::h95090dc407d71b16
  28:     0x7fc89359f609 - start_thread
  29:     0x7fc893373133 - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (2df5a508b 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "lint-me" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_group_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread 'rustc' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7f2bcaa1a97c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h21bd9c7cdc48d0a4
   1:     0x7f2bcaa80fd8 - core::fmt::write::hc460522e84e320df
   2:     0x7f2bcaa0a801 - std::io::Write::write_fmt::h3fa3fb7cbe034a52
   3:     0x7f2bcaa1d9ce - std::panicking::default_hook::{{closure}}::h8acdb61d396c8dfb
   4:     0x7f2bcaa1d5fc - std::panicking::default_hook::hf0275d4850e0aea6
   5:     0x5601d80ea8e1 - rustc_driver[3f2533b0400fee3f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2bcaa1e22e - std::panicking::rust_panic_with_hook::h23a09bd700c5dc51
   7:     0x7f2bbed345eb - std[15814acf61565875]::panicking::begin_panic::<&str>::{closure#0}
   8:     0x7f2bbed34164 - std[15814acf61565875]::sys_common::backtrace::__rust_end_short_backtrace::<std[15814acf61565875]::panicking::begin_panic<&str>::{closure#0}, !>
   9:     0x7f2bbbf00f2c - std[15814acf61565875]::panicking::begin_panic::<&str>
  10:     0x7f2bbed3df01 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::with::<<rustc_span[5d78bee91e5c8a17]::symbol::Symbol>::as_str::{closure#0}, &str>
  11:     0x7f2bbbf0b0ea - <lint_group_plugin_test::Pass as rustc_lint::passes::LateLintPass>::check_item::h8a789a8b7d309dae
  12:     0x5601da382650 - <rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects as rustc_lint[462294dbdfc0c258]::passes::LateLintPass>::check_item
  13:     0x5601d82b2b7b - <rustc_lint[462294dbdfc0c258]::late::LateContextAndPass<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects> as rustc_hir[87fa454805819152]::intravisit::Visitor>::visit_nested_item
  14:     0x5601d82b7e6c - rustc_hir[87fa454805819152]::intravisit::walk_mod::<rustc_lint[462294dbdfc0c258]::late::LateContextAndPass<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects>>
  15:     0x5601d82ac154 - rustc_lint[462294dbdfc0c258]::late::late_lint_pass_crate::<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects>
  16:     0x5601d82ab715 - rustc_lint[462294dbdfc0c258]::late::late_lint_crate::<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass>
  17:     0x5601d824e12b - <rustc_session[c42791251753e321]::session::Session>::time::<(), rustc_lint[462294dbdfc0c258]::late::check_crate<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass, rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  18:     0x5601d823fbb2 - rustc_data_structures[d49369102ab912af]::sync::join::<rustc_lint[462294dbdfc0c258]::late::check_crate<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass, rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[462294dbdfc0c258]::late::check_crate<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass, rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  19:     0x5601d8251cd6 - std[15814acf61565875]::panicking::try::<(), core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  20:     0x5601d821a25b - <core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}> as core[6397b3f443c99aab]::ops::function::FnOnce<()>>::call_once
  21:     0x5601d824fb26 - <rustc_session[c42791251753e321]::session::Session>::time::<(), rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}>
  22:     0x5601d823405c - rustc_interface[a5b41241d2768600]::passes::analysis
  23:     0x5601d9adf77e - rustc_query_system[ef6bb102a4740ed2]::query::plumbing::try_execute_query::<rustc_query_impl[d63db2b8b9fd4a65]::plumbing::QueryCtxt, rustc_query_system[ef6bb102a4740ed2]::query::caches::DefaultCache<(), core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>>
  24:     0x5601d9bbf042 - rustc_query_system[ef6bb102a4740ed2]::query::plumbing::get_query::<rustc_query_impl[d63db2b8b9fd4a65]::queries::analysis, rustc_query_impl[d63db2b8b9fd4a65]::plumbing::QueryCtxt>
  25:     0x5601d9746e9e - <rustc_query_impl[d63db2b8b9fd4a65]::Queries as rustc_middle[50530032cda51348]::ty::query::QueryEngine>::analysis
  26:     0x5601d8167c14 - <rustc_interface[a5b41241d2768600]::passes::QueryContext>::enter::<rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  27:     0x5601d811459e - <rustc_interface[a5b41241d2768600]::interface::Compiler>::enter::<rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}::{closure#2}, core[6397b3f443c99aab]::result::Result<core[6397b3f443c99aab]::option::Option<rustc_interface[a5b41241d2768600]::queries::Linker>, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  28:     0x5601d80f852b - rustc_span[5d78bee91e5c8a17]::with_source_map::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_interface[a5b41241d2768600]::interface::create_compiler_and_run<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x5601d8115709 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::set::<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  30:     0x5601d8170789 - std[15814acf61565875]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  31:     0x5601d8116691 - std[15814acf61565875]::panicking::try::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  32:     0x5601d816b792 - <<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1} as core[6397b3f443c99aab]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7f2bcaa29603 - std::sys::unix::thread::Thread::new::thread_start::h95090dc407d71b16
  34:     0x7f2bc516f609 - start_thread
  35:     0x7f2bc4f43133 - clone
  36:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (2df5a508b 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui-fulldeps/lint-group-plugin.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_group_plugin_test)] //~ WARNING use of deprecated attribute
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread 'rustc' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7fecaeab797c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h21bd9c7cdc48d0a4
   1:     0x7fecaeb1dfd8 - core::fmt::write::hc460522e84e320df
   2:     0x7fecaeaa7801 - std::io::Write::write_fmt::h3fa3fb7cbe034a52
   3:     0x7fecaeaba9ce - std::panicking::default_hook::{{closure}}::h8acdb61d396c8dfb
   4:     0x7fecaeaba5fc - std::panicking::default_hook::hf0275d4850e0aea6
   5:     0x565281e6b8e1 - rustc_driver[3f2533b0400fee3f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fecaeabb22e - std::panicking::rust_panic_with_hook::h23a09bd700c5dc51
   7:     0x7feca2d345eb - std[15814acf61565875]::panicking::begin_panic::<&str>::{closure#0}
   8:     0x7feca2d34164 - std[15814acf61565875]::sys_common::backtrace::__rust_end_short_backtrace::<std[15814acf61565875]::panicking::begin_panic<&str>::{closure#0}, !>
   9:     0x7fec9ff00f2c - std[15814acf61565875]::panicking::begin_panic::<&str>
  10:     0x7feca2d3df01 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::with::<<rustc_span[5d78bee91e5c8a17]::symbol::Symbol>::as_str::{closure#0}, &str>
  11:     0x7fec9ff0b0ea - <lint_group_plugin_test::Pass as rustc_lint::passes::LateLintPass>::check_item::h8a789a8b7d309dae
  12:     0x565284103650 - <rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects as rustc_lint[462294dbdfc0c258]::passes::LateLintPass>::check_item
  13:     0x565282033b7b - <rustc_lint[462294dbdfc0c258]::late::LateContextAndPass<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects> as rustc_hir[87fa454805819152]::intravisit::Visitor>::visit_nested_item
  14:     0x565282038e6c - rustc_hir[87fa454805819152]::intravisit::walk_mod::<rustc_lint[462294dbdfc0c258]::late::LateContextAndPass<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects>>
  15:     0x56528202d154 - rustc_lint[462294dbdfc0c258]::late::late_lint_pass_crate::<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects>
  16:     0x56528202c715 - rustc_lint[462294dbdfc0c258]::late::late_lint_crate::<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass>
  17:     0x565281fcf12b - <rustc_session[c42791251753e321]::session::Session>::time::<(), rustc_lint[462294dbdfc0c258]::late::check_crate<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass, rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  18:     0x565281fc0bb2 - rustc_data_structures[d49369102ab912af]::sync::join::<rustc_lint[462294dbdfc0c258]::late::check_crate<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass, rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[462294dbdfc0c258]::late::check_crate<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass, rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  19:     0x565281fd2cd6 - std[15814acf61565875]::panicking::try::<(), core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  20:     0x565281f9b25b - <core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}> as core[6397b3f443c99aab]::ops::function::FnOnce<()>>::call_once
  21:     0x565281fd0b26 - <rustc_session[c42791251753e321]::session::Session>::time::<(), rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}>
  22:     0x565281fb505c - rustc_interface[a5b41241d2768600]::passes::analysis
  23:     0x56528386077e - rustc_query_system[ef6bb102a4740ed2]::query::plumbing::try_execute_query::<rustc_query_impl[d63db2b8b9fd4a65]::plumbing::QueryCtxt, rustc_query_system[ef6bb102a4740ed2]::query::caches::DefaultCache<(), core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>>
  24:     0x565283940042 - rustc_query_system[ef6bb102a4740ed2]::query::plumbing::get_query::<rustc_query_impl[d63db2b8b9fd4a65]::queries::analysis, rustc_query_impl[d63db2b8b9fd4a65]::plumbing::QueryCtxt>
  25:     0x5652834c7e9e - <rustc_query_impl[d63db2b8b9fd4a65]::Queries as rustc_middle[50530032cda51348]::ty::query::QueryEngine>::analysis
  26:     0x565281ee8c14 - <rustc_interface[a5b41241d2768600]::passes::QueryContext>::enter::<rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  27:     0x565281e9559e - <rustc_interface[a5b41241d2768600]::interface::Compiler>::enter::<rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}::{closure#2}, core[6397b3f443c99aab]::result::Result<core[6397b3f443c99aab]::option::Option<rustc_interface[a5b41241d2768600]::queries::Linker>, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  28:     0x565281e7952b - rustc_span[5d78bee91e5c8a17]::with_source_map::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_interface[a5b41241d2768600]::interface::create_compiler_and_run<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x565281e96709 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::set::<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  30:     0x565281ef1789 - std[15814acf61565875]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  31:     0x565281e97691 - std[15814acf61565875]::panicking::try::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  32:     0x565281eec792 - <<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1} as core[6397b3f443c99aab]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7fecaeac6603 - std::sys::unix::thread::Thread::new::thread_start::h95090dc407d71b16
  34:     0x7feca920c609 - start_thread
  35:     0x7feca8fe0133 - clone
  36:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (2df5a508b 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui-fulldeps/issue-40001.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-40001.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(issue_40001_plugin)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread 'rustc' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7f034fba797c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h21bd9c7cdc48d0a4
   1:     0x7f034fc0dfd8 - core::fmt::write::hc460522e84e320df
   2:     0x7f034fb97801 - std::io::Write::write_fmt::h3fa3fb7cbe034a52
   3:     0x7f034fbaa9ce - std::panicking::default_hook::{{closure}}::h8acdb61d396c8dfb
   4:     0x7f034fbaa5fc - std::panicking::default_hook::hf0275d4850e0aea6
   5:     0x557aae91e8e1 - rustc_driver[3f2533b0400fee3f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f034fbab22e - std::panicking::rust_panic_with_hook::h23a09bd700c5dc51
   7:     0x7f0342d339cb - std[15814acf61565875]::panicking::begin_panic::<&str>::{closure#0}
   8:     0x7f0342d33544 - std[15814acf61565875]::sys_common::backtrace::__rust_end_short_backtrace::<std[15814acf61565875]::panicking::begin_panic<&str>::{closure#0}, !>
   9:     0x7f033fefff2c - std[15814acf61565875]::panicking::begin_panic::<&str>
  10:     0x7f0342d3d2e1 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::with::<<rustc_span[5d78bee91e5c8a17]::symbol::Symbol>::as_str::{closure#0}, &str>
  11:     0x7f0342d20ce8 - <rustc_span[5d78bee91e5c8a17]::symbol::IdentPrinter as core[6397b3f443c99aab]::fmt::Display>::fmt
  12:     0x7f0342c852e4 - <rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::state::State as rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::state::PrintState>::print_ident
  13:     0x7f0342c83221 - <rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::state::State as rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::state::PrintState>::print_path
  14:     0x7f0342c827be - <rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::state::State as rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::state::PrintState>::print_attr_item
  15:     0x7f0342c826b5 - <rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::state::State as rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::state::PrintState>::print_attribute_inline
  16:     0x7f0342c84817 - <rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::state::State as rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::state::PrintState>::attribute_to_string
  17:     0x7f0342cac45c - rustc_ast_pretty[3ad1ab78b5129ca6]::pprust::attribute_to_string
  18:     0x7f033ff0a13c - <issue_40001_plugin::MissingAllowedAttrPass as rustc_lint::passes::LateLintPass>::check_fn::h76c596b8ddd222e4
  19:     0x557ab0bb6c8b - <rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects as rustc_lint[462294dbdfc0c258]::passes::LateLintPass>::check_fn
  20:     0x557aaeae5f46 - <rustc_lint[462294dbdfc0c258]::late::LateContextAndPass<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects> as rustc_hir[87fa454805819152]::intravisit::Visitor>::visit_fn
  21:     0x557aaeaed458 - rustc_hir[87fa454805819152]::intravisit::walk_item::<rustc_lint[462294dbdfc0c258]::late::LateContextAndPass<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects>>
  22:     0x557aaeae6b86 - <rustc_lint[462294dbdfc0c258]::late::LateContextAndPass<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects> as rustc_hir[87fa454805819152]::intravisit::Visitor>::visit_nested_item
  23:     0x557aaeaebe6c - rustc_hir[87fa454805819152]::intravisit::walk_mod::<rustc_lint[462294dbdfc0c258]::late::LateContextAndPass<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects>>
  24:     0x557aaeae0154 - rustc_lint[462294dbdfc0c258]::late::late_lint_pass_crate::<rustc_lint[462294dbdfc0c258]::late::LateLintPassObjects>
  25:     0x557aaeadf715 - rustc_lint[462294dbdfc0c258]::late::late_lint_crate::<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass>
  26:     0x557aaea8212b - <rustc_session[c42791251753e321]::session::Session>::time::<(), rustc_lint[462294dbdfc0c258]::late::check_crate<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass, rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  27:     0x557aaea73bb2 - rustc_data_structures[d49369102ab912af]::sync::join::<rustc_lint[462294dbdfc0c258]::late::check_crate<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass, rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[462294dbdfc0c258]::late::check_crate<rustc_lint[462294dbdfc0c258]::BuiltinCombinedLateLintPass, rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  28:     0x557aaea85cd6 - std[15814acf61565875]::panicking::try::<(), core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  29:     0x557aaea4e25b - <core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}::{closure#1}> as core[6397b3f443c99aab]::ops::function::FnOnce<()>>::call_once
  30:     0x557aaea83b26 - <rustc_session[c42791251753e321]::session::Session>::time::<(), rustc_interface[a5b41241d2768600]::passes::analysis::{closure#5}>
  31:     0x557aaea6805c - rustc_interface[a5b41241d2768600]::passes::analysis
  32:     0x557ab031377e - rustc_query_system[ef6bb102a4740ed2]::query::plumbing::try_execute_query::<rustc_query_impl[d63db2b8b9fd4a65]::plumbing::QueryCtxt, rustc_query_system[ef6bb102a4740ed2]::query::caches::DefaultCache<(), core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>>
  33:     0x557ab03f3042 - rustc_query_system[ef6bb102a4740ed2]::query::plumbing::get_query::<rustc_query_impl[d63db2b8b9fd4a65]::queries::analysis, rustc_query_impl[d63db2b8b9fd4a65]::plumbing::QueryCtxt>
  34:     0x557aaff7ae9e - <rustc_query_impl[d63db2b8b9fd4a65]::Queries as rustc_middle[50530032cda51348]::ty::query::QueryEngine>::analysis
  35:     0x557aae99bc14 - <rustc_interface[a5b41241d2768600]::passes::QueryContext>::enter::<rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  36:     0x557aae94859e - <rustc_interface[a5b41241d2768600]::interface::Compiler>::enter::<rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}::{closure#2}, core[6397b3f443c99aab]::result::Result<core[6397b3f443c99aab]::option::Option<rustc_interface[a5b41241d2768600]::queries::Linker>, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  37:     0x557aae92c52b - rustc_span[5d78bee91e5c8a17]::with_source_map::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_interface[a5b41241d2768600]::interface::create_compiler_and_run<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#1}>
  38:     0x557aae949709 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::set::<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  39:     0x557aae9a4789 - std[15814acf61565875]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  40:     0x557aae94a691 - std[15814acf61565875]::panicking::try::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x557aae99f792 - <<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1} as core[6397b3f443c99aab]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7f034fbb6603 - std::sys::unix::thread::Thread::new::thread_start::h95090dc407d71b16
  43:     0x7f034a2fc609 - start_thread
  44:     0x7f034a0d0133 - clone
  45:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (2df5a508b 2022-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


thread 'rustc' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
stack backtrace:
   0:     0x7f27f141897c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h21bd9c7cdc48d0a4
   1:     0x7f27f147efd8 - core::fmt::write::hc460522e84e320df
   2:     0x7f27f1408801 - std::io::Write::write_fmt::h3fa3fb7cbe034a52
   3:     0x7f27f141b9ce - std::panicking::default_hook::{{closure}}::h8acdb61d396c8dfb
   4:     0x7f27f141b5fc - std::panicking::default_hook::hf0275d4850e0aea6
   5:     0x55b5bf6f88e1 - rustc_driver[3f2533b0400fee3f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f27f141c22e - std::panicking::rust_panic_with_hook::h23a09bd700c5dc51
   7:     0x7f27e2d343ab - std[15814acf61565875]::panicking::begin_panic::<&str>::{closure#0}
   8:     0x7f27e2d33f24 - std[15814acf61565875]::sys_common::backtrace::__rust_end_short_backtrace::<std[15814acf61565875]::panicking::begin_panic<&str>::{closure#0}, !>
   9:     0x7f27dff00f2c - std[15814acf61565875]::panicking::begin_panic::<&str>
  10:     0x7f27e2d3dcc1 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::with::<<rustc_span[5d78bee91e5c8a17]::symbol::Symbol>::as_str::{closure#0}, &str>
  11:     0x7f27dff0b01d - <lint_plugin_test::Pass as rustc_lint::passes::EarlyLintPass>::check_item::h0786d59c57c13ae5
  12:     0x55b5c198f470 - <rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects as rustc_lint[462294dbdfc0c258]::passes::EarlyLintPass>::check_item
  13:     0x55b5bf832847 - <rustc_lint[462294dbdfc0c258]::early::EarlyContextAndPass<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects> as rustc_ast[ca3c63eecea61c62]::visit::Visitor>::visit_item
  14:     0x55b5bf8869ef - rustc_ast[ca3c63eecea61c62]::visit::walk_crate::<rustc_lint[462294dbdfc0c258]::early::EarlyContextAndPass<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects>>
  15:     0x55b5bf8309b0 - rustc_lint[462294dbdfc0c258]::early::early_lint_node::<rustc_lint[462294dbdfc0c258]::early::EarlyLintPassObjects, &rustc_ast[ca3c63eecea61c62]::ast::Crate>
  16:     0x55b5bf82f29d - rustc_lint[462294dbdfc0c258]::early::check_ast_node::<rustc_lint[462294dbdfc0c258]::BuiltinCombinedEarlyLintPass, &rustc_ast[ca3c63eecea61c62]::ast::Crate>
  17:     0x55b5bf85ca1a - <rustc_session[c42791251753e321]::session::Session>::time::<(), rustc_interface[a5b41241d2768600]::passes::configure_and_expand::{closure#8}>
  18:     0x55b5bf83da37 - rustc_interface[a5b41241d2768600]::passes::configure_and_expand
  19:     0x55b5bf869eef - <rustc_interface[a5b41241d2768600]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[a5b41241d2768600]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[6397b3f443c99aab]::result::Result<rustc_ast[ca3c63eecea61c62]::ast::Crate, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  20:     0x55b5bf82bab2 - <rustc_interface[a5b41241d2768600]::queries::Queries>::expansion
  21:     0x55b5bf72248c - <rustc_interface[a5b41241d2768600]::interface::Compiler>::enter::<rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}::{closure#2}, core[6397b3f443c99aab]::result::Result<core[6397b3f443c99aab]::option::Option<rustc_interface[a5b41241d2768600]::queries::Linker>, rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  22:     0x55b5bf70652b - rustc_span[5d78bee91e5c8a17]::with_source_map::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_interface[a5b41241d2768600]::interface::create_compiler_and_run<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#1}>
  23:     0x55b5bf723709 - <scoped_tls[2131656f40589b58]::ScopedKey<rustc_span[5d78bee91e5c8a17]::SessionGlobals>>::set::<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  24:     0x55b5bf77e789 - std[15814acf61565875]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>
  25:     0x55b5bf724691 - std[15814acf61565875]::panicking::try::<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, core[6397b3f443c99aab]::panic::unwind_safe::AssertUnwindSafe<<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  26:     0x55b5bf779792 - <<std[15814acf61565875]::thread::Builder>::spawn_unchecked_<rustc_interface[a5b41241d2768600]::util::run_in_thread_pool_with_globals<rustc_interface[a5b41241d2768600]::interface::run_compiler<core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>, rustc_driver[3f2533b0400fee3f]::run_compiler::{closure#1}>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#0}, core[6397b3f443c99aab]::result::Result<(), rustc_errors[6a2fb7a492e50598]::ErrorGuaranteed>>::{closure#1} as core[6397b3f443c99aab]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f27f1427603 - std::sys::unix::thread::Thread::new::thread_start::h95090dc407d71b16
  28:     0x7f27ebb6d609 - start_thread
  29:     0x7f27eb941133 - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
