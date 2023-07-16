Rust
            let layout_of = |ty: Ty<'tcx>| {
                ty.layout(tcx, ty::ParamEnv::empty(traits::Reveal::All))
                    .map_err(|err| {
                        ConstEvalErr { span: e.span, kind: LayoutError(err) }
                    })
            };
