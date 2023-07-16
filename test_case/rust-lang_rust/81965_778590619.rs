rust
        // For `const fn` we want to render the optimized MIR. If you want the mir used in
        // ctfe, you can dump the MIR after the `Deaggregator` optimization pass.
        if tcx.is_const_fn_raw(def_id) {
            render_body(w, tcx.optimized_mir(def_id))?;
            writeln!(w)?;
            writeln!(w, "// MIR FOR CTFE")?;
            // Do not use `render_body`, as that would render the promoteds again, but these
            // are shared between mir_for_ctfe and optimized_mir
            write_mir_fn(tcx, tcx.mir_for_ctfe(def_id), &mut |_, _| Ok(()), w)?;
        } else {
            match def_id.as_local() {
                None => {
                    let instance_mir = tcx.instance_mir(ty::InstanceDef::Item(
                        ty::WithOptConstParam::unknown(def_id),
                    ));
                    render_body(w, instance_mir)?;
                }
                Some(_local_def_id) =>
                    render_body(w, tcx.optimized_mir(def_id))?,
            }
        }
