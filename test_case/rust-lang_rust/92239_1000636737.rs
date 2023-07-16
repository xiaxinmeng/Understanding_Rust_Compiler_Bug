
2021-12-24T02:12:07.7357772Z error: failed to sync
2021-12-24T02:12:07.7358416Z 
2021-12-24T02:12:07.7358773Z Caused by:
2021-12-24T02:12:07.7359650Z   found duplicate version of package `hashbrown v0.11.2` vendored from two sources:
2021-12-24T02:12:07.7372682Z 
2021-12-24T02:12:07.7374834Z   	source 1: registry `crates-io`
2021-12-24T02:12:07.7376518Z   	source 2: https://github.com/nnethercote/hashbrown?branch=add-is_empty-checks#dec8e8cc
2021-12-24T02:12:07.7378823Z 
2021-12-24T02:12:07.7379152Z 
2021-12-24T02:12:07.7380930Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor" "--sync" "/checkout/./src/tools/rust-analyzer/Cargo.toml" "/checkout/./compiler/rustc_codegen_cranelift/Cargo.toml"
2021-12-24T02:12:07.7382936Z expected success, got: exit status: 101
