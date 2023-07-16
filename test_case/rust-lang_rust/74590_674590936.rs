rust
    tcx.sess.time("item_types_checking", || {
        for &module in tcx.hir().krate().modules.keys() {
            tcx.sess.time("check_mod_item_types", || {
            tcx.ensure().check_mod_item_types(tcx.hir().local_def_id(module));
            });
        }
    });
