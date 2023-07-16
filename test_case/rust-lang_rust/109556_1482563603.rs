plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:917222de331afc95ef8d3a6300048017039b2b08)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `TyAndLayout { ty: for<'a, 'b> fn(&'a mut rustc_expand::base::ExtCtxt<'b>, rustc_span::Span, rustc_ast::tokenstream::TokenStream) -> std::boxed::Box<(dyn rustc_expand::base::MacResult + 'a)>, layout: Layout { size: Size(8 bytes), align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, abi: Scalar(Initialized { value: Pointer(AddressSpace(0)), valid_range: 1..=18446744073709551615 }), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: Pointer(AddressSpace(0)), valid_range: 1..=18446744073709551615 }), variants: Single { index: 0 } } }`,
 right: `TyAndLayout { ty: for<'a, 'b> fn(&'a mut rustc_expand::base::ExtCtxt<'b>, rustc_span::Span, rustc_ast::tokenstream::TokenStream) -> std::boxed::Box<dyn rustc_expand::base::MacResult>, layout: Layout { size: Size(8 bytes), align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, abi: Scalar(Initialized { value: Pointer(AddressSpace(0)), valid_range: 1..=18446744073709551615 }), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: Pointer(AddressSpace(0)), valid_range: 1..=18446744073709551615 }), variants: Single { index: 0 } } }`', /checkout/compiler/rustc_codegen_ssa/src/mir/rvalue.rs:232:9
