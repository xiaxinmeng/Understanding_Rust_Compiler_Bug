plain
---- [ui] src/test/ui/thir-tree.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/thir-tree.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=thir-tree" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::thir::Thir', compiler/rustc_mir_build/src/thir/cx/mod.rs:38:48
   0:     0x7f77e36209dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h03230d5968398730
   0:     0x7f77e36209dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h03230d5968398730
   1:     0x7f77e3686fe8 - core::fmt::write::h9b3c2049bba331b3
   2:     0x7f77e3610841 - std::io::Write::write_fmt::hd678b1e8609aa08d
   3:     0x7f77e3623a0e - std::panicking::default_hook::{{closure}}::h3fd5eb4f36285a1c
   4:     0x7f77e362363c - std::panicking::default_hook::h36e0f2860ae74e52
   5:     0x7f77e419f4f1 - rustc_driver[db522d2160af7791]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f77e362426e - std::panicking::rust_panic_with_hook::h24d88a3d24c101f0
   7:     0x7f77e3624067 - std::panicking::begin_panic_handler::{{closure}}::h35f9ec0395d0dc42
   8:     0x7f77e3620ef4 - std::sys_common::backtrace::__rust_end_short_backtrace::h2b3375b10a6894c8
   9:     0x7f77e3623d49 - rust_begin_unwind
  10:     0x7f77e35d8073 - core::panicking::panic_fmt::h8031d596a1ef0f77
  11:     0x7f77e508a27d - <rustc_data_structures[6133be247e612f70]::steal::Steal<rustc_middle[c39020904e6a13db]::thir::Thir>>::borrow
  12:     0x7f77e50db162 - rustc_mir_build[8b00df703a8f70cb]::thir::cx::thir_tree
  13:     0x7f77e5c4cc65 - rustc_query_system[1c4defb63036635d]::query::plumbing::get_query::<rustc_query_impl[ac1c4973d46531ac]::queries::thir_tree, rustc_query_impl[ac1c4973d46531ac]::plumbing::QueryCtxt>
  14:     0x7f77e5a7406a - <rustc_query_impl[ac1c4973d46531ac]::Queries as rustc_middle[c39020904e6a13db]::ty::query::QueryEngine>::thir_tree
  15:     0x7f77e4192ca8 - rustc_driver[db522d2160af7791]::pretty::print_after_hir_lowering
  16:     0x7f77e41841a5 - <rustc_interface[ce2316cb1afaa4a4]::passes::QueryContext>::enter::<rustc_driver[db522d2160af7791]::run_compiler::{closure#1}::{closure#2}::{closure#1}, core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>>
  17:     0x7f77e412fe1a - <rustc_interface[ce2316cb1afaa4a4]::interface::Compiler>::enter::<rustc_driver[db522d2160af7791]::run_compiler::{closure#1}::{closure#2}, core[7bccc78272a100a0]::result::Result<core[7bccc78272a100a0]::option::Option<rustc_interface[ce2316cb1afaa4a4]::queries::Linker>, rustc_errors[1ac103f251d19404]::ErrorGuaranteed>>
  18:     0x7f77e411161b - rustc_span[99587dde63df57c0]::with_source_map::<core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>, rustc_interface[ce2316cb1afaa4a4]::interface::create_compiler_and_run<core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>, rustc_driver[db522d2160af7791]::run_compiler::{closure#1}>::{closure#1}>
  19:     0x7f77e4131539 - <scoped_tls[36640aa5ad3fc60b]::ScopedKey<rustc_span[99587dde63df57c0]::SessionGlobals>>::set::<rustc_interface[ce2316cb1afaa4a4]::interface::run_compiler<core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>, rustc_driver[db522d2160af7791]::run_compiler::{closure#1}>::{closure#0}, core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>>
  20:     0x7f77e41872a9 - std[2402eae1a4c9a3c1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ce2316cb1afaa4a4]::util::run_in_thread_pool_with_globals<rustc_interface[ce2316cb1afaa4a4]::interface::run_compiler<core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>, rustc_driver[db522d2160af7791]::run_compiler::{closure#1}>::{closure#0}, core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>>::{closure#0}, core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>>
  21:     0x7f77e4132501 - std[2402eae1a4c9a3c1]::panicking::try::<core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>, core[7bccc78272a100a0]::panic::unwind_safe::AssertUnwindSafe<<std[2402eae1a4c9a3c1]::thread::Builder>::spawn_unchecked_<rustc_interface[ce2316cb1afaa4a4]::util::run_in_thread_pool_with_globals<rustc_interface[ce2316cb1afaa4a4]::interface::run_compiler<core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>, rustc_driver[db522d2160af7791]::run_compiler::{closure#1}>::{closure#0}, core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>>::{closure#0}, core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  22:     0x7f77e4187fc2 - <<std[2402eae1a4c9a3c1]::thread::Builder>::spawn_unchecked_<rustc_interface[ce2316cb1afaa4a4]::util::run_in_thread_pool_with_globals<rustc_interface[ce2316cb1afaa4a4]::interface::run_compiler<core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>, rustc_driver[db522d2160af7791]::run_compiler::{closure#1}>::{closure#0}, core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>>::{closure#0}, core[7bccc78272a100a0]::result::Result<(), rustc_errors[1ac103f251d19404]::ErrorGuaranteed>>::{closure#1} as core[7bccc78272a100a0]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  23:     0x7f77e362f5c3 - std::sys::unix::thread::Thread::new::thread_start::h56c396f1857cf473
  24:     0x7f77ddb81609 - start_thread
  25:     0x7f77e3494163 - clone
  26:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (54dc152ea 2022-05-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z unpretty=thir-tree
query stack during panic:
query stack during panic:
#0 [thir_tree] constructing THIR tree for `main`
------------------------------------------



