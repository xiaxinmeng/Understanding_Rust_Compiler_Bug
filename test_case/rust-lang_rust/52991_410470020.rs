
[00:13:14] [0m[1m[38;5;9merror[0m[0m[1m: field is never used: `tcx`[0m
[00:13:14] [0m  [0m[0m[1m[38;5;12m--> [0m[0mlibrustc_mir/borrow_check/nll/escaping_locals.rs:79:5[0m
[00:13:14] [0m   [0m[0m[1m[38;5;12m|[0m
[00:13:14] [0m[1m[38;5;12m79[0m[0m [0m[0m[1m[38;5;12m| [0m[0m    tcx: TyCtxt<'cx, 'gcx, 'tcx>,[0m
[00:13:14] [0m   [0m[0m[1m[38;5;12m| [0m[0m    [0m[0m[1m[38;5;9m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
[00:13:14] [0m   [0m[0m[1m[38;5;12m|[0m
[00:13:14] [0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: `-D dead-code` implied by `-D warnings`[0m
[00:13:14] 
[00:13:14] [0m[1m[38;5;9merror[0m[0m[1m: field is never used: `mir`[0m
[00:13:14] [0m  [0m[0m[1m[38;5;12m--> [0m[0mlibrustc_mir/borrow_check/nll/escaping_locals.rs:80:5[0m
[00:13:14] [0m   [0m[0m[1m[38;5;12m|[0m
[00:13:14] [0m[1m[38;5;12m80[0m[0m [0m[0m[1m[38;5;12m| [0m[0m    mir: &'cx Mir<'tcx>,[0m
[00:13:14] [0m   [0m[0m[1m[38;5;12m| [0m[0m    [0m[0m[1m[38;5;9m^^^^^^^^^^^^^^^^^^^[0m
[00:13:14] 
[00:13:14] [0m[1m[38;5;9merror[0m[0m[1m: aborting due to 2 previous errors[0m
[00:13:14] 
[00:13:14] [0m[0m[1m[31merror:[0m Could not compile `rustc_mir`.
[00:13:14] 
