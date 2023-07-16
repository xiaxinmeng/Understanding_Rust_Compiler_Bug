
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-3f391b84552f210adec7893b50c5da74f9362ae4-ctfe-stress-5-Debug-IncrFull; results/cgfilt-d584f7c49e7118da3500a697b2f6e3ba36f4af0a-ctfe-stress-5-Debug-IncrFull
Command:          /home/jake/.rustup/toolchains/3f391b84552f210adec7893b50c5da74f9362ae4/bin/rustc --crate-name ctfe_stress_5 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C incremental=/tmp/.tmpfvxNSK/incremental-state -C metadata=4aff8ac78ce36ed0 -C extra-filename=-4aff8ac78ce36ed0 --out-dir /tmp/.tmpfvxNSK/target/debug/deps -L dependency=/tmp/.tmpfvxNSK/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich; /home/jake/.rustup/toolchains/d584f7c49e7118da3500a697b2f6e3ba36f4af0a/bin/rustc --crate-name ctfe_stress_5 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C incremental=/tmp/.tmpM6OXv8/incremental-state -C metadata=4aff8ac78ce36ed0 -C extra-filename=-4aff8ac78ce36ed0 --out-dir /tmp/.tmpM6OXv8/target/debug/deps -L dependency=/tmp/.tmpM6OXv8/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich
Data file:        results/cgfilt-diff-3f391b84552f210adec7893b50c5da74f9362ae4-d584f7c49e7118da3500a697b2f6e3ba36f4af0a-ctfe-stress-5-Debug-IncrFull
Events recorded:  Ir
Events shown:     Ir
Event sort order: Ir
Thresholds:       0.1
Include dirs:     
User annotated:   
Auto-annotation:  on

--------------------------------------------------------------------------------
Ir          
--------------------------------------------------------------------------------
126,984,827  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
 77,782,066  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::run
 40,129,096  ???:__memcpy_sse2_unaligned_erms
 27,525,798  ???:<rustc_data_structures::intern::Interned<rustc_middle::mir::interpret::allocation::Allocation> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
-11,011,547  ???:__memcpy_avx_unaligned_erms
 -6,235,908  ???:<rustc_const_eval::const_eval::machine::CompileTimeInterpreter as rustc_const_eval::interpret::machine::Machine>::find_mir_or_eval_fn
  5,141,834  ???:memcpy@GLIBC_2.2.5
 -4,470,840  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
 -2,202,162  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_fn_call
  1,574,512  ???:llvm::MCAssembler::writeSectionData(llvm::raw_ostream&, llvm::MCSection const*, llvm::MCAsmLayout const&) const
 -1,153,476  ???:<rustc_const_eval::interpret::memory::AllocRef<rustc_middle::mir::interpret::AllocId, ()>>::read_ptr_sized
 -1,101,050  ???:core::iter::adapters::try_process::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::mir::Operand>, <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_operands::{closure
   -787,282  ???:<rustc_const_eval::interpret::memory::AllocRef<rustc_middle::mir::interpret::AllocId, ()>>::read_scalar
    688,131  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::operand_projection
   -486,026  ???:rustc_infer::infer::lexical_region_resolve::resolve
    433,001  ???:<rustc_infer::infer::lexical_region_resolve::LexicalResolver>::infer_variable_values
    367,016  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::mir_const_to_op
    139,917  ???:<rustc_middle::ty::instance::Instance>::resolve_opt_const_arg
    131,146  ???:<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>>::field::<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>
    131,071  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::unsize_into_ptr

