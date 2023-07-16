rust
                if let Some(node_id) = tcx.hir.as_local_node_id(*def_id) {
                    check_main_fn_ty(tcx, node_id, *def_id, *sp);
                } else {
                    span_bug!(*sp, "NodeId of `main` is not found");
                }
