
running: "/usr/x86_64-pc-linux-gnu/bin/cargo-beta" "generate-lockfile" "--offline"
running: "/usr/x86_64-pc-linux-gnu/bin/cargo-beta" "vendor" "--offline" "--locked" "/var/tmp/paludis/build/dev-lang-rust-1.49.0-scm/work/rust-1.49.0-scm/build/tmp/dist/rust-src-beta-image/lib/rustlib/src/rust/vend
or"
                                                                                                 
command did not execute successfully: "/usr/x86_64-pc-linux-gnu/bin/cargo-beta" "vendor" "--offline" "--locked" "/var/tmp/paludis/build/dev-lang-rust-1.49.0-scm/work/rust-1.49.0-scm/build/tmp/dist/rust-src-beta-im
age/lib/rustlib/src/rust/vendor"
expected success, got: exit code: 101                                                                              
error: failed to sync            
Caused by:                   
  failed to download packages                                                               
Caused by:               
  failed to download `rustc-std-workspace-alloc v1.0.0`
Caused by:                                                                                                                                                                                                           
  can't make HTTP request in the offline mode   
