 rust
        let ids = self.paths.iter()
                            .map(|path| {
                                if let Some(new_krate) = self.map_crate(path.krate) {
                                    let mut new_path = DefPath { krate: new_krate, data: path.data.clone() };
                                    tcx.retrace_path(&new_path)
                                } else {
                                    // ...just as before...
                                }
                            })
                            .collect();
