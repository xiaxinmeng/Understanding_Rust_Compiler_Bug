rust
   .or_else(|| self.resolve_path(&path_root, TypeNS, module_id))
   .or_else(|| self_id.map(|id| Res::Def(self.cx.tcx.def_kind(id), id)))
