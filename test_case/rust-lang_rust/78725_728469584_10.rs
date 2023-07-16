
220:                    &format!("upvar_tys for closure not found. Expected capture information for closure {}", ty,),
226:                for ty in substs.as_closure().upvar_tys() {
262:                    &format!("upvar_tys for generator not found. Expected capture information for generator {}", ty,),
270:                    .upvar_tys()
