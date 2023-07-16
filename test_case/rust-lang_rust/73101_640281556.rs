
            // The immediate parent might not always be a module.
            // Find the first parent which is.
            loop {
                if let Some(parent) = self.cx.tcx.parent(current) {
                    if self.cx.tcx.def_kind(parent) == DefKind::Mod {
                        break Some(parent);
                    }
                    current = parent;
                } else {
                    break None;
                }
            }
