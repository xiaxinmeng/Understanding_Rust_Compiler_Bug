rust
let trait_: Option<DefId> = self.tcx.trait_of_item(self.def_id);
let generics: &Generics = self.tcx.generics_of(trait_.unwrap());
