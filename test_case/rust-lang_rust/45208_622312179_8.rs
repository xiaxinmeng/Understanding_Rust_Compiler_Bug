
librustc_mir/borrow_check/mod.rs:140:    let tables = tcx.typeck_tables_of(def_id);
librustc_mir_build/hair/cx/mod.rs:59:        let tables = tcx.typeck_tables_of(src_def_id);
librustc_passes/intrinsicck.rs:135:        let tables = self.tcx.typeck_tables_of(owner_def_id);
librustc_passes/liveness.rs:685:        let tables = ir.tcx.typeck_tables_of(def_id);
librustc_privacy/lib.rs:357:    if tcx.has_typeck_tables(def_id) { tcx.typeck_tables_of(def_id) } else { empty_tables }
librustc_save_analysis/dump_visitor.rs:113:            self.tcx.typeck_tables_of(item_def_id)
librustc_trait_selection/traits/error_reporting/suggestions.rs:1246:                query_tables = self.tcx.typeck_tables_of(generator_did.expect_local());
librustc_driver/pretty.rs:333:            self.tables.set(self.tcx.body_tables(id));
librustc_lint/late.rs:108:        self.context.tables = self.context.tcx.body_tables(body);
librustc_lint/late.rs:184:        self.context.tables = self.context.tcx.body_tables(body_id);
librustc_mir_build/hair/pattern/check_match.rs:30:        MatchVisitor { tcx, tables: tcx.body_tables(body_id), param_env: tcx.param_env(def_id) };
librustc_passes/dead.rs:223:        self.tables = self.tcx.body_tables(body);
librustc_passes/reachable.rs:85:        self.tables = self.tcx.body_tables(body);
librustc_privacy/lib.rs:1092:        let orig_tables = mem::replace(&mut self.tables, self.tcx.body_tables(body));
librustc_privacy/lib.rs:1234:        let orig_tables = mem::replace(&mut self.tables, self.tcx.body_tables(body));