stack backtrace:
   0:     0x7f57b4e29425 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h58f3d5f9613fc903
   1:     0x7f57b4e95da8 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f57b4e1d731 - std::io::Write::write_fmt::h46e9ff7509b654e1
   3:     0x7f57b4e29235 - std::sys_common::backtrace::print::h0b48412da775984a
   4:     0x7f57b4e2c424 - std::panicking::default_hook::{{closure}}::h4eaef8437e4e66b7
   5:     0x7f57b4e2c112 - std::panicking::default_hook::hef252b4dd17d3bdf
   6:     0x7f57b589e645 - rustc_driver_impl[5ab6970a15cdd324]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f57b4e2cb59 - std::panicking::rust_panic_with_hook::hff848c0b52cb878e
   8:     0x7f57b4e2c8d9 - std::panicking::begin_panic_handler::{{closure}}::h35a687b7fe1e45d3
   9:     0x7f57b4e298d6 - std::sys_common::backtrace::__rust_end_short_backtrace::h582581d5cc1315f5
  10:     0x7f57b4e2c5a2 - rust_begin_unwind
  11:     0x7f57b4de60b3 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7f57b4de643f - core::panicking::assert_failed_inner::he3c123340f62e981
  13:     0x7f57b565e1cb - core[54ab13d2a06817e1]::panicking::assert_failed::<rustc_target[58f23de92af80a5f]::abi::TyAndLayout<rustc_middle[ef4d301d6aa1f8df]::ty::Ty>, rustc_target[58f23de92af80a5f]::abi::TyAndLayout<rustc_middle[ef4d301d6aa1f8df]::ty::Ty>>
  14:     0x7f57b5ac4c2c - <rustc_codegen_ssa[d284098dc66eb442]::mir::FunctionCx<rustc_codegen_llvm[ad916b057f2f8375]::builder::Builder>>::codegen_rvalue_operand
  15:     0x7f57b5abe69f - rustc_codegen_ssa[d284098dc66eb442]::mir::codegen_mir::<rustc_codegen_llvm[ad916b057f2f8375]::builder::Builder>
  16:     0x7f57b5b9525b - rustc_codegen_ssa[d284098dc66eb442]::base::codegen_instance::<rustc_codegen_llvm[ad916b057f2f8375]::builder::Builder>
  17:     0x7f57b5bdd21f - <rustc_middle[ef4d301d6aa1f8df]::mir::mono::MonoItem as rustc_codegen_ssa[d284098dc66eb442]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[ad916b057f2f8375]::builder::Builder>
  18:     0x7f57b5ab5d30 - rustc_codegen_llvm[ad916b057f2f8375]::base::compile_codegen_unit::module_codegen
  19:     0x7f57b5ab5130 - rustc_codegen_llvm[ad916b057f2f8375]::base::compile_codegen_unit
  20:     0x7f57b5b946d5 - rustc_codegen_ssa[d284098dc66eb442]::base::codegen_crate::<rustc_codegen_llvm[ad916b057f2f8375]::LlvmCodegenBackend>
  21:     0x7f57b5b55506 - <rustc_codegen_llvm[ad916b057f2f8375]::LlvmCodegenBackend as rustc_codegen_ssa[d284098dc66eb442]::traits::backend::CodegenBackend>::codegen_crate
  22:     0x7f57b59ddb3f - <rustc_session[9db97d1686892f5f]::session::Session>::time::<alloc[5a8e793bc0d22b6e]::boxed::Box<dyn core[54ab13d2a06817e1]::any::Any>, rustc_interface[ea4bed2aa28b4af0]::passes::start_codegen::{closure#0}>
  23:     0x7f57b5992981 - rustc_interface[ea4bed2aa28b4af0]::passes::start_codegen
  24:     0x7f57b599822a - <rustc_middle[ef4d301d6aa1f8df]::ty::context::GlobalCtxt>::enter::<<rustc_interface[ea4bed2aa28b4af0]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<alloc[5a8e793bc0d22b6e]::boxed::Box<dyn core[54ab13d2a06817e1]::any::Any>, rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>
  25:     0x7f57b5a7c424 - <rustc_interface[ea4bed2aa28b4af0]::queries::Queries>::ongoing_codegen
  26:     0x7f57b58ed1bf - <rustc_interface[ea4bed2aa28b4af0]::interface::Compiler>::enter::<rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}::{closure#2}, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::option::Option<rustc_interface[ea4bed2aa28b4af0]::queries::Linker>, rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>
  27:     0x7f57b592f640 - rustc_span[7f1d9ddd6ed7818e]::with_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_interface[ea4bed2aa28b4af0]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  28:     0x7f57b58bd7e9 - std[e292dd3bffb96032]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ea4bed2aa28b4af0]::util::run_in_thread_pool_with_globals<rustc_interface[ea4bed2aa28b4af0]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>
  29:     0x7f57b58b0fb3 - <<std[e292dd3bffb96032]::thread::Builder>::spawn_unchecked_<rustc_interface[ea4bed2aa28b4af0]::util::run_in_thread_pool_with_globals<rustc_interface[ea4bed2aa28b4af0]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7f57b4e38ede - std::sys::unix::thread::Thread::new::thread_start::hb98a15cf8f2a3509
  31:     0x7f57b4bd6b43 - <unknown>
  32:     0x7f57b4c68a00 - <unknown>
  33:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (9536b266a 2023-03-24) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `rustc_builtin_macros`
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `TyAndLayout { ty: fn(&&rustc_query_system::dep_graph::DepNode<dep_graph::dep_node::DepKind>, &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>, layout: Layout { size: Size(8 bytes), align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, abi: Scalar(Initialized { value: Pointer(AddressSpace(0)), valid_range: 1..=18446744073709551615 }), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: Pointer(AddressSpace(0)), valid_range: 1..=18446744073709551615 }), variants: Single { index: 0 } } }`,
 right: `TyAndLayout { ty: for<'a, 'b, 'c> fn(&'a &rustc_query_system::dep_graph::DepNode<dep_graph::dep_node::DepKind>, &'b mut std::fmt::Formatter<'c>) -> std::result::Result<(), std::fmt::Error>, layout: Layout { size: Size(8 bytes), align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, abi: Scalar(Initialized { value: Pointer(AddressSpace(0)), valid_range: 1..=18446744073709551615 }), fields: Primitive, largest_niche: Some(Niche { offset: Size(0 bytes), value: Pointer(AddressSpace(0)), valid_range: 1..=18446744073709551615 }), variants: Single { index: 0 } } }`', /checkout/compiler/rustc_codegen_ssa/src/mir/rvalue.rs:232:9
