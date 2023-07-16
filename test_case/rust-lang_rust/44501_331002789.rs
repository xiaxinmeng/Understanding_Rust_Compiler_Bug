
[00:49:06] [1m[31merror[E0624](B[m[1m: method `uninhabited_from` is private(B[m
[00:49:06]     (B[m[1m[34m--> (B[m/checkout/src/tools/miri/src/librustc_mir/interpret/eval_context.rs:2241:8(B[m
[00:49:06]      (B[m[1m[34m|(B[m
[00:49:06] [1m[34m2241(B[m (B[m[1m[34m| (B[m    ty.uninhabited_from(&mut HashMap::default(), tcx).is_empty()(B[m
[00:49:06]      (B[m[1m[34m| (B[m       (B[m[1m[31m^^^^^^^^^^^^^^^^(B[m
[00:49:06] 
[00:49:07] [1m[31merror(B[m[1m: aborting due to previous error(B[m
[00:49:07] 
