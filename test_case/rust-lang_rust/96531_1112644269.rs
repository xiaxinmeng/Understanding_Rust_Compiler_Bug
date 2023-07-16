rust
tcx.sess.time("item_types_checking", || { 
  tcx.hir().for_each_module(|module| tcx.ensure().check_mod_item_types(module))
});

// and

tcx.hir().for_each_module(module| tcx.ensure().check_mod_impl_wf(module)) });
