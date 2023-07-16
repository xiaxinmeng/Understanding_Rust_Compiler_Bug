
An error occurred in miri:
   0: <rustc_middle::mir::interpret::error::InterpErrorInfo as core::convert::From<rustc_middle::mir::interpret::error::InterpError>>::from
             at ./compiler/rustc_middle/src/mir/interpret/error.rs:102:53
   1: rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra>::check_relocations
             at ./compiler/rustc_middle/src/mir/interpret/allocation.rs:477:13
      rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra>::check_relocation_edges
             at ./compiler/rustc_middle/src/mir/interpret/allocation.rs:534:9
      rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra>::get_bytes_internal
             at ./compiler/rustc_middle/src/mir/interpret/allocation.rs:216:13
      rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra>::get_bytes_with_uninit_and_ptr
             at ./compiler/rustc_middle/src/mir/interpret/allocation.rs:251:9
   2: rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra>::read_scalar
             at ./compiler/rustc_middle/src/mir/interpret/allocation.rs:360:21
   3: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::try_read_immediate_from_mplace
             at ./compiler/rustc_mir/src/interpret/operand.rs:269:30
      rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::try_read_immediate
             at ./compiler/rustc_mir/src/interpret/operand.rs:302:36
   4: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::read_immediate
             at ./compiler/rustc_mir/src/interpret/operand.rs:318:26
      rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::read_scalar
             at ./compiler/rustc_mir/src/interpret/operand.rs:330:12
   5: rustc_mir::interpret::validity::ValidityVisitor<M>::try_visit_primitive
             at ./compiler/rustc_mir/src/interpret/validity.rs:514:29
   6: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_value
             at ./compiler/rustc_mir/src/interpret/validity.rs:739:12
   7: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_field::{{closure}}
             at ./compiler/rustc_mir/src/interpret/validity.rs:706:42
      rustc_mir::interpret::validity::ValidityVisitor<M>::with_elem
             at ./compiler/rustc_mir/src/interpret/validity.rs:304:17
      <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_field
             at ./compiler/rustc_mir/src/interpret/validity.rs:706:9
   8: rustc_mir::interpret::visitor::ValueVisitor::walk_aggregate
             at ./compiler/rustc_mir/src/interpret/visitor.rs:197:21
      <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_aggregate
             at ./compiler/rustc_mir/src/interpret/validity.rs:884:17
      rustc_mir::interpret::visitor::ValueVisitor::walk_value
             at ./compiler/rustc_mir/src/interpret/visitor.rs:236:25
   9: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_value
             at ./compiler/rustc_mir/src/interpret/validity.rs:755:9
  10: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_field::{{closure}}
             at ./compiler/rustc_mir/src/interpret/validity.rs:706:42
      rustc_mir::interpret::validity::ValidityVisitor<M>::with_elem
             at ./compiler/rustc_mir/src/interpret/validity.rs:304:17
      <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_field
             at ./compiler/rustc_mir/src/interpret/validity.rs:706:9
  11: rustc_mir::interpret::visitor::ValueVisitor::walk_aggregate
             at ./compiler/rustc_mir/src/interpret/visitor.rs:197:21
      <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_aggregate
             at ./compiler/rustc_mir/src/interpret/validity.rs:884:17
      rustc_mir::interpret::visitor::ValueVisitor::walk_value
             at ./compiler/rustc_mir/src/interpret/visitor.rs:236:25
  12: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_value
             at ./compiler/rustc_mir/src/interpret/validity.rs:755:9
      rustc_mir::interpret::validity::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::validate_operand_internal
             at ./compiler/rustc_mir/src/interpret/validity.rs:908:15
  13: rustc_mir::interpret::validity::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::const_validate_operand
             at ./compiler/rustc_mir/src/interpret/validity.rs:941:9
      rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider
             at ./compiler/rustc_mir/src/const_eval/eval_queries.rs:398:25
  14: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_allocation_raw>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
