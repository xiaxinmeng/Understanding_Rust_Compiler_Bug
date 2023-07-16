
error[E0271]: type mismatch resolving `<i32 as TraitB>::AssocB == impl std::marker::Send`
  --> atb.rs:37:1
   |
37 | existential type Bar: Debug + TraitB<AssocB = impl Send>;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found opaque type
   |
   = note: expected type `u32`
              found type `impl std::marker::Send`
   = note: the return type of a function must have a statically known size

DEBUG 2019-01-25T02:19:53Z: rustc_typeck::collect: find_existential_constraints(DefId(0/1:9 ~ atb[317d]::Bar[0]::{{impl-Trait}}[0]))
TRACE 2019-01-25T02:19:53Z: rustc_typeck::collect: find_existential_constraints: parent_id=NodeId(0)
