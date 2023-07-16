
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: `InferCtxt` incorrectly tainted by errors
  |
  = note: delayed at compiler/rustc_infer/src/infer/mod.rs:1282:27

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_hir_analysis/src/check/expr.rs:526:21

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/project.rs:1238:30

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_hir_analysis/src/check/fn_ctxt/_impl.rs:724:23

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_hir_analysis/src/check/coercion.rs:176:49

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_hir_analysis/src/check/fn_ctxt/checks.rs:109:29

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_hir_analysis/src/check/coercion.rs:963:53

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_hir_analysis/src/check/coercion.rs:1404:42

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_infer/src/infer/sub.rs:123:31

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_hir_analysis/src/check/fn_ctxt/checks.rs:1536:27

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_hir_analysis/src/check/fallback.rs:109:58

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/relate.rs:419:59

error: internal compiler error: expected fullfillment errors
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:210:23

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:627:18

error: internal compiler error: PromoteTemps: MIR had errors
 --> src/lib.rs:1:38
  |
2 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_lib_rs_1_0() {
  |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:53:22

error: internal compiler error: broken MIR in DefId(0:4 ~ rust_out[b9b2]::main::_doctest_main_src_lib_rs_1_0) ("return type"): bad type [type error]
 --> src/lib.rs:1:38
  |
2 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_lib_rs_1_0() {
  |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:523:13

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:791:20

error: internal compiler error: broken MIR in DefId(0:4 ~ rust_out[b9b2]::main::_doctest_main_src_lib_rs_1_0) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: src/lib.rs:2:38: 2:71 (#0), scope: scope[0] } }): bad type [type error]
 --> src/lib.rs:1:38
  |
2 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_lib_rs_1_0() {
  |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:523:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1535:13
stack backtrace:
   0:     0x7fee569b5ada - std::backtrace_rs::backtrace::libunwind::trace::h8f2eb234454895b5
                               at /home/nilsh/projects/rust/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fee569b5ada - std::backtrace_rs::backtrace::trace_unsynchronized::h92c29b31ac06fd9e
                               at /home/nilsh/projects/rust/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fee569b5ada - std::sys_common::backtrace::_print_fmt::h18c573bdedef2902
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fee569b5ada - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2cc87b43d8a562b6
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fee56a117f8 - core::fmt::write::hf898e0a9cc3c89a4
                               at /home/nilsh/projects/rust/library/core/src/fmt/mod.rs:1209:17
   5:     0x7fee56980da1 - std::io::Write::write_fmt::h2ad619589edaa38f
                               at /home/nilsh/projects/rust/library/std/src/io/mod.rs:1679:15
   6:     0x7fee569b592f - std::sys_common::backtrace::_print::h96ff3c099826a10a
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fee569b592f - std::sys_common::backtrace::print::hee5b5a20ab57e326
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fee5699e6e4 - std::panicking::default_hook::{{closure}}::hdb24a3f35c559c6e
   9:     0x7fee5699e49a - std::panicking::default_hook::ha3e88625b127343c
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:286:9
  10:     0x7fee57329c04 - <alloc[b389559989e95008]::boxed::Box<dyn for<'a, 'b> core[73fa0f4e5d90a029]::ops::function::Fn<(&'a core[73fa0f4e5d90a029]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[73fa0f4e5d90a029]::marker::Send + core[73fa0f4e5d90a029]::marker::Sync> as core[73fa0f4e5d90a029]::ops::function::Fn<(&core[73fa0f4e5d90a029]::panic::panic_info::PanicInfo,)>>::call
                               at /home/nilsh/projects/rust/library/alloc/src/boxed.rs:1952:9
  11:     0x7fee57329c04 - rustc_driver[371a0ea167db5f7f]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_driver/src/lib.rs:1195:13
  12:     0x7fee5699eb0d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hafc548451be5b4cf
                               at /home/nilsh/projects/rust/library/alloc/src/boxed.rs:1952:9
  13:     0x7fee5699eb0d - std::panicking::rust_panic_with_hook::hbd6e17eb7e759fb0
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:673:13
  14:     0x7fee59cdf713 - std[d559eab05d6b0603]::panicking::begin_panic::<rustc_errors[7f5df6e7b9fd1751]::ExplicitBug>::{closure#0}
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:589:9
  15:     0x7fee59cdf6c6 - std[d559eab05d6b0603]::sys_common::backtrace::__rust_end_short_backtrace::<std[d559eab05d6b0603]::panicking::begin_panic<rustc_errors[7f5df6e7b9fd1751]::ExplicitBug>::{closure#0}, !>
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:138:18
  16:     0x7fee572e74c6 - std[d559eab05d6b0603]::panicking::begin_panic::<rustc_errors[7f5df6e7b9fd1751]::ExplicitBug>
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:588:12
  17:     0x7fee59c89906 - std[d559eab05d6b0603]::panic::panic_any::<rustc_errors[7f5df6e7b9fd1751]::ExplicitBug>
                               at /home/nilsh/projects/rust/library/std/src/panic.rs:61:5
  18:     0x7fee59c8c61c - <rustc_errors[7f5df6e7b9fd1751]::HandlerInner>::flush_delayed::<alloc[b389559989e95008]::vec::Vec<rustc_errors[7f5df6e7b9fd1751]::diagnostic::Diagnostic>, &str>
                               at /home/nilsh/projects/rust/compiler/rustc_errors/src/lib.rs:1535:13
  19:     0x7fee59ca7743 - <rustc_errors[7f5df6e7b9fd1751]::Handler>::flush_delayed
                               at /home/nilsh/projects/rust/compiler/rustc_errors/src/lib.rs:1140:9
  20:     0x7fee57468eb9 - <rustc_interface[7313dbe8dd443abf]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/queries.rs:249:17
  21:     0x7fee57468eb9 - <rustc_interface[7313dbe8dd443abf]::passes::QueryContext>::enter::<<rustc_interface[7313dbe8dd443abf]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[73fa0f4e5d90a029]::result::Result<alloc[b389559989e95008]::boxed::Box<dyn core[73fa0f4e5d90a029]::any::Any>, rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/passes.rs:769:42
  22:     0x7fee57468eb9 - rustc_middle[3b85d9020c912d92]::ty::context::tls::enter_context::<<rustc_interface[7313dbe8dd443abf]::passes::QueryContext>::enter<<rustc_interface[7313dbe8dd443abf]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[73fa0f4e5d90a029]::result::Result<alloc[b389559989e95008]::boxed::Box<dyn core[73fa0f4e5d90a029]::any::Any>, rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<alloc[b389559989e95008]::boxed::Box<dyn core[73fa0f4e5d90a029]::any::Any>, rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1928:50
  23:     0x7fee57468eb9 - rustc_middle[3b85d9020c912d92]::ty::context::tls::set_tlv::<rustc_middle[3b85d9020c912d92]::ty::context::tls::enter_context<<rustc_interface[7313dbe8dd443abf]::passes::QueryContext>::enter<<rustc_interface[7313dbe8dd443abf]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[73fa0f4e5d90a029]::result::Result<alloc[b389559989e95008]::boxed::Box<dyn core[73fa0f4e5d90a029]::any::Any>, rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<alloc[b389559989e95008]::boxed::Box<dyn core[73fa0f4e5d90a029]::any::Any>, rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<alloc[b389559989e95008]::boxed::Box<dyn core[73fa0f4e5d90a029]::any::Any>, rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1912:9
  24:     0x7fee57468eb9 - rustc_middle[3b85d9020c912d92]::ty::context::tls::enter_context::<<rustc_interface[7313dbe8dd443abf]::passes::QueryContext>::enter<<rustc_interface[7313dbe8dd443abf]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[73fa0f4e5d90a029]::result::Result<alloc[b389559989e95008]::boxed::Box<dyn core[73fa0f4e5d90a029]::any::Any>, rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<alloc[b389559989e95008]::boxed::Box<dyn core[73fa0f4e5d90a029]::any::Any>, rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1928:9
  25:     0x7fee57468eb9 - <rustc_interface[7313dbe8dd443abf]::passes::QueryContext>::enter::<<rustc_interface[7313dbe8dd443abf]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[73fa0f4e5d90a029]::result::Result<alloc[b389559989e95008]::boxed::Box<dyn core[73fa0f4e5d90a029]::any::Any>, rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/passes.rs:769:9
  26:     0x7fee57469bed - <rustc_interface[7313dbe8dd443abf]::queries::Queries>::ongoing_codegen::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/queries.rs:243:13
  27:     0x7fee57469bed - <rustc_interface[7313dbe8dd443abf]::queries::Query<alloc[b389559989e95008]::boxed::Box<dyn core[73fa0f4e5d90a029]::any::Any>>>::compute::<<rustc_interface[7313dbe8dd443abf]::queries::Queries>::ongoing_codegen::{closure#0}>
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/queries.rs:38:28
  28:     0x7fee57330c4e - rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}::{closure#2}
                               at /home/nilsh/projects/rust/compiler/rustc_driver/src/lib.rs:408:13
  29:     0x7fee57330c4e - <rustc_interface[7313dbe8dd443abf]::interface::Compiler>::enter::<rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}::{closure#2}, core[73fa0f4e5d90a029]::result::Result<core[73fa0f4e5d90a029]::option::Option<rustc_interface[7313dbe8dd443abf]::queries::Linker>, rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/queries.rs:379:19
  30:     0x7fee57316ff2 - rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}
                               at /home/nilsh/projects/rust/compiler/rustc_driver/src/lib.rs:316:22
  31:     0x7fee57316ff2 - rustc_interface[7313dbe8dd443abf]::interface::create_compiler_and_run::<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#1}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/interface.rs:323:13
  32:     0x7fee57316ff2 - rustc_span[84dbe569a712e516]::with_source_map::<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_interface[7313dbe8dd443abf]::interface::create_compiler_and_run<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#1}>
                               at /home/nilsh/projects/rust/compiler/rustc_span/src/lib.rs:999:5
  33:     0x7fee57331759 - rustc_interface[7313dbe8dd443abf]::interface::create_compiler_and_run::<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/interface.rs:317:5
  34:     0x7fee57337e82 - rustc_interface[7313dbe8dd443abf]::interface::run_compiler::<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/interface.rs:339:12
  35:     0x7fee57337e82 - <scoped_tls[4b2a4c8958d0f0a7]::ScopedKey<rustc_span[84dbe569a712e516]::SessionGlobals>>::set::<rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>
                               at /home/nilsh/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  36:     0x7fee57303aea - rustc_span[84dbe569a712e516]::create_session_globals_then::<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}>
                               at /home/nilsh/projects/rust/compiler/rustc_span/src/lib.rs:111:5
  37:     0x7fee57303aea - rustc_interface[7313dbe8dd443abf]::util::run_in_thread_pool_with_globals::<rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/util.rs:159:32
  38:     0x7fee57303aea - std[d559eab05d6b0603]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7313dbe8dd443abf]::util::run_in_thread_pool_with_globals<rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:122:18
  39:     0x7fee5730560e - <std[d559eab05d6b0603]::thread::Builder>::spawn_unchecked_::<rustc_interface[7313dbe8dd443abf]::util::run_in_thread_pool_with_globals<rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /home/nilsh/projects/rust/library/std/src/thread/mod.rs:514:17
  40:     0x7fee5730560e - <core[73fa0f4e5d90a029]::panic::unwind_safe::AssertUnwindSafe<<std[d559eab05d6b0603]::thread::Builder>::spawn_unchecked_<rustc_interface[7313dbe8dd443abf]::util::run_in_thread_pool_with_globals<rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[73fa0f4e5d90a029]::ops::function::FnOnce<()>>::call_once
                               at /home/nilsh/projects/rust/library/core/src/panic/unwind_safe.rs:271:9
  41:     0x7fee5730560e - std[d559eab05d6b0603]::panicking::try::do_call::<core[73fa0f4e5d90a029]::panic::unwind_safe::AssertUnwindSafe<<std[d559eab05d6b0603]::thread::Builder>::spawn_unchecked_<rustc_interface[7313dbe8dd443abf]::util::run_in_thread_pool_with_globals<rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:464:40
  42:     0x7fee5730560e - std[d559eab05d6b0603]::panicking::try::<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, core[73fa0f4e5d90a029]::panic::unwind_safe::AssertUnwindSafe<<std[d559eab05d6b0603]::thread::Builder>::spawn_unchecked_<rustc_interface[7313dbe8dd443abf]::util::run_in_thread_pool_with_globals<rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:428:19
  43:     0x7fee5731cf04 - std[d559eab05d6b0603]::panic::catch_unwind::<core[73fa0f4e5d90a029]::panic::unwind_safe::AssertUnwindSafe<<std[d559eab05d6b0603]::thread::Builder>::spawn_unchecked_<rustc_interface[7313dbe8dd443abf]::util::run_in_thread_pool_with_globals<rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/library/std/src/panic.rs:137:14
  44:     0x7fee5731cf04 - <std[d559eab05d6b0603]::thread::Builder>::spawn_unchecked_::<rustc_interface[7313dbe8dd443abf]::util::run_in_thread_pool_with_globals<rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#1}
                               at /home/nilsh/projects/rust/library/std/src/thread/mod.rs:513:30
  45:     0x7fee5731cf04 - <<std[d559eab05d6b0603]::thread::Builder>::spawn_unchecked_<rustc_interface[7313dbe8dd443abf]::util::run_in_thread_pool_with_globals<rustc_interface[7313dbe8dd443abf]::interface::run_compiler<core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>, rustc_driver[371a0ea167db5f7f]::run_compiler::{closure#1}>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#0}, core[73fa0f4e5d90a029]::result::Result<(), rustc_errors[7f5df6e7b9fd1751]::ErrorGuaranteed>>::{closure#1} as core[73fa0f4e5d90a029]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /home/nilsh/projects/rust/library/core/src/ops/function.rs:251:5
  46:     0x7fee5697a818 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h73d336d1c9e65b4a
                               at /home/nilsh/projects/rust/library/alloc/src/boxed.rs:1938:9
  47:     0x7fee5697a818 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::ha38990988e6c8e8b
                               at /home/nilsh/projects/rust/library/alloc/src/boxed.rs:1938:9
  48:     0x7fee56995f97 - std::sys::unix::thread::Thread::new::thread_start::h0e8d69c155def017
                               at /home/nilsh/projects/rust/library/std/src/sys/unix/thread.rs:108:17
  49:     0x7fee56750b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  50:     0x7fee567e2a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  51:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C codegen-units=1 -C embed-bitcode=no

query stack during panic:
end of query stack
error: aborting due to 19 previous errors
