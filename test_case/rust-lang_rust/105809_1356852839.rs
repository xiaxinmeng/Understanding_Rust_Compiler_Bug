
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:969:13: failed to get layout for `Connection<H>`: the type `H` has an unknown layout

thread 'rustc' panicked at 'Box<dyn Any>', /home/nilsh/projects/rust/compiler/rustc_errors/src/lib.rs:973:33
stack backtrace:
   0:     0x7f2b3af34647 - std::backtrace_rs::backtrace::libunwind::trace::hbda7f17d8173f1da
                               at /home/nilsh/projects/rust/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f2b3af34647 - std::backtrace_rs::backtrace::trace_unsynchronized::h8133d047512bc619
                               at /home/nilsh/projects/rust/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f2b3af34647 - std::sys_common::backtrace::_print_fmt::h2bc30874d5baf609
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f2b3af34647 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4dca3ce2a252ea70
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f2b3b00ab28 - core::fmt::write::hb4f3f6f08e10fabf
                               at /home/nilsh/projects/rust/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f2b3af99fe1 - std::io::Write::write_fmt::hf860c428b5d56ea2
                               at /home/nilsh/projects/rust/library/std/src/io/mod.rs:1682:15
   6:     0x7f2b3af3446a - std::sys_common::backtrace::_print::hc34de8c5877b433d
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f2b3af3446a - std::sys_common::backtrace::print::ha0dbaa3c0bce9d10
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f2b3af414c7 - std::panicking::default_hook::{{closure}}::h86122d147d40ec67
   9:     0x7f2b3af412c0 - std::panicking::default_hook::h1c25ed6507f0a719
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:286:9
  10:     0x7f2b3c7f6ff4 - <alloc[d426fd0be2d16a6b]::boxed::Box<dyn for<'a, 'b> core[584d80f921d3c2b7]::ops::function::Fn<(&'a core[584d80f921d3c2b7]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[584d80f921d3c2b7]::marker::Sync + core[584d80f921d3c2b7]::marker::Send> as core[584d80f921d3c2b7]::ops::function::Fn<(&core[584d80f921d3c2b7]::panic::panic_info::PanicInfo,)>>::call
                               at /home/nilsh/projects/rust/library/alloc/src/boxed.rs:2024:9
  11:     0x7f2b3c7f6ff4 - rustc_driver[c0ebef4b5a350350]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_driver/src/lib.rs:1202:13
  12:     0x7f2b3c7f6ff4 - <rustc_driver[c0ebef4b5a350350]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[584d80f921d3c2b7]::ops::function::FnOnce<(&core[584d80f921d3c2b7]::panic::panic_info::PanicInfo,)>>::call_once
                               at /home/nilsh/projects/rust/library/core/src/ops/function.rs:422:5
  13:     0x7f2b3c7f6ff4 - <rustc_driver[c0ebef4b5a350350]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[584d80f921d3c2b7]::ops::function::FnOnce<(&core[584d80f921d3c2b7]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
                               at /home/nilsh/projects/rust/library/core/src/ops/function.rs:422:5
  14:     0x7f2b3af41b54 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hf5a3aab415c4e1e7
                               at /home/nilsh/projects/rust/library/alloc/src/boxed.rs:2024:9
  15:     0x7f2b3af41b54 - std::panicking::rust_panic_with_hook::h4cc86ae4240d3987
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:692:13
  16:     0x7f2b3cb0af13 - std[adc46dc960aa655]::panicking::begin_panic::<rustc_errors[2fbccf98e0fb6daa]::ExplicitBug>::{closure#0}
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:608:9
  17:     0x7f2b3cb075d6 - std[adc46dc960aa655]::sys_common::backtrace::__rust_end_short_backtrace::<std[adc46dc960aa655]::panicking::begin_panic<rustc_errors[2fbccf98e0fb6daa]::ExplicitBug>::{closure#0}, !>
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:137:18
  18:     0x7f2b3cb06c46 - std[adc46dc960aa655]::panicking::begin_panic::<rustc_errors[2fbccf98e0fb6daa]::ExplicitBug>
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:607:12
  19:     0x7f2b3cb40eb6 - std[adc46dc960aa655]::panic::panic_any::<rustc_errors[2fbccf98e0fb6daa]::ExplicitBug>
                               at /home/nilsh/projects/rust/library/std/src/panic.rs:61:5
  20:     0x7f2b3cb40e58 - <rustc_errors[2fbccf98e0fb6daa]::HandlerInner>::span_bug::<rustc_span[d50b082f3e833889]::span_encoding::Span, &alloc[d426fd0be2d16a6b]::string::String>
                               at /home/nilsh/projects/rust/compiler/rustc_errors/src/lib.rs:1514:9
  21:     0x7f2b3cb40a17 - <rustc_errors[2fbccf98e0fb6daa]::Handler>::span_bug::<rustc_span[d50b082f3e833889]::span_encoding::Span, &alloc[d426fd0be2d16a6b]::string::String>
                               at /home/nilsh/projects/rust/compiler/rustc_errors/src/lib.rs:973:9
  22:     0x7f2b3cb5a564 - rustc_middle[59b60a8cad6ad26d]::util::bug::opt_span_bug_fmt::<rustc_span[d50b082f3e833889]::span_encoding::Span>::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/util/bug.rs:34:40
  23:     0x7f2b3cb5a564 - rustc_middle[59b60a8cad6ad26d]::ty::context::tls::with_opt::<rustc_middle[59b60a8cad6ad26d]::util::bug::opt_span_bug_fmt<rustc_span[d50b082f3e833889]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1376:40
  24:     0x7f2b3cb5a564 - rustc_middle[59b60a8cad6ad26d]::ty::context::tls::with_context_opt::<rustc_middle[59b60a8cad6ad26d]::ty::context::tls::with_opt<rustc_middle[59b60a8cad6ad26d]::util::bug::opt_span_bug_fmt<rustc_span[d50b082f3e833889]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1328:22
  25:     0x7f2b3cb5a564 - rustc_middle[59b60a8cad6ad26d]::ty::context::tls::with_opt::<rustc_middle[59b60a8cad6ad26d]::util::bug::opt_span_bug_fmt<rustc_span[d50b082f3e833889]::span_encoding::Span>::{closure#0}, ()>
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1376:9
  26:     0x7f2b3cb5a3a9 - rustc_middle[59b60a8cad6ad26d]::util::bug::opt_span_bug_fmt::<rustc_span[d50b082f3e833889]::span_encoding::Span>
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/util/bug.rs:31:5
  27:     0x7f2b3cb5a367 - rustc_middle[59b60a8cad6ad26d]::util::bug::span_bug_fmt::<rustc_span[d50b082f3e833889]::span_encoding::Span>
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/util/bug.rs:22:5
  28:     0x7f2b3cb0bf07 - <rustc_codegen_llvm[ef8529757c6fb92]::context::CodegenCx as rustc_middle[59b60a8cad6ad26d]::ty::layout::LayoutOfHelpers>::handle_layout_err
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_llvm/src/context.rs:969:13
  29:     0x7f2b3cb0b812 - <rustc_codegen_llvm[ef8529757c6fb92]::context::CodegenCx as rustc_middle[59b60a8cad6ad26d]::ty::layout::LayoutOf>::spanned_layout_of::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/layout.rs:528:32
  30:     0x7f2b3cb0bd82 - <core[584d80f921d3c2b7]::result::Result<rustc_target[9d5e5d6a8fd248c]::abi::TyAndLayout<rustc_middle[59b60a8cad6ad26d]::ty::Ty>, rustc_middle[59b60a8cad6ad26d]::ty::layout::LayoutError>>::map_err::<!, <rustc_codegen_llvm[ef8529757c6fb92]::context::CodegenCx as rustc_middle[59b60a8cad6ad26d]::ty::layout::LayoutOf>::spanned_layout_of::{closure#0}>
                               at /home/nilsh/projects/rust/library/core/src/result.rs:860:27
  31:     0x7f2b3cb0bd82 - <rustc_codegen_llvm[ef8529757c6fb92]::context::CodegenCx as rustc_middle[59b60a8cad6ad26d]::ty::layout::LayoutOf>::spanned_layout_of
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/layout.rs:527:13
  32:     0x7f2b3cb0bd82 - <rustc_codegen_llvm[ef8529757c6fb92]::context::CodegenCx as rustc_middle[59b60a8cad6ad26d]::ty::layout::LayoutOf>::layout_of
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/layout.rs:514:9
  33:     0x7f2b3cb0c63c - <rustc_target[9d5e5d6a8fd248c]::abi::TyAndLayout<rustc_middle[59b60a8cad6ad26d]::ty::Ty> as rustc_codegen_llvm[ef8529757c6fb92]::type_of::LayoutLlvmExt>::llvm_type
  34:     0x7f2b3cb0f2bc - rustc_codegen_llvm[ef8529757c6fb92]::type_of::struct_llfields
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_llvm/src/type_of.rs:138:21
  35:     0x7f2b3cb0e5f2 - rustc_codegen_llvm[ef8529757c6fb92]::type_of::uncached_llvm_type
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_llvm/src/type_of.rs:88:63
  36:     0x7f2b3cb0c5dd - <rustc_target[9d5e5d6a8fd248c]::abi::TyAndLayout<rustc_middle[59b60a8cad6ad26d]::ty::Ty> as rustc_codegen_llvm[ef8529757c6fb92]::type_of::LayoutLlvmExt>::llvm_type
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_llvm/src/type_of.rs:276:13
  37:     0x7f2b3cad8756 - <rustc_codegen_llvm[ef8529757c6fb92]::context::CodegenCx as rustc_codegen_ssa[9d9dcb37c59659f6]::traits::type_::LayoutTypeMethods>::backend_type
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_llvm/src/type_.rs:257:9
  38:     0x7f2b3cad8756 - <rustc_codegen_ssa[9d9dcb37c59659f6]::mir::place::PlaceRef<&rustc_codegen_llvm[ef8529757c6fb92]::llvm_::ffi::Value>>::project_type::<rustc_codegen_llvm[ef8529757c6fb92]::builder::Builder>
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/mir/place.rs:517:26
  39:     0x7f2b3cb23f98 - <rustc_codegen_ssa[9d9dcb37c59659f6]::mir::FunctionCx<rustc_codegen_llvm[ef8529757c6fb92]::builder::Builder>>::codegen_place
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/mir/place.rs:565:56
  40:     0x7f2b3cb196fd - <rustc_codegen_ssa[9d9dcb37c59659f6]::mir::FunctionCx<rustc_codegen_llvm[ef8529757c6fb92]::builder::Builder>>::codegen_place_to_pointer::<<rustc_codegen_ssa[9d9dcb37c59659f6]::mir::FunctionCx<rustc_codegen_llvm[ef8529757c6fb92]::builder::Builder>>::codegen_rvalue_operand::{closure#0}>
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/mir/rvalue.rs:511:24
  41:     0x7f2b3cb196fd - <rustc_codegen_ssa[9d9dcb37c59659f6]::mir::FunctionCx<rustc_codegen_llvm[ef8529757c6fb92]::builder::Builder>>::codegen_rvalue_operand
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/mir/rvalue.rs:356:17
  42:     0x7f2b3cb14b54 - <rustc_codegen_ssa[9d9dcb37c59659f6]::mir::FunctionCx<rustc_codegen_llvm[ef8529757c6fb92]::builder::Builder>>::codegen_statement
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/mir/statement.rs:22:43
  43:     0x7f2b3cb14b54 - <rustc_codegen_ssa[9d9dcb37c59659f6]::mir::FunctionCx<rustc_codegen_llvm[ef8529757c6fb92]::builder::Builder>>::codegen_block
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/mir/block.rs:1226:17
  44:     0x7f2b3cb14b54 - rustc_codegen_ssa[9d9dcb37c59659f6]::mir::codegen_mir::<rustc_codegen_llvm[ef8529757c6fb92]::builder::Builder>
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/mir/mod.rs:261:9
  45:     0x7f2b3ca4ea28 - rustc_codegen_ssa[9d9dcb37c59659f6]::base::codegen_instance::<rustc_codegen_llvm[ef8529757c6fb92]::builder::Builder>
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/base.rs:398:5
  46:     0x7f2b3cb51f86 - <rustc_middle[59b60a8cad6ad26d]::mir::mono::MonoItem as rustc_codegen_ssa[9d9dcb37c59659f6]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[ef8529757c6fb92]::builder::Builder>
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/mono_item.rs:91:17
  47:     0x7f2b3c9f7c6b - rustc_codegen_llvm[ef8529757c6fb92]::base::compile_codegen_unit::module_codegen
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_llvm/src/base.rs:95:17
  48:     0x7f2b3cb3c642 - <rustc_query_system[e73a8ec897c74754]::dep_graph::graph::DepGraph<rustc_middle[59b60a8cad6ad26d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[59b60a8cad6ad26d]::ty::context::TyCtxt, rustc_span[d50b082f3e833889]::symbol::Symbol, rustc_codegen_ssa[9d9dcb37c59659f6]::ModuleCodegen<rustc_codegen_llvm[ef8529757c6fb92]::ModuleLlvm>>
                               at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:296:14
  49:     0x7f2b3c9f788a - rustc_codegen_llvm[ef8529757c6fb92]::base::compile_codegen_unit
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_llvm/src/base.rs:64:23
  50:     0x7f2b3ca4dd2e - <rustc_codegen_llvm[ef8529757c6fb92]::LlvmCodegenBackend as rustc_codegen_ssa[9d9dcb37c59659f6]::traits::backend::ExtraBackendMethods>::compile_codegen_unit
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_llvm/src/lib.rs:128:9
  51:     0x7f2b3ca4dd2e - rustc_codegen_ssa[9d9dcb37c59659f6]::base::codegen_crate::<rustc_codegen_llvm[ef8529757c6fb92]::LlvmCodegenBackend>
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/base.rs:737:34
  52:     0x7f2b3cb6ffb4 - <rustc_codegen_llvm[ef8529757c6fb92]::LlvmCodegenBackend as rustc_codegen_ssa[9d9dcb37c59659f6]::traits::backend::CodegenBackend>::codegen_crate
                               at /home/nilsh/projects/rust/compiler/rustc_codegen_llvm/src/lib.rs:335:18
  53:     0x7f2b3c9beaff - rustc_interface[a457c10140b33d94]::passes::start_codegen::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/passes.rs:989:9
  54:     0x7f2b3c9beaff - <rustc_data_structures[b3d31ca178a02ff3]::profiling::VerboseTimingGuard>::run::<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_interface[a457c10140b33d94]::passes::start_codegen::{closure#0}>
                               at /home/nilsh/projects/rust/compiler/rustc_data_structures/src/profiling.rs:726:9
  55:     0x7f2b3c9beaff - <rustc_session[dd3e8ae2581a4383]::session::Session>::time::<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_interface[a457c10140b33d94]::passes::start_codegen::{closure#0}>
                               at /home/nilsh/projects/rust/compiler/rustc_session/src/utils.rs:10:9
  56:     0x7f2b3c98b979 - rustc_interface[a457c10140b33d94]::passes::start_codegen
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/passes.rs:988:19
  57:     0x7f2b3c9dd69d - <rustc_interface[a457c10140b33d94]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/queries.rs:253:20
  58:     0x7f2b3c9dd69d - <rustc_interface[a457c10140b33d94]::passes::QueryContext>::enter::<<rustc_interface[a457c10140b33d94]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/passes.rs:765:42
  59:     0x7f2b3c9dd69d - rustc_middle[59b60a8cad6ad26d]::ty::context::tls::enter_context::<<rustc_interface[a457c10140b33d94]::passes::QueryContext>::enter<<rustc_interface[a457c10140b33d94]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}, core[584d80f921d3c2b7]::result::Result<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
  60:     0x7f2b3c9dd69d - rustc_middle[59b60a8cad6ad26d]::ty::context::tls::set_tlv::<rustc_middle[59b60a8cad6ad26d]::ty::context::tls::enter_context<<rustc_interface[a457c10140b33d94]::passes::QueryContext>::enter<<rustc_interface[a457c10140b33d94]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}, core[584d80f921d3c2b7]::result::Result<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}, core[584d80f921d3c2b7]::result::Result<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
  61:     0x7f2b3c9dd69d - rustc_middle[59b60a8cad6ad26d]::ty::context::tls::enter_context::<<rustc_interface[a457c10140b33d94]::passes::QueryContext>::enter<<rustc_interface[a457c10140b33d94]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}, core[584d80f921d3c2b7]::result::Result<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
  62:     0x7f2b3c9dd69d - <rustc_interface[a457c10140b33d94]::passes::QueryContext>::enter::<<rustc_interface[a457c10140b33d94]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/passes.rs:765:9
  63:     0x7f2b3c96c2b4 - <rustc_interface[a457c10140b33d94]::queries::Queries>::ongoing_codegen::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/queries.rs:240:13
  64:     0x7f2b3c96c2b4 - <core[584d80f921d3c2b7]::option::Option<core[584d80f921d3c2b7]::result::Result<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>>::get_or_insert_with::<<rustc_interface[a457c10140b33d94]::queries::Queries>::ongoing_codegen::{closure#0}>
                               at /home/nilsh/projects/rust/library/core/src/option.rs:1590:49
  65:     0x7f2b3c96c2b4 - <rustc_interface[a457c10140b33d94]::queries::Query<alloc[d426fd0be2d16a6b]::boxed::Box<dyn core[584d80f921d3c2b7]::any::Any>>>::compute::<<rustc_interface[a457c10140b33d94]::queries::Queries>::ongoing_codegen::{closure#0}>
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/queries.rs:38:9
  66:     0x7f2b3c7f02f9 - rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}::{closure#2}
                               at /home/nilsh/projects/rust/compiler/rustc_driver/src/lib.rs:395:13
  67:     0x7f2b3c7f02f9 - <rustc_interface[a457c10140b33d94]::interface::Compiler>::enter::<rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}::{closure#2}, core[584d80f921d3c2b7]::result::Result<core[584d80f921d3c2b7]::option::Option<rustc_interface[a457c10140b33d94]::queries::Linker>, rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/queries.rs:380:19
  68:     0x7f2b3c813f26 - rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}
                               at /home/nilsh/projects/rust/compiler/rustc_driver/src/lib.rs:306:22
  69:     0x7f2b3c813f26 - rustc_interface[a457c10140b33d94]::interface::run_compiler::<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/interface.rs:327:21
  70:     0x7f2b3c813f26 - rustc_span[d50b082f3e833889]::with_source_map::<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
                               at /home/nilsh/projects/rust/compiler/rustc_span/src/lib.rs:1023:5
  71:     0x7f2b3c7ecb18 - rustc_interface[a457c10140b33d94]::interface::run_compiler::<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/interface.rs:321:13
  72:     0x7f2b3c7ecb18 - <scoped_tls[3f7a8f801319db86]::ScopedKey<rustc_span[d50b082f3e833889]::SessionGlobals>>::set::<rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>
                               at /home/nilsh/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  73:     0x7f2b3c7f419a - rustc_span[d50b082f3e833889]::create_session_globals_then::<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}>
                               at /home/nilsh/projects/rust/compiler/rustc_span/src/lib.rs:111:5
  74:     0x7f2b3c7f419a - rustc_interface[a457c10140b33d94]::util::run_in_thread_pool_with_globals::<rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}::{closure#0}
                               at /home/nilsh/projects/rust/compiler/rustc_interface/src/util.rs:145:38
  75:     0x7f2b3c7f419a - std[adc46dc960aa655]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a457c10140b33d94]::util::run_in_thread_pool_with_globals<rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/library/std/src/sys_common/backtrace.rs:121:18
  76:     0x7f2b3c7e500b - <std[adc46dc960aa655]::thread::Builder>::spawn_unchecked_::<rustc_interface[a457c10140b33d94]::util::run_in_thread_pool_with_globals<rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /home/nilsh/projects/rust/library/std/src/thread/mod.rs:550:17
  77:     0x7f2b3c7e500b - <core[584d80f921d3c2b7]::panic::unwind_safe::AssertUnwindSafe<<std[adc46dc960aa655]::thread::Builder>::spawn_unchecked_<rustc_interface[a457c10140b33d94]::util::run_in_thread_pool_with_globals<rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[584d80f921d3c2b7]::ops::function::FnOnce<()>>::call_once
                               at /home/nilsh/projects/rust/library/core/src/panic/unwind_safe.rs:271:9
  78:     0x7f2b3c7e500b - std[adc46dc960aa655]::panicking::try::do_call::<core[584d80f921d3c2b7]::panic::unwind_safe::AssertUnwindSafe<<std[adc46dc960aa655]::thread::Builder>::spawn_unchecked_<rustc_interface[a457c10140b33d94]::util::run_in_thread_pool_with_globals<rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:483:40
  79:     0x7f2b3c7e500b - std[adc46dc960aa655]::panicking::try::<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, core[584d80f921d3c2b7]::panic::unwind_safe::AssertUnwindSafe<<std[adc46dc960aa655]::thread::Builder>::spawn_unchecked_<rustc_interface[a457c10140b33d94]::util::run_in_thread_pool_with_globals<rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /home/nilsh/projects/rust/library/std/src/panicking.rs:447:19
  80:     0x7f2b3c7d3a7a - std[adc46dc960aa655]::panic::catch_unwind::<core[584d80f921d3c2b7]::panic::unwind_safe::AssertUnwindSafe<<std[adc46dc960aa655]::thread::Builder>::spawn_unchecked_<rustc_interface[a457c10140b33d94]::util::run_in_thread_pool_with_globals<rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>
                               at /home/nilsh/projects/rust/library/std/src/panic.rs:137:14
  81:     0x7f2b3c7d3a7a - <std[adc46dc960aa655]::thread::Builder>::spawn_unchecked_::<rustc_interface[a457c10140b33d94]::util::run_in_thread_pool_with_globals<rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#1}
                               at /home/nilsh/projects/rust/library/std/src/thread/mod.rs:549:30
  82:     0x7f2b3c7d3a7a - <<std[adc46dc960aa655]::thread::Builder>::spawn_unchecked_<rustc_interface[a457c10140b33d94]::util::run_in_thread_pool_with_globals<rustc_interface[a457c10140b33d94]::interface::run_compiler<core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>, rustc_driver[c0ebef4b5a350350]::run_compiler::{closure#1}>::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[584d80f921d3c2b7]::result::Result<(), rustc_errors[2fbccf98e0fb6daa]::ErrorGuaranteed>>::{closure#1} as core[584d80f921d3c2b7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /home/nilsh/projects/rust/library/core/src/ops/function.rs:422:5
  83:     0x7f2b3af90548 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3587e93147111652
                               at /home/nilsh/projects/rust/library/alloc/src/boxed.rs:1990:9
  84:     0x7f2b3af90548 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3f0a63b14d118cff
                               at /home/nilsh/projects/rust/library/alloc/src/boxed.rs:1990:9
  85:     0x7f2b3af7413c - std::sys::unix::thread::Thread::new::thread_start::hd1102045f43f18a5
                               at /home/nilsh/projects/rust/library/std/src/sys/unix/thread.rs:108:17
  86:     0x7f2b3ac86b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  87:     0x7f2b3ad18a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  88:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
error: aborting due to previous error
