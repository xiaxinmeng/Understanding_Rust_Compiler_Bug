txt
error[E0463]: can't find crate for `std`

For more information about this error, try `rustc --explain E0463`.
error: could not compile `serde_derive` due to previous error
warning: build failed, waiting for other jobs to finish...
error[E0463]: can't find crate for `core`

error: build failed


command did not execute successfully: ".../build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--manifest-path" ".../src/tools/clippy/Cargo.toml" "-Zskip-rustdoc-fingerprint" "--no-deps" "-p" "clippy_lints" "-p" "clippy_utils"
expected success, got: exit status: 101
