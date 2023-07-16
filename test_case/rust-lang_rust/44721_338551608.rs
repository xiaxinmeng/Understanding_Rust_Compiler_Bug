rust
            inputs: decl.inputs.iter()
                .map(|arg| {
                    // FIXME causes ICE compliling stage 1, need to find a better solution
                    let def_id = self.resolver.definitions().local_def_id(arg.id);
                    self.lower_ty(&arg.ty, TyLoweringCtx::FnParameter(def_id))
                }).collect(),
