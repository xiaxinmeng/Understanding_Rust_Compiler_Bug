
234,881,855  ???:<rustc_middle::mir::interpret::allocation::provenance_map::ProvenanceMap>::prepare_copy::<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>
 15,729,623  ???:<rustc_middle::mir::interpret::allocation::init_mask::InitMask>::set_range
-15,729,396  ???:<rustc_middle::mir::interpret::allocation::InitMask>::set_range
 12,813,596  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
  8,388,642  ???:<rustc_const_eval::interpret::memory::AllocRef<rustc_middle::mir::interpret::AllocId, ()>>::get_bytes_strip_provenance
 -7,341,503  ???:<rustc_middle::mir::interpret::allocation::Allocation>::is_init
  7,078,108  ???:<rustc_middle::mir::interpret::allocation::Allocation>::init_mask_apply_copy
 -7,078,082  ???:<rustc_middle::mir::interpret::allocation::Allocation>::mark_compressed_init_range
  6,292,512  ???:<rustc_middle::mir::interpret::allocation::init_mask::InitMask>::prepare_copy
 -6,263,018  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::run
 -5,506,061  ???:<rustc_middle::mir::interpret::allocation::Allocation>::compress_uninit_range
  4,718,592  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::read_scalar
 -2,856,650  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::copy_op_no_validate
  1,834,335  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_fn_call
  1,205,870  ???:<rustc_middle::ty::context::TyCtxt>::try_subst_and_normalize_erasing_regions::<rustc_middle::ty::Ty>
 -1,048,576  ???:<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::unsize_into_ptr
    734,054  ???:<rustc_const_eval::const_eval::machine::CompileTimeInterpreter as rustc_const_eval::interpret::machine::Machine>::find_mir_or_eval_fn
   -427,346  ???:rustc_metadata::rmeta::decoder::cstore_impl::provide_extern::implementations_of_trait
    393,344  ???:rustc_codegen_llvm::consts::const_alloc_to_llvm
   -393,315  ???:<rustc_middle::mir::interpret::value::Scalar>::to_machine_usize::<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>
    392,908  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor>
    391,766  ???:<rustc_middle::arena::Arena>::alloc_from_iter::<(rustc_span::def_id::DefId, core::option::Option<rustc_middle::ty::fast_reject::SimplifiedTypeGen<rustc_span::def_id::DefId>>), rustc_arena::IsCopy, core::iter::adapters::map::Map<rustc_metadata::rmeta::decoder::DecodeIterator<(rustc_span::def_id::DefIndex, core::option::Option<rustc_middle::ty::fast_reject::SimplifiedTypeGen<rustc_span::def_id::DefId>>)>, <rustc_metadata::creader::CrateMetadataRef>::get_implementations_of_trait::{closure
   -314,718  ???:rustc_trait_selection::traits::specialize::specializes
    275,520  ???:rustc_trait_selection::traits::specialize::fulfill_implication

