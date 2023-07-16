rust
        for id in defids.to_sorted_vec() {
            let CodegenFnAttrs { optimize, .. } = tcx.codegen_fn_attrs(*id);
            match optimize {
                attr::OptimizeAttr::None => continue,
                attr::OptimizeAttr::Size => continue,
                attr::OptimizeAttr::Speed => {
                    return for_speed;
                }
            }
        }
