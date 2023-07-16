
librustc_mir/borrow_check/type_check/mod.rs:1236:            &tcx.typeck_tables_of(anon_owner_def_id.expect_local()).concrete_opaque_types;
librustc_typeck/collect/type_of.rs:123:                            &tcx.typeck_tables_of(owner.expect_local()).concrete_opaque_types
librustc_typeck/collect/type_of.rs:431:            if !self.tcx.typeck_tables_of(def_id).concrete_opaque_types.contains_key(&self.def_id) {
