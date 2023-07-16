rust
impl fmt::Debug for DefId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DefId {{ krate: {:?}, node: {:?}",
               self.krate, self.index)?;

        ty::tls::with_opt(|opt_tcx| {
            if let Some(tcx) = opt_tcx {
                if let Some(def_path) = tcx.opt_def_path(*self) {
                    write!(f, " => {}", def_path.to_string(tcx))?;       //  <--- LINE 124
                }
            }
            Ok(())
        })?;

        write!(f, " }}")
    }
}
