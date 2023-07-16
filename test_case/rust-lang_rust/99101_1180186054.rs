
-2,582,601,110  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::run
 1,416,740,602  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::place_projection
   780,827,916  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::mir::syntax::ProjectionElem<rustc_middle::mir::Local, rustc_middle::ty::Ty>>> as core::iter::traits::iterator::Iterator>::try_fold::<rustc_const_eval::interpret::place::PlaceTy, <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_place::{closure
   517,636,783  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::operand_projection
   422,177,426  ???:<rustc_middle::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle::mir::ConstantKind>
  -234,321,940  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_place
   231,210,917  ???:<hashbrown::map::RawEntryBuilder<rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId>, (core::result::Result<rustc_middle::mir::interpret::value::ConstAlloc, rustc_middle::mir::interpret::error::ErrorHandled>, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::from_key_hashed_nocheck::<rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId>>
   193,633,898  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::mir::syntax::ProjectionElem<rustc_middle::mir::Local, rustc_middle::ty::Ty>>> as core::iter::traits::iterator::Iterator>::try_fold::<rustc_const_eval::interpret::operand::OpTy, <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_place_to_op::{closure
   193,462,272  ???:<rustc_middle::ty::context::TyCtxt>::def_kind::<rustc_span::def_id::DefId>
    82,575,360  ???:<rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId> as core::hash::Hash>::hash::<rustc_hash::FxHasher>
    77,440,963  ???:core::iter::adapters::try_process::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::mir::syntax::Operand>, <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_operands::{closure
    68,975,205  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_fn_call
    56,819,838  ???:<rustc_middle::mir::interpret::value::Scalar>::to_bool
    52,187,019  ???:<rustc_middle::ty::context::TyCtxt>::normalize_erasing_late_bound_regions::<rustc_middle::ty::sty::FnSig>
   -50,331,648  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::mplace_projection
    39,303,720  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::copy_op_no_validate
    35,599,183  ???:<hashbrown::map::RawEntryBuilder<rustc_middle::ty::ParamEnvAnd<(rustc_middle::ty::instance::Instance, &rustc_middle::ty::list::List<rustc_middle::ty::Ty>)>, (core::result::Result<&rustc_target::abi::call::FnAbi<rustc_middle::ty::Ty>, rustc_middle::ty::layout::FnAbiError>, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::from_key_hashed_nocheck::<rustc_middle::ty::ParamEnvAnd<(rustc_middle::ty::instance::Instance, &rustc_middle::ty::list::List<rustc_middle::ty::Ty>)>>
     9,909,729  ???:<rustc_middle::ty::ParamEnvAnd<(rustc_middle::ty::instance::Instance, &rustc_middle::ty::list::List<rustc_middle::ty::Ty>)> as core::hash::Hash>::hash::<rustc_hash::FxHasher>
     8,441,621  ???:<rustc_const_eval::const_eval::machine::CompileTimeInterpreter as rustc_const_eval::interpret::machine::Machine>::after_stack_pop
     6,606,486  ???:<rustc_middle::ty::context::TyCtxt>::mk_type_list::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_const_eval::interpret::operand::OpTy>, <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_terminator::{closure
     2,657,099  ???:<rustc_middle::ty::context::TyCtxt>::intern_type_list
    -2,359,340  ???:<rustc_middle::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_mir_const
     1,974,134  ???:<rustc_middle::ty::instance::Instance>::resolve_opt_const_arg
