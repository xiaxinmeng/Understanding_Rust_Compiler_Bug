
2020-09-14T20:15:45.5866300Z error: failed to sync
2020-09-14T20:15:45.5868991Z 
2020-09-14T20:15:45.5869767Z Caused by:
2020-09-14T20:15:45.5870722Z   found duplicate version of package `psm v0.1.11` vendored from two sources:
2020-09-14T20:15:45.5871743Z 
2020-09-14T20:15:45.5873616Z   	source 1: registry `https://github.com/rust-lang/crates.io-index`
2020-09-14T20:15:45.5877915Z   	source 2: https://github.com/Julian-Wollersberger/stacker#71993c7a
2020-09-14T20:15:45.5890461Z 
2020-09-14T20:15:45.5890685Z 
2020-09-14T20:15:45.5892185Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor" "--sync" "/checkout/./src/tools/rust-analyzer/Cargo.toml"
