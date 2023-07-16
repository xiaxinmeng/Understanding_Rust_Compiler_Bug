rust
ty::FnDef(..) => {
    let sig_hash = ty.fn_sig(self.tcx).to_string();
    self = self.print_def_path(def_id, substs)?;
    self.push(&sig_hash);
}
