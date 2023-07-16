
 λ  x test
Updating only changed submodules
  Submodules updated in 0.12 seconds
Building rustbuild
   Compiling bootstrap v0.0.0 (G:\Users\Chase\Code\Rust\rust\src\bootstrap)
    Finished dev [unoptimized] target(s) in 8.72s
Building stage0 tool tidy (x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.36s
tidy check
* 630 error codes
* highest error code: E0788
Checking which error codes lack tests...
* 378 features
Found 505 error codes
Found 0 error(s) in error codes
Done!
fmt check
Building stage0 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.39s
Copying stage0 std from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Building stage0 compiler artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
   Compiling rustc_llvm v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_llvm)
   Compiling rustc_middle v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_middle)
   Compiling rustc_infer v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_infer)
   Compiling rustc_metadata v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_metadata)
   Compiling rustc_mir_dataflow v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_mir_dataflow)
   Compiling rustc_symbol_mangling v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_symbol_mangling)
   Compiling rustc_incremental v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_incremental)
   Compiling rustc_query_impl v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_query_impl)
   Compiling rustc_monomorphize v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_monomorphize)
   Compiling rustc_passes v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_passes)
   Compiling rustc_save_analysis v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_save_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_trait_selection)
   Compiling rustc_lint v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_lint)
   Compiling rustc_const_eval v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_const_eval)
   Compiling rustc_ty_utils v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_ty_utils)
   Compiling rustc_traits v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_traits)
   Compiling rustc_mir_build v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_mir_build)
   Compiling rustc_codegen_llvm v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_codegen_llvm)
   Compiling rustc_typeck v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_typeck)
   Compiling rustc_plugin_impl v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_plugin_impl)
   Compiling rustc_mir_transform v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_mir_transform)
   Compiling rustc_borrowck v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_borrowck)
   Compiling rustc_privacy v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_privacy)
   Compiling rustc_interface v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_interface)
   Compiling rustc_driver v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc_driver)
   Compiling rustc-main v0.0.0 (G:\Users\Chase\Code\Rust\rust\compiler\rustc)
    Finished release [optimized] target(s) in 5m 05s
