plain
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-94287.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-94287.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-94287" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-94287/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/mir/mir-inlining/ice-issue-77306-2.rs stdout ----

error: test compilation failed although it shouldn't!
error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/mir-inlining/ice-issue-77306-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir-inlining/ice-issue-77306-2/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=3" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir-inlining/ice-issue-77306-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:198:90: Failed to normalize std::option::Option<(<&mut std::iter::FlatMap<Cursor, std::vec::IntoIter<TokenTree>, fn(TokenTree) -> impl std::iter::Iterator<Item = TokenTree> {tokenstream_probably_equal_for_proc_macro::break_tokens}> as std::iter::Iterator>::Item, <&mut std::iter::FlatMap<Cursor, std::vec::IntoIter<TokenTree>, fn(TokenTree) -> impl std::iter::Iterator<Item = TokenTree> {tokenstream_probably_equal_for_proc_macro::break_tokens}> as std::iter::Iterator>::Item)>, maybe try to call `try_normalize_erasing_regions` instead
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1458:9
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stack backtrace:
stack backtrace:
   0:     0x7f893cde352e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb8e8c92a942530a6
   1:     0x7f893ce4c308 - core::fmt::write::h25d20a7840d504f3
   2:     0x7f893cdd4271 - std::io::Write::write_fmt::h05aa27c8cff88f78
   3:     0x7f893cde64fe - std::panicking::default_hook::{{closure}}::he299c4e419620267
   4:     0x7f893cde61be - std::panicking::default_hook::h06afcb84e23c5637
   5:     0x7f893d7b6ba4 - rustc_driver[35c7d04f50e936a2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f893cde6cb1 - std::panicking::rust_panic_with_hook::hf35a0d8e2e365fbd
   7:     0x7f894030ab13 - std[5d078736eb5c7b3e]::panicking::begin_panic::<rustc_errors[ff7160f62e43015a]::ExplicitBug>::{closure#0}
   8:     0x7f89403086c6 - std[5d078736eb5c7b3e]::sys_common::backtrace::__rust_end_short_backtrace::<std[5d078736eb5c7b3e]::panicking::begin_panic<rustc_errors[ff7160f62e43015a]::ExplicitBug>::{closure#0}, !>
   9:     0x7f893d75dcc6 - std[5d078736eb5c7b3e]::panicking::begin_panic::<rustc_errors[ff7160f62e43015a]::ExplicitBug>
  10:     0x7f8940304736 - std[5d078736eb5c7b3e]::panic::panic_any::<rustc_errors[ff7160f62e43015a]::ExplicitBug>
  11:     0x7f89403029f9 - <rustc_errors[ff7160f62e43015a]::HandlerInner>::bug::<&alloc[60eb012fd5967a49]::string::String>
  12:     0x7f89403024d0 - <rustc_errors[ff7160f62e43015a]::Handler>::bug::<&alloc[60eb012fd5967a49]::string::String>
  13:     0x7f89403edc9e - rustc_middle[510ff57b9bd76ae3]::ty::context::tls::with_context_opt::<rustc_middle[510ff57b9bd76ae3]::ty::context::tls::with_opt<rustc_middle[510ff57b9bd76ae3]::util::bug::opt_span_bug_fmt<rustc_span[f9f6f74ced6a2c0a]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f89403f16e9 - rustc_middle[510ff57b9bd76ae3]::util::bug::opt_span_bug_fmt::<rustc_span[f9f6f74ced6a2c0a]::span_encoding::Span>
  15:     0x7f893d767c78 - rustc_middle[510ff57b9bd76ae3]::util::bug::bug_fmt
  16:     0x7f8940414a0e - <rustc_middle[510ff57b9bd76ae3]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::normalize_generic_arg_after_erasing_regions
  17:     0x7f8940414613 - <rustc_middle[510ff57b9bd76ae3]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle[510ff57b9bd76ae3]::ty::fold::FallibleTypeFolder>::try_fold_ty
  18:     0x7f89402a924b - <&rustc_middle[510ff57b9bd76ae3]::ty::list::List<rustc_middle[510ff57b9bd76ae3]::ty::Ty> as rustc_middle[510ff57b9bd76ae3]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[510ff57b9bd76ae3]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  19:     0x7f894028497d - <rustc_middle[510ff57b9bd76ae3]::ty::sty::Binder<rustc_middle[510ff57b9bd76ae3]::ty::sty::FnSig> as rustc_middle[510ff57b9bd76ae3]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle[510ff57b9bd76ae3]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  20:     0x7f89404145db - <rustc_middle[510ff57b9bd76ae3]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle[510ff57b9bd76ae3]::ty::fold::FallibleTypeFolder>::try_fold_binder::<rustc_middle[510ff57b9bd76ae3]::ty::sty::FnSig>
  21:     0x7f894028111b - <rustc_middle[510ff57b9bd76ae3]::ty::sty::Binder<rustc_middle[510ff57b9bd76ae3]::ty::sty::FnSig> as rustc_middle[510ff57b9bd76ae3]::ty::fold::TypeFoldable>::fold_with::<rustc_middle[510ff57b9bd76ae3]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  22:     0x7f894036ce06 - <rustc_middle[510ff57b9bd76ae3]::ty::context::TyCtxt>::normalize_erasing_regions::<rustc_middle[510ff57b9bd76ae3]::ty::sty::Binder<rustc_middle[510ff57b9bd76ae3]::ty::sty::FnSig>>
  23:     0x7f89402feca0 - <rustc_middle[510ff57b9bd76ae3]::ty::instance::Instance>::fn_sig_for_fn_abi
  24:     0x7f8940222d31 - rustc_middle[510ff57b9bd76ae3]::ty::layout::fn_abi_of_instance
  25:     0x7f893f4be502 - rustc_query_system[d955f77eb999be5f]::query::plumbing::get_query::<rustc_query_impl[3a998da5b31fd108]::queries::fn_abi_of_instance, rustc_query_impl[3a998da5b31fd108]::plumbing::QueryCtxt>
  26:     0x7f893f33e09c - <rustc_query_impl[3a998da5b31fd108]::Queries as rustc_middle[510ff57b9bd76ae3]::ty::query::QueryEngine>::fn_abi_of_instance
  27:     0x7f893d9c4857 - <rustc_codegen_llvm[4fc087869b205030]::context::CodegenCx as rustc_codegen_ssa[9bb0471a6b3d5f0e]::traits::declare::PreDefineMethods>::predefine_fn
  28:     0x7f893da8c273 - <rustc_middle[510ff57b9bd76ae3]::mir::mono::MonoItem as rustc_codegen_ssa[9bb0471a6b3d5f0e]::mono_item::MonoItemExt>::predefine::<rustc_codegen_llvm[4fc087869b205030]::builder::Builder>
  29:     0x7f893dad8d32 - rustc_codegen_llvm[4fc087869b205030]::base::compile_codegen_unit::module_codegen
  30:     0x7f893d9f0045 - <rustc_query_system[d955f77eb999be5f]::dep_graph::graph::DepGraph<rustc_middle[510ff57b9bd76ae3]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[510ff57b9bd76ae3]::ty::context::TyCtxt, rustc_span[f9f6f74ced6a2c0a]::symbol::Symbol, rustc_codegen_ssa[9bb0471a6b3d5f0e]::ModuleCodegen<rustc_codegen_llvm[4fc087869b205030]::ModuleLlvm>>
  31:     0x7f893dad89ee - rustc_codegen_llvm[4fc087869b205030]::base::compile_codegen_unit
  32:     0x7f893da3f2fa - rustc_codegen_ssa[9bb0471a6b3d5f0e]::base::codegen_crate::<rustc_codegen_llvm[4fc087869b205030]::LlvmCodegenBackend>
  33:     0x7f893db32324 - <rustc_codegen_llvm[4fc087869b205030]::LlvmCodegenBackend as rustc_codegen_ssa[9bb0471a6b3d5f0e]::traits::backend::CodegenBackend>::codegen_crate
  34:     0x7f893d9123ad - <rustc_interface[3fe4102037289fb4]::passes::QueryContext>::enter::<<rustc_interface[3fe4102037289fb4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[e6e031e6c279338a]::result::Result<alloc[60eb012fd5967a49]::boxed::Box<dyn core[e6e031e6c279338a]::any::Any>, rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>>
  35:     0x7f893d8e956d - <rustc_interface[3fe4102037289fb4]::queries::Queries>::ongoing_codegen
  36:     0x7f893d7baa99 - rustc_interface[3fe4102037289fb4]::interface::create_compiler_and_run::<core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>, rustc_driver[35c7d04f50e936a2]::run_compiler::{closure#1}>
  37:     0x7f893d83148f - <scoped_tls[ca9491c2b16e3962]::ScopedKey<rustc_span[f9f6f74ced6a2c0a]::SessionGlobals>>::set::<rustc_interface[3fe4102037289fb4]::interface::run_compiler<core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>, rustc_driver[35c7d04f50e936a2]::run_compiler::{closure#1}>::{closure#0}, core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>>
  38:     0x7f893d81c54f - std[5d078736eb5c7b3e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3fe4102037289fb4]::util::run_in_thread_pool_with_globals<rustc_interface[3fe4102037289fb4]::interface::run_compiler<core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>, rustc_driver[35c7d04f50e936a2]::run_compiler::{closure#1}>::{closure#0}, core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>>::{closure#0}, core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>>
  39:     0x7f893d7bc021 - std[5d078736eb5c7b3e]::panic::catch_unwind::<core[e6e031e6c279338a]::panic::unwind_safe::AssertUnwindSafe<<std[5d078736eb5c7b3e]::thread::Builder>::spawn_unchecked_<rustc_interface[3fe4102037289fb4]::util::run_in_thread_pool_with_globals<rustc_interface[3fe4102037289fb4]::interface::run_compiler<core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>, rustc_driver[35c7d04f50e936a2]::run_compiler::{closure#1}>::{closure#0}, core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>>::{closure#0}, core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>>
  40:     0x7f893d81ef20 - <<std[5d078736eb5c7b3e]::thread::Builder>::spawn_unchecked_<rustc_interface[3fe4102037289fb4]::util::run_in_thread_pool_with_globals<rustc_interface[3fe4102037289fb4]::interface::run_compiler<core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>, rustc_driver[35c7d04f50e936a2]::run_compiler::{closure#1}>::{closure#0}, core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>>::{closure#0}, core[e6e031e6c279338a]::result::Result<(), rustc_errors[ff7160f62e43015a]::ErrorGuaranteed>>::{closure#1} as core[e6e031e6c279338a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f893cdf3aa5 - std::sys::unix::thread::Thread::new::thread_start::h6379701915743f1b
  42:     0x7f893cb8eb43 - <unknown>
  43:     0x7f893cc20a00 - <unknown>
  44:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (bc75012c4 2022-09-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=3
query stack during panic:
query stack during panic:
#0 [fn_abi_of_instance] computing call ABI of `<core::iter::adapters::zip::Zip<&mut core::iter::adapters::flatten::FlatMap<Cursor, alloc::vec::into_iter::IntoIter<TokenTree>, fn(TokenTree) -> impl core::iter::traits::iterator::Iterator<Item = TokenTree> {tokenstream_probably_equal_for_proc_macro::break_tokens}>, &mut core::iter::adapters::flatten::FlatMap<Cursor, alloc::vec::into_iter::IntoIter<TokenTree>, fn(TokenTree) -> impl core::iter::traits::iterator::Iterator<Item = TokenTree> {tokenstream_probably_equal_for_proc_macro::break_tokens}>> as core::iter::adapters::zip::ZipImpl<&mut core::iter::adapters::flatten::FlatMap<Cursor, alloc::vec::into_iter::IntoIter<TokenTree>, fn(TokenTree) -> impl core::iter::traits::iterator::Iterator<Item = TokenTree> {tokenstream_probably_equal_for_proc_macro::break_tokens}>, &mut core::iter::adapters::flatten::FlatMap<Cursor, alloc::vec::into_iter::IntoIter<TokenTree>, fn(TokenTree) -> impl core::iter::traits::iterator::Iterator<Item = TokenTree> {tokenstream_probably_equal_for_proc_macro::break_tokens}>>>::next`
error: aborting due to previous error
------------------------------------------


