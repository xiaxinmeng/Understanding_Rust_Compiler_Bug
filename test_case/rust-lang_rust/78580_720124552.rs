rust
let callee =
    Instance::resolve(self.tcx, self.param_env, callee_def_id, normalized_substs)
        .ok()
        .flatten()?;