Copying stage0 rustc from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Assembling stage1 compiler (x86_64-pc-windows-msvc)
Building stage1 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
   Compiling compiler_builtins v0.1.73
   Compiling core v0.0.0 (G:\Users\Chase\Code\Rust\rust\library\core)
   Compiling libc v0.2.126
   Compiling cc v1.0.69
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (G:\Users\Chase\Code\Rust\rust\library\std)
   Compiling unwind v0.0.0 (G:\Users\Chase\Code\Rust\rust\library\unwind)
   Compiling rustc-std-workspace-core v1.99.0 (G:\Users\Chase\Code\Rust\rust\library\rustc-std-workspace-core)
   Compiling alloc v0.0.0 (G:\Users\Chase\Code\Rust\rust\library\alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error: internal compiler error: compiler\rustc_codegen_llvm\src\type_of.rs:146:13: layout: TyAndLayout {
                                    ty: rustc_std_workspace_core::result::Result<usize, rustc_std_workspace_core::fmt::Error>,
                                    layout: Layout {
                                        fields: Arbitrary {
                                            offsets: [
                                                Size(8 bytes),
                                            ],
                                            memory_index: [
                                                0,
                                            ],
                                        },
                                        variants: Single {
                                            index: 1,
                                        },
                                        abi: Aggregate {
                                            sized: true,
                                        },
                                        largest_niche: None,
                                        align: AbiAndPrefAlign {
                                            abi: Align(1 bytes),
                                            pref: Align(8 bytes),
                                        },
                                        size: Size(2 bytes),
                                    },
                                } stride: Size(2 bytes) offset: Size(8 bytes)

thread 'rustc' panicked at 'Box<dyn Any>', G:\Users\Chase\Code\Rust\rust\compiler\rustc_errors\src\lib.rs:1334:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: <rustc_errors::Handler>::span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
   2: <rustc_errors::Handler>::span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
   3: <rustc_errors::Handler>::bug::<&alloc::string::String>
   4: <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_binder::<rustc_middle::ty::sty::FnSig>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_codegen_llvm::type_of::struct_llfields
   8: <rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty> as rustc_codegen_llvm::type_of::LayoutLlvmExt>::llvm_type
   9: <rustc_codegen_ssa::mir::place::PlaceRef<&rustc_codegen_llvm::llvm_::ffi::Value>>::project_downcast::<rustc_codegen_llvm::builder::Builder>
  10: <rustc_codegen_ssa::mir::FunctionCx<rustc_codegen_llvm::builder::Builder>>::codegen_place
  11: rustc_codegen_ssa::mir::codegen_mir::<rustc_codegen_llvm::builder::Builder>
  12: rustc_codegen_ssa::base::codegen_instance::<rustc_codegen_llvm::builder::Builder>
  13: <rustc_middle::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define::<rustc_codegen_llvm::builder::Builder>
  14: rustc_codegen_llvm::base::compile_codegen_unit
  15: RINvMs1_NtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph5graphINtB6_8DepGraphNtNtNtCscInEZjIJEzE_12rustc_middle9dep_graph8dep_node7DepKindE9with_taskNtNtNtB1j_2ty7context6TyCtxtNtNtCs2vb4bKWGfvL_10rustc_span6symbol6SymbolINtCs4wqBg0PzBnM_17rustc_codegen_s
  16: rustc_codegen_llvm::base::compile_codegen_unit
  17: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
  18: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  19: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  20: rustc_interface::passes::start_codegen
  21: RINvMs_NtCs6lxe6u1d1HN_15rustc_interface6passesNtB5_12QueryContext5enterNCNCNvMs0_NtB7_7queriesNtB1i_7Queries15ongoing_codegen00INtNtCs9qroIgEvBu5_4core6result6ResultINtNtCs4fCcauHRDzO_5alloc5boxed3BoxDNtNtB27_3any3AnyEL_ENtCslxr0cr2Vr2Y_12rustc_errors15E
  22: <rustc_interface::queries::Query<alloc::boxed::Box<dyn core::any::Any>>>::compute::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}>
  23: RINvMs2_NtCs6lxe6u1d1HN_15rustc_interface7queriesNtNtB8_9interface8Compiler5enterNCNCNvCsg2BpsMvkavS_12rustc_driver12run_compilers_0s0_0INtNtCs9qroIgEvBu5_4core6result6ResultINtNtB2f_6option6OptionNtB6_6LinkerENtCslxr0cr2Vr2Y_12rustc_errors15ErrorGuarante
  24: RINvCs2vb4bKWGfvL_10rustc_span15with_source_mapINtNtCs9qroIgEvBu5_4core6result6ResultuNtCslxr0cr2Vr2Y_12rustc_errors15ErrorGuaranteedENCINvNtCs6lxe6u1d1HN_15rustc_interface9interface23create_compiler_and_runBJ_NCNvCsg2BpsMvkavS_12rustc_driver12run_compile
  25: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
  26: RINvMs_CslsHbb2RadMK_10scoped_tlsINtB5_9ScopedKeyNtCs2vb4bKWGfvL_10rustc_span14SessionGlobalsE3setNCINvNtCs6lxe6u1d1HN_15rustc_interface9interface12run_compilerINtNtCs9qroIgEvBu5_4core6result6ResultuNtCslxr0cr2Vr2Y_12rustc_errors15ErrorGuaranteedENCNvCsg2
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-dev running on x86_64-pc-windows-msvc

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -Z randomize-layout -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C target-feature=+crt-static -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `rustc-demangle`

