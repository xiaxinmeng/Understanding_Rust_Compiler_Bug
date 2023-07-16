rs
let suggest = if let ty::FnDef(def_id, _) = rcvr_ty.kind()
                            && let DefKind::Ctor(of, _) = tcx.def_kind(def_id)
                        {
                            let parent_def_id = tcx.parent(*def_id);
                            Some((tcx.associated_item_def_ids(parent_def_id).len(), of))
                        } else {
                            None
                        };
