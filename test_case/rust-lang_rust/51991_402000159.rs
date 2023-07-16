
[01:04:16] [0m[0m[1m[32m   Compiling[0m clippy_lints v0.0.211 (file:///checkout/src/tools/clippy/clippy_lints)
[01:04:22] [0m[1m[38;5;9merror[E0609][0m[0m[1m: no field `id` on type `&rustc::middle::mem_categorization::cmt_<'tcx>`[0m
[01:04:22] [0m   [0m[0m[1m[38;5;12m--> [0m[0mtools/clippy/clippy_lints/src/escape.rs:111:74[0m
[01:04:22] [0m    [0m[0m[1m[38;5;12m|[0m
[01:04:22] [0m[1m[38;5;12m111[0m[0m [0m[0m[1m[38;5;12m| [0m[0m            if let Some(NodeStmt(st)) = map.find(map.get_parent_node(cmt.id)) {[0m
[01:04:22] [0m    [0m[0m[1m[38;5;12m| [0m[0m                                                                         [0m[0m[1m[38;5;9m^^[0m
[01:04:22] 
[01:04:24] [0m[1m[38;5;9merror[0m[0m[1m: aborting due to previous error[0m
[01:04:24] 
[01:04:24] [0m[1mFor more information about this error, try `rustc --explain E0609`.[0m
[01:04:24] [RUSTC-TIMING] clippy_lints test:false 7.924
[01:04:24] [0m[0m[1m[31merror:[0m Could not compile `clippy_lints`.