Caused by:
  process didn't exit successfully: `G:\Users\Chase\Code\Rust\rust\build\bootstrap\debug\rustc --crate-name rustc_demangle C:\Users\Chase\.cargo\registry\src\github.com-1ecc6299db9ec823\rustc-demangle-0.1.21\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 --cfg "feature=\"compiler_builtins\"" --cfg "feature=\"core\"" --cfg "feature=\"rustc-dep-of-std\"" -Zunstable-options --check-cfg names() --check-cfg values() -C metadata=4d891bf2cbe595ae -C extra-filename=-4d891bf2cbe595ae --out-dir G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -L dependency=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps -L dependency=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\release\deps --extern compiler_builtins=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps\libcompiler_builtins-2c6af95dce564732.rmeta --extern core=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps\librustc_std_workspace_core-710036e409e62f0d.rmeta --cap-lints allow -Z randomize-layout -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) "--check-cfg=values(target_env,\"libnx\")" "--check-cfg=values(target_os,\"watchos\")" "--check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\")" --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Ctarget-feature=+crt-static -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes "-Zcrate-attr=doc(html_root_url=\"https://doc.rust-lang.org/nightly/\")" -Z binary-dep-depinfo` (exit code: 0x80000003)
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: compiler\rustc_codegen_llvm\src\type_of.rs:146:13: layout: TyAndLayout {
                                    ty: core::result::Result<usize, core::alloc::LayoutError>,
                                    layout: Layout {
                                        fields: Arbitrary {
                                            offsets: [
                                                Size(8 bytes),
                                            ],
                                            memory_index: [
                                                0,
                                            ],
                                        },
                                        variants: Single {
                                            index: 1,
                                        },
                                        abi: Aggregate {
                                            sized: true,
                                        },
                                        largest_niche: None,
                                        align: AbiAndPrefAlign {
                                            abi: Align(1 bytes),
                                            pref: Align(8 bytes),
                                        },
                                        size: Size(2 bytes),
                                    },
                                } stride: Size(2 bytes) offset: Size(8 bytes)

