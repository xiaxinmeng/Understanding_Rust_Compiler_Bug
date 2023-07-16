
warning: variants `A` and `B` are never constructed
 --> align.rs:2:18
  |
2 |     enum Error { A, B }
  |          -----   ^  ^
  |          |
  |          variants in this enum
  |
  = note: `#[warn(dead_code)]` on by default

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Align(8 bytes)`,
 right: `Align(1 bytes)`: alignment mismatch between ABI and layout in Layout {
    fields: Arbitrary {
        offsets: [
            Size(0 bytes),
        ],
        memory_index: [
            0,
        ],
    },
    variants: Multiple {
        tag: Initialized {
            value: Int(
                I8,
                false,
            ),
            valid_range: 0..=2,
        },
        tag_encoding: Niche {
            dataful_variant: 1,
            niche_variants: 0..=0,
            niche_start: 2,
        },
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [
                        Size(0 bytes),
                    ],
                    memory_index: [
                        0,
                    ],
                },
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align(8 bytes),
                    pref: Align(8 bytes),
                },
                size: Size(0 bytes),
            },
            Layout {
                fields: Arbitrary {
                    offsets: [
                        Size(0 bytes),
                    ],
                    memory_index: [
                        0,
                    ],
                },
                variants: Single {
                    index: 1,
                },
                abi: Scalar(
                    Initialized {
                        value: Int(
                            I8,
                            false,
                        ),
                        valid_range: 0..=1,
                    },
                ),
                largest_niche: Some(
                    Niche {
                        offset: Size(0 bytes),
                        value: Int(
                            I8,
                            false,
                        ),
                        valid_range: 0..=1,
                    },
                ),
                align: AbiAndPrefAlign {
                    abi: Align(1 bytes),
                    pref: Align(8 bytes),
                },
                size: Size(1 bytes),
            },
        ],
    },
    abi: Scalar(
        Initialized {
            value: Int(
                I8,
                false,
            ),
            valid_range: 0..=2,
        },
    ),
    largest_niche: Some(
        Niche {
            offset: Size(0 bytes),
            value: Int(
                I8,
                false,
            ),
            valid_range: 0..=2,
        },
    ),
    align: AbiAndPrefAlign {
        abi: Align(8 bytes),
        pref: Align(8 bytes),
    },
    size: Size(1 bytes),
}', compiler/rustc_middle/src/ty/layout.rs:240:21
stack backtrace:
   0:     0x7ff8704d57b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h57183f18665a904e
   1:     0x7ff870542308 - core::fmt::write::h1fc3741f5707a7e9
   2:     0x7ff8704d88e1 - std::io::Write::write_fmt::h5e582d931bc0e652
   3:     0x7ff8704d5609 - std::sys_common::backtrace::print::h915370a8cb158d0f
   4:     0x7ff8704c2914 - std::panicking::default_hook::{{closure}}::hd0ff95d9e670b00d
   5:     0x7ff8704c26c3 - std::panicking::default_hook::h1a88faa49859dbbf
   6:     0x7ff870f9f514 - rustc_driver[ba7942d2ed61c3a8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff8704c2d4f - std::panicking::rust_panic_with_hook::h216d9410c99eb34c
   8:     0x7ff870498a37 - std::panicking::begin_panic_handler::{{closure}}::h7a2b559b1f7ec0ab
   9:     0x7ff870498974 - std::sys_common::backtrace::__rust_end_short_backtrace::h94cd04d25d25882a
  10:     0x7ff8704c2982 - rust_begin_unwind
  11:     0x7ff870493663 - core::panicking::panic_fmt::hb503280b62085f97
  12:     0x7ff870532a6e - core::panicking::assert_failed_inner::h2c15a679abd16964
  13:     0x7ff870ed41ab - core[6398db64b51061bb]::panicking::assert_failed::<rustc_target[e3ac180738d9092e]::abi::Align, rustc_target[e3ac180738d9092e]::abi::Align>
  14:     0x7ff8735c1c53 - rustc_middle[f8389151b4735317]::ty::layout::sanity_check_layout::check_layout_abi
  15:     0x7ff8735d049e - rustc_middle[f8389151b4735317]::ty::context::tls::with_context::<rustc_middle[f8389151b4735317]::ty::context::tls::with_related_context<rustc_middle[f8389151b4735317]::ty::layout::layout_of::{closure#0}, core[6398db64b51061bb]::result::Result<rustc_target[e3ac180738d9092e]::abi::TyAndLayout<rustc_middle[f8389151b4735317]::ty::Ty>, rustc_middle[f8389151b4735317]::ty::layout::LayoutError>>::{closure#0}, core[6398db64b51061bb]::result::Result<rustc_target[e3ac180738d9092e]::abi::TyAndLayout<rustc_middle[f8389151b4735317]::ty::Ty>, rustc_middle[f8389151b4735317]::ty::layout::LayoutError>>::{closure#0}
  16:     0x7ff8735d1e5b - rustc_middle[f8389151b4735317]::ty::layout::layout_of
  17:     0x7ff8727c9568 - rustc_query_system[b1d5cf42652e4b5a]::query::plumbing::get_query::<rustc_query_impl[d9de55591e7c271a]::queries::layout_of, rustc_query_impl[d9de55591e7c271a]::plumbing::QueryCtxt>
  18:     0x7ff8725f02a3 - <rustc_query_impl[d9de55591e7c271a]::Queries as rustc_middle[f8389151b4735317]::ty::query::QueryEngine>::layout_of
  19:     0x7ff87121315a - <rustc_codegen_llvm[4c68097bd99075c]::context::CodegenCx as rustc_middle[f8389151b4735317]::ty::layout::LayoutOf>::spanned_layout_of
  20:     0x7ff871200dee - <rustc_codegen_ssa[8ac719cfe9297717]::mir::FunctionCx<rustc_codegen_llvm[4c68097bd99075c]::builder::Builder>>::codegen_rvalue_operand
  21:     0x7ff8712076d4 - <rustc_codegen_ssa[8ac719cfe9297717]::mir::FunctionCx<rustc_codegen_llvm[4c68097bd99075c]::builder::Builder>>::codegen_block
  22:     0x7ff8711fc976 - rustc_codegen_ssa[8ac719cfe9297717]::mir::codegen_mir::<rustc_codegen_llvm[4c68097bd99075c]::builder::Builder>
  23:     0x7ff87121e776 - rustc_codegen_ssa[8ac719cfe9297717]::base::codegen_instance::<rustc_codegen_llvm[4c68097bd99075c]::builder::Builder>
  24:     0x7ff8712145a9 - <rustc_middle[f8389151b4735317]::mir::mono::MonoItem as rustc_codegen_ssa[8ac719cfe9297717]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[4c68097bd99075c]::builder::Builder>
  25:     0x7ff8711cfec4 - rustc_codegen_llvm[4c68097bd99075c]::base::compile_codegen_unit::module_codegen
  26:     0x7ff8711eb404 - <rustc_query_system[b1d5cf42652e4b5a]::dep_graph::graph::DepGraph<rustc_middle[f8389151b4735317]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[f8389151b4735317]::ty::context::TyCtxt, rustc_span[e49f3a209ef89a16]::symbol::Symbol, rustc_codegen_ssa[8ac719cfe9297717]::ModuleCodegen<rustc_codegen_llvm[4c68097bd99075c]::ModuleLlvm>>
  27:     0x7ff8711cfaf7 - rustc_codegen_llvm[4c68097bd99075c]::base::compile_codegen_unit
  28:     0x7ff87121da9f - rustc_codegen_ssa[8ac719cfe9297717]::base::codegen_crate::<rustc_codegen_llvm[4c68097bd99075c]::LlvmCodegenBackend>
  29:     0x7ff8711a14fd - <rustc_codegen_llvm[4c68097bd99075c]::LlvmCodegenBackend as rustc_codegen_ssa[8ac719cfe9297717]::traits::backend::CodegenBackend>::codegen_crate
  30:     0x7ff87101bbdb - <rustc_session[f601aaa54490cdf2]::session::Session>::time::<alloc[ebca99b57d59156]::boxed::Box<dyn core[6398db64b51061bb]::any::Any>, rustc_interface[72861cfa91fc646]::passes::start_codegen::{closure#0}>
  31:     0x7ff8710b3fcf - rustc_interface[72861cfa91fc646]::passes::start_codegen
  32:     0x7ff87108b96d - <rustc_interface[72861cfa91fc646]::passes::QueryContext>::enter::<<rustc_interface[72861cfa91fc646]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6398db64b51061bb]::result::Result<alloc[ebca99b57d59156]::boxed::Box<dyn core[6398db64b51061bb]::any::Any>, rustc_errors[685f6944170c16d8]::ErrorGuaranteed>>
  33:     0x7ff87107da4e - <rustc_interface[72861cfa91fc646]::queries::Query<alloc[ebca99b57d59156]::boxed::Box<dyn core[6398db64b51061bb]::any::Any>>>::compute::<<rustc_interface[72861cfa91fc646]::queries::Queries>::ongoing_codegen::{closure#0}>
  34:     0x7ff870f3be4c - <rustc_interface[72861cfa91fc646]::interface::Compiler>::enter::<rustc_driver[ba7942d2ed61c3a8]::run_compiler::{closure#1}::{closure#2}, core[6398db64b51061bb]::result::Result<core[6398db64b51061bb]::option::Option<rustc_interface[72861cfa91fc646]::queries::Linker>, rustc_errors[685f6944170c16d8]::ErrorGuaranteed>>
  35:     0x7ff870f1d687 - rustc_span[e49f3a209ef89a16]::with_source_map::<core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>, rustc_interface[72861cfa91fc646]::interface::create_compiler_and_run<core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>, rustc_driver[ba7942d2ed61c3a8]::run_compiler::{closure#1}>::{closure#1}>
  36:     0x7ff870f3cd15 - rustc_interface[72861cfa91fc646]::interface::create_compiler_and_run::<core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>, rustc_driver[ba7942d2ed61c3a8]::run_compiler::{closure#1}>
  37:     0x7ff870f16f1f - <scoped_tls[e6a8abd170e2c0fc]::ScopedKey<rustc_span[e49f3a209ef89a16]::SessionGlobals>>::set::<rustc_interface[72861cfa91fc646]::interface::run_compiler<core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>, rustc_driver[ba7942d2ed61c3a8]::run_compiler::{closure#1}>::{closure#0}, core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>>
  38:     0x7ff870f90719 - std[a7e9441a3df41b71]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[72861cfa91fc646]::util::run_in_thread_pool_with_globals<rustc_interface[72861cfa91fc646]::interface::run_compiler<core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>, rustc_driver[ba7942d2ed61c3a8]::run_compiler::{closure#1}>::{closure#0}, core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>>::{closure#0}, core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>>
  39:     0x7ff870f1e231 - std[a7e9441a3df41b71]::panicking::try::<core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>, core[6398db64b51061bb]::panic::unwind_safe::AssertUnwindSafe<<std[a7e9441a3df41b71]::thread::Builder>::spawn_unchecked_<rustc_interface[72861cfa91fc646]::util::run_in_thread_pool_with_globals<rustc_interface[72861cfa91fc646]::interface::run_compiler<core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>, rustc_driver[ba7942d2ed61c3a8]::run_compiler::{closure#1}>::{closure#0}, core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>>::{closure#0}, core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7ff870f3547e - <<std[a7e9441a3df41b71]::thread::Builder>::spawn_unchecked_<rustc_interface[72861cfa91fc646]::util::run_in_thread_pool_with_globals<rustc_interface[72861cfa91fc646]::interface::run_compiler<core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>, rustc_driver[ba7942d2ed61c3a8]::run_compiler::{closure#1}>::{closure#0}, core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>>::{closure#0}, core[6398db64b51061bb]::result::Result<(), rustc_errors[685f6944170c16d8]::ErrorGuaranteed>>::{closure#1} as core[6398db64b51061bb]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7ff8704efd08 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h2a92cd1facb82b24
  42:     0x7ff8704d3e67 - std::sys::unix::thread::Thread::new::thread_start::h7ad153c748885102
  43:     0x7ff870293e2d - start_thread
  44:     0x7ff8703191b0 - __clone3
  45:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [layout_of] computing layout of `core::result::Result<[usize; 0], main::Error>`
end of query stack
warning: 1 warning emitted

