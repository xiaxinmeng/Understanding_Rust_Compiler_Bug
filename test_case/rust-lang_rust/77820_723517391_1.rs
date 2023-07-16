rust
    fn clean(&self, cx: &DocContext<'_>) -> Visibility {
        match *self {
            ty::Visibility::Public => Visibility::Public,
            ty::Visibility::Restricted(module) => {
                if module.is_top_level_module() {
                    Visibility::Crate
                } else {
                    Visibility::Restricted(module, cx.tcx.def_path(module).clean(cx))
                }
            }
            ty::Visibility::Invisible => Visibility::Inherited,
        }
    }