thread 'rustc' panicked at 'Box<dyn Any>', G:\Users\Chase\Code\Rust\rust\compiler\rustc_errors\src\lib.rs:1334:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: <rustc_errors::Handler>::span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
   2: <rustc_errors::Handler>::span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
   3: <rustc_errors::Handler>::bug::<&alloc::string::String>
   4: <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_binder::<rustc_middle::ty::sty::FnSig>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_codegen_llvm::type_of::struct_llfields
   8: <rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty> as rustc_codegen_llvm::type_of::LayoutLlvmExt>::llvm_type
   9: <rustc_codegen_ssa::mir::place::PlaceRef<&rustc_codegen_llvm::llvm_::ffi::Value>>::project_downcast::<rustc_codegen_llvm::builder::Builder>
  10: <rustc_codegen_ssa::mir::FunctionCx<rustc_codegen_llvm::builder::Builder>>::codegen_place
  11: rustc_codegen_ssa::mir::codegen_mir::<rustc_codegen_llvm::builder::Builder>
  12: rustc_codegen_ssa::base::codegen_instance::<rustc_codegen_llvm::builder::Builder>
  13: <rustc_middle::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define::<rustc_codegen_llvm::builder::Builder>
  14: rustc_codegen_llvm::base::compile_codegen_unit
  15: RINvXNtCscInEZjIJEzE_12rustc_middle9dep_graphNtNtB3_8dep_node7DepKindNtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph7DepKind9with_depsNCINvMs1_NtB17_5graphINtB2i_8DepGraphBH_E14with_task_implNtNtNtB5_2ty7context6TyCtxtNtNtCs2vb4bKWGfvL_10rustc_span6symbo
  16: RINvMs1_NtNtCs1LEY3TOdXnz_18rustc_query_system9dep_graph5graphINtB6_8DepGraphNtNtNtCscInEZjIJEzE_12rustc_middle9dep_graph8dep_node7DepKindE9with_taskNtNtNtB1j_2ty7context6TyCtxtNtNtCs2vb4bKWGfvL_10rustc_span6symbol6SymbolINtCs4wqBg0PzBnM_17rustc_codegen_s
  17: rustc_codegen_llvm::base::compile_codegen_unit
  18: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
  19: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  20: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  21: rustc_interface::passes::start_codegen
  22: RINvMs_NtCs6lxe6u1d1HN_15rustc_interface6passesNtB5_12QueryContext5enterNCNCNvMs0_NtB7_7queriesNtB1i_7Queries15ongoing_codegen00INtNtCs9qroIgEvBu5_4core6result6ResultINtNtCs4fCcauHRDzO_5alloc5boxed3BoxDNtNtB27_3any3AnyEL_ENtCslxr0cr2Vr2Y_12rustc_errors15E
  23: <rustc_interface::queries::Query<alloc::boxed::Box<dyn core::any::Any>>>::compute::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}>
  24: RINvMs2_NtCs6lxe6u1d1HN_15rustc_interface7queriesNtNtB8_9interface8Compiler5enterNCNCNvCsg2BpsMvkavS_12rustc_driver12run_compilers_0s0_0INtNtCs9qroIgEvBu5_4core6result6ResultINtNtB2f_6option6OptionNtB6_6LinkerENtCslxr0cr2Vr2Y_12rustc_errors15ErrorGuarante
  25: RINvCs2vb4bKWGfvL_10rustc_span15with_source_mapINtNtCs9qroIgEvBu5_4core6result6ResultuNtCslxr0cr2Vr2Y_12rustc_errors15ErrorGuaranteedENCINvNtCs6lxe6u1d1HN_15rustc_interface9interface23create_compiler_and_runBJ_NCNvCsg2BpsMvkavS_12rustc_driver12run_compile
  26: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
  27: RINvMs_CslsHbb2RadMK_10scoped_tlsINtB5_9ScopedKeyNtCs2vb4bKWGfvL_10rustc_span14SessionGlobalsE3setNCINvNtCs6lxe6u1d1HN_15rustc_interface9interface12run_compilerINtNtCs9qroIgEvBu5_4core6result6ResultuNtCslxr0cr2Vr2Y_12rustc_errors15ErrorGuaranteedENCNvCsg2
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-dev running on x86_64-pc-windows-msvc

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C incremental -Z randomize-layout -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C target-feature=+crt-static -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `alloc`

Caused by:
  process didn't exit successfully: `G:\Users\Chase\Code\Rust\rust\build\bootstrap\debug\rustc --crate-name alloc --edition=2021 library\alloc\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Zunstable-options --check-cfg names() --check-cfg values() -C metadata=44eda8c292567317 -C extra-filename=-44eda8c292567317 --out-dir G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -C incremental=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\incremental -L dependency=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps -L dependency=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\release\deps --extern compiler_builtins=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps\libcompiler_builtins-2c6af95dce564732.rmeta --extern core=G:\Users\Chase\Code\Rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps\libcore-a73f3dcab1ae4e39.rmeta -Z randomize-layout -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(stdarch_intel_sde) --check-cfg=values(no_fp_fmt_parse) --check-cfg=values(no_global_oom_handling) --check-cfg=values(freebsd12) --check-cfg=values(backtrace_in_libstd) "--check-cfg=values(target_env,\"libnx\")" "--check-cfg=values(target_os,\"watchos\")" "--check-cfg=values(target_arch,\"asmjs\",\"spirv\",\"nvptx\",\"nvptx64\",\"le32\",\"xtensa\")" --check-cfg=values(dont_compile_me) -Zmacro-backtrace -Ctarget-feature=+crt-static -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes "-Zcrate-attr=doc(html_root_url=\"https://doc.rust-lang.org/nightly/\")" -Z binary-dep-depinfo` (exit code: 0x80000003)
Build completed unsuccessfully in 0:06:02
