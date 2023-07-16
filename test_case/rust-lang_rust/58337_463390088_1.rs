
[01:15:52] [0m[0m[1m[32m   Compiling[0m clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
[01:15:57] [0m[1m[38;5;9merror[E0599][0m[0m[1m: no method named `node_id_to_type` found for type `&'a rustc::ty::TypeckTables<'tcx>` in the current scope[0m
[01:15:57] [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/tools/clippy/clippy_lints/src/utils/inspector.rs:130:65[0m
[01:15:57] [0m    [0m[0m[1m[38;5;12m|[0m
[01:15:57] [0m[1m[38;5;12m130[0m[0m [0m[0m[1m[38;5;12m| [0m[0m                println!("local variable of type {}", cx.tables.node_id_to_type(local.hir_id));[0m
