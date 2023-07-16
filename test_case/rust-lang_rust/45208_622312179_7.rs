
librustc_mir/borrow_check/diagnostics/conflict_errors.rs:883:                                    .typeck_tables_of(def_id)
librustc_mir/borrow_check/diagnostics/mutability_errors.rs:510:            let tables = self.infcx.tcx.typeck_tables_of(def_id);
librustc_mir/borrow_check/universal_regions.rs:501:                    let tables = tcx.typeck_tables_of(self.mir_def_id.expect_local());
librustc_typeck/collect/type_of.rs:606:    let ty = tcx.diagnostic_only_typeck_tables_of(def_id).node_type(body_id.hir_id);
librustc_mir_build/build/mod.rs:89:                    let gen_ty = tcx.body_tables(body_id).node_type(id);
librustc_mir_build/build/mod.rs:144:                let gen_ty = tcx.body_tables(body_id).node_type(id);
librustc_mir_build/build/mod.rs:211:    let closure_ty = tcx.body_tables(body_id).node_type(closure_expr_id);
