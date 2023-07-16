 rust
                let sp = match ccx.tcx.items.get_copy(&ref_id.get()) {
                    ast_map::node_expr(e) => e.span,
                    _ => fail!("transmute has non-expr arg"),
                };
