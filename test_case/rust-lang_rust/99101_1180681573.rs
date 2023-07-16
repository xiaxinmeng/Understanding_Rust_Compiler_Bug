
-3,801,248,960  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::run
 1,613,392,295  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::place_projection
 1,385,495,085  ???:<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::read_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure
   753,302,796  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::mir::syntax::ProjectionElem<rustc_middle::mir::Local, rustc_middle::ty::Ty>>> as core::iter::traits::iterator::Iterator>::try_fold::<rustc_const_eval::interpret::place::PlaceTy, <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_place::{closure
   423,232,360  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::operand_projection
   422,177,426  ???:<rustc_middle::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle::mir::ConstantKind>
   337,379,637  ???:<rustc_middle::ty::context::TyCtxt>::def_kind::<rustc_span::def_id::DefId>
  -287,883,397  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_place
  -238,841,573  ???:<rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty
   231,210,827  ???:<hashbrown::map::RawEntryBuilder<rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId>, (core::result::Result<rustc_middle::mir::interpret::value::ConstAlloc, rustc_middle::mir::interpret::error::ErrorHandled>, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::from_key_hashed_nocheck::<rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId>>
   128,949,866  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::mir::syntax::ProjectionElem<rustc_middle::mir::Local, rustc_middle::ty::Ty>>> as core::iter::traits::iterator::Iterator>::try_fold::<rustc_const_eval::interpret::operand::OpTy, <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_place_to_op::{closure
    92,367,305  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_fn_call
    82,575,360  ???:<rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId> as core::hash::Hash>::hash::<rustc_hash::FxHasher>
    77,073,927  ???:core::iter::adapters::try_process::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::mir::syntax::Operand>, <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_operands::{closure
   -66,060,288  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::mplace_projection
    52,187,019  ???:<rustc_middle::ty::context::TyCtxt>::normalize_erasing_late_bound_regions::<rustc_middle::ty::sty::FnSig>
   -44,867,150  ???:<rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
    39,303,819  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::copy_op_no_validate
    35,861,557  ???:<hashbrown::map::RawEntryBuilder<rustc_middle::ty::ParamEnvAnd<(rustc_middle::ty::instance::Instance, &rustc_middle::ty::list::List<rustc_middle::ty::Ty>)>, (core::result::Result<&rustc_target::abi::call::FnAbi<rustc_middle::ty::Ty>, rustc_middle::ty::layout::FnAbiError>, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::from_key_hashed_nocheck::<rustc_middle::ty::ParamEnvAnd<(rustc_middle::ty::instance::Instance, &rustc_middle::ty::list::List<rustc_middle::ty::Ty>)>>
    27,918,462  ???:<rustc_middle::mir::interpret::value::Scalar>::to_bool
   -23,609,880  ???:<rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::TypeFolder>::fold_const
    14,163,497  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::subst::SubstFolder>
     9,909,729  ???:<rustc_middle::ty::ParamEnvAnd<(rustc_middle::ty::instance::Instance, &rustc_middle::ty::list::List<rustc_middle::ty::Ty>)> as core::hash::Hash>::hash::<rustc_hash::FxHasher>
     8,441,621  ???:<rustc_const_eval::const_eval::machine::CompileTimeInterpreter as rustc_const_eval::interpret::machine::Machine>::after_stack_pop
     6,606,486  ???:<rustc_middle::ty::context::TyCtxt>::mk_type_list::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_const_eval::interpret::operand::OpTy>, <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_terminator::{closure
     5,601,675  ???:<rustc_middle::ty::instance::Instance>::resolve_opt_const_arg
    -2,216,572  ???:<rustc_middle::ty::Ty>::fn_sig
    -2,113,558  ???:<rustc_middle::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_mir_const
