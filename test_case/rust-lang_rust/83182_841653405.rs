
An error occurred in miri:
   0: <rustc_middle::mir::interpret::error::InterpErrorInfo as core::convert::From<rustc_middle::mir::interpret::error::InterpError>>::from
             at ./compiler/rustc_middle/src/mir/interpret/error.rs:118:53
   1: rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra>::get_bytes_internal
   2: rustc_middle::mir::interpret::allocation::Allocation<Tag,Extra>::get_bytes
             at ./compiler/rustc_middle/src/mir/interpret/allocation.rs:237:9
      rustc_mir::interpret::memory::Memory<M>::read_bytes
             at ./compiler/rustc_mir/src/interpret/memory.rs:804:9
   3: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_aggregate
             at ./compiler/rustc_mir/src/interpret/validity.rs:833:21
      rustc_mir::interpret::visitor::ValueVisitor::walk_value
             at ./compiler/rustc_mir/src/interpret/visitor.rs:252:25
   4: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_value
             at ./compiler/rustc_mir/src/interpret/validity.rs:788:9
   5: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_field::{{closure}}
             at ./compiler/rustc_mir/src/interpret/validity.rs:739:42
      rustc_mir::interpret::validity::ValidityVisitor<M>::with_elem
             at ./compiler/rustc_mir/src/interpret/validity.rs:311:17
      <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_field
             at ./compiler/rustc_mir/src/interpret/validity.rs:739:9
   6: rustc_mir::interpret::visitor::ValueVisitor::walk_aggregate
             at ./compiler/rustc_mir/src/interpret/visitor.rs:201:21
      <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_aggregate
             at ./compiler/rustc_mir/src/interpret/validity.rs:917:17
      rustc_mir::interpret::visitor::ValueVisitor::walk_value
             at ./compiler/rustc_mir/src/interpret/visitor.rs:240:25
   7: <rustc_mir::interpret::validity::ValidityVisitor<M> as rustc_mir::interpret::visitor::ValueVisitor<M>>::visit_value
             at ./compiler/rustc_mir/src/interpret/validity.rs:788:9
      rustc_mir::interpret::validity::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::validate_operand_internal
             at ./compiler/rustc_mir/src/interpret/validity.rs:941:15
   8: rustc_mir::interpret::validity::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::const_validate_operand
             at ./compiler/rustc_mir/src/interpret/validity.rs:974:9
      rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider
             at ./compiler/rustc_mir/src/const_eval/eval_queries.rs:352:21
