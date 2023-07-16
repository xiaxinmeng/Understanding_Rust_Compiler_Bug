rust
// Filter out -O and /O (the optimization flags) that we picked up from
// gcc-rs because the build scripts will determine that for themselves.
let mut base = self.cc[target].0.args().iter()
                           .map(|s| s.to_string_lossy().into_owned())
                           .filter(|s| !s.starts_with("-O") && !s.starts_with("/O"))
                           .collect::<Vec<_>>();
