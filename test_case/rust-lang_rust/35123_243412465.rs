 rust
        let ids = self.paths.iter()
                            .map(|path| {
                                if self.krate_still_valid(tcx, max_current_crate, path.krate) {
                                    tcx.retrace_path(path)
                                } else {
                                    debug!("crate {} changed from {:?} to {:?}/{:?}",
                                           path.krate,
                                           self.krates[path.krate as usize],
                                           tcx.crate_name(path.krate),
                                           tcx.crate_disambiguator(path.krate));
                                    None
                                }
                            })
                            .collect();
