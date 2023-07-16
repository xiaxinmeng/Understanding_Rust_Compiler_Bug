rust
pub(crate) fn check_local_binding_shadows_glob_reexport(&mut self) {
    debug!("check_local_binding_shadows_glob_reexport");
    for module in self.arenas.local_modules().iter() {
        let resolutions = self.resolutions(module).borrow();
        for (_, resolution) in resolutions.iter() {
            let resolution = resolution.borrow();

            if let Some(binding) = resolution.binding
            && let Some(glob_binding) = resolution.shadowed_glob
            && let Some(glob_def_id) = glob_binding.res().opt_def_id()
            && let Some(glob_local_def_id) = glob_def_id.as_local()
            {
                debug!("check_local_binding_shadows_glob_reexport: self.effective_visibilities.is_exported(glob_local_def_id) = {}", self.effective_visibilities.is_exported(glob_local_def_id));
                if self.effective_visibilities.is_exported(glob_local_def_id)
                {
                    debug!("check_local_binding_shadows_glob_reexport: Some(binding) && Some(amb_binding)");
                    if let Some(def_id) = binding.res().opt_def_id()
                    && let Some(local_def_id) = def_id.as_local()
                    && let Some(ns) = binding.res().ns()
                    {
                        self.lint_buffer.buffer_lint_with_diagnostic(
                            LOCAL_BINDING_SHADOWS_GLOB_REEXPORT,
                            self.def_id_to_node_id[local_def_id],
                            binding.span,
                            "local binding shadows glob re-export",
                            BuiltinLintDiagnostics::LocalShadowsGlobReexport {
                                name: self.tcx.item_name(def_id).to_string(),
                                namespace: ns.descr().to_owned(),
                                glob_reexport_span: glob_binding.span,
                                local_binding_span: binding.span,
                            },
                        );
                    }
                }
            }
        }
    }
}