stack backtrace:
   0:     0x7fc4329d3425 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h58f3d5f9613fc903
   1:     0x7fc432a3fda8 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7fc4329c7731 - std::io::Write::write_fmt::h46e9ff7509b654e1
   3:     0x7fc4329d3235 - std::sys_common::backtrace::print::h0b48412da775984a
   4:     0x7fc4329d6424 - std::panicking::default_hook::{{closure}}::h4eaef8437e4e66b7
   5:     0x7fc4329d6112 - std::panicking::default_hook::hef252b4dd17d3bdf
   6:     0x7fc433448645 - rustc_driver_impl[5ab6970a15cdd324]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc4329d6b59 - std::panicking::rust_panic_with_hook::hff848c0b52cb878e
   8:     0x7fc4329d68d9 - std::panicking::begin_panic_handler::{{closure}}::h35a687b7fe1e45d3
   9:     0x7fc4329d38d6 - std::sys_common::backtrace::__rust_end_short_backtrace::h582581d5cc1315f5
  10:     0x7fc4329d65a2 - rust_begin_unwind
  11:     0x7fc4329900b3 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7fc43299043f - core::panicking::assert_failed_inner::he3c123340f62e981
  13:     0x7fc4332081cb - core[54ab13d2a06817e1]::panicking::assert_failed::<rustc_target[58f23de92af80a5f]::abi::TyAndLayout<rustc_middle[ef4d301d6aa1f8df]::ty::Ty>, rustc_target[58f23de92af80a5f]::abi::TyAndLayout<rustc_middle[ef4d301d6aa1f8df]::ty::Ty>>
  14:     0x7fc43366ec2c - <rustc_codegen_ssa[d284098dc66eb442]::mir::FunctionCx<rustc_codegen_llvm[ad916b057f2f8375]::builder::Builder>>::codegen_rvalue_operand
  15:     0x7fc43366869f - rustc_codegen_ssa[d284098dc66eb442]::mir::codegen_mir::<rustc_codegen_llvm[ad916b057f2f8375]::builder::Builder>
  16:     0x7fc43373f25b - rustc_codegen_ssa[d284098dc66eb442]::base::codegen_instance::<rustc_codegen_llvm[ad916b057f2f8375]::builder::Builder>
  17:     0x7fc43378721f - <rustc_middle[ef4d301d6aa1f8df]::mir::mono::MonoItem as rustc_codegen_ssa[d284098dc66eb442]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[ad916b057f2f8375]::builder::Builder>
  18:     0x7fc43365fd30 - rustc_codegen_llvm[ad916b057f2f8375]::base::compile_codegen_unit::module_codegen
  19:     0x7fc43365f130 - rustc_codegen_llvm[ad916b057f2f8375]::base::compile_codegen_unit
  20:     0x7fc43373e6d5 - rustc_codegen_ssa[d284098dc66eb442]::base::codegen_crate::<rustc_codegen_llvm[ad916b057f2f8375]::LlvmCodegenBackend>
  21:     0x7fc4336ff506 - <rustc_codegen_llvm[ad916b057f2f8375]::LlvmCodegenBackend as rustc_codegen_ssa[d284098dc66eb442]::traits::backend::CodegenBackend>::codegen_crate
  22:     0x7fc433587b3f - <rustc_session[9db97d1686892f5f]::session::Session>::time::<alloc[5a8e793bc0d22b6e]::boxed::Box<dyn core[54ab13d2a06817e1]::any::Any>, rustc_interface[ea4bed2aa28b4af0]::passes::start_codegen::{closure#0}>
  23:     0x7fc43353c981 - rustc_interface[ea4bed2aa28b4af0]::passes::start_codegen
  24:     0x7fc43354222a - <rustc_middle[ef4d301d6aa1f8df]::ty::context::GlobalCtxt>::enter::<<rustc_interface[ea4bed2aa28b4af0]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<alloc[5a8e793bc0d22b6e]::boxed::Box<dyn core[54ab13d2a06817e1]::any::Any>, rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>
  25:     0x7fc433626424 - <rustc_interface[ea4bed2aa28b4af0]::queries::Queries>::ongoing_codegen
  26:     0x7fc4334971bf - <rustc_interface[ea4bed2aa28b4af0]::interface::Compiler>::enter::<rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}::{closure#2}, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::option::Option<rustc_interface[ea4bed2aa28b4af0]::queries::Linker>, rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>
  27:     0x7fc4334d9640 - rustc_span[7f1d9ddd6ed7818e]::with_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_interface[ea4bed2aa28b4af0]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  28:     0x7fc4334677e9 - std[e292dd3bffb96032]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ea4bed2aa28b4af0]::util::run_in_thread_pool_with_globals<rustc_interface[ea4bed2aa28b4af0]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>
  29:     0x7fc43345afb3 - <<std[e292dd3bffb96032]::thread::Builder>::spawn_unchecked_<rustc_interface[ea4bed2aa28b4af0]::util::run_in_thread_pool_with_globals<rustc_interface[ea4bed2aa28b4af0]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7fc4329e2ede - std::sys::unix::thread::Thread::new::thread_start::hb98a15cf8f2a3509
  31:     0x7fc432780b43 - <unknown>
  32:     0x7fc432812a00 - <unknown>
  33:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (9536b266a 2023-03-24) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
