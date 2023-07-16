
2020-07-10T02:47:08.2898965Z [0m[1m[38;5;9merror[E0412][0m[0m[1m: cannot find type `ThreadBuilder` in this scope[0m
2020-07-10T02:47:08.2899884Z [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/librustc_interface/util.rs:189:50[0m
2020-07-10T02:47:08.2900915Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-10T02:47:08.2901666Z [0m[1m[38;5;12m189[0m[0m [0m[0m[1m[38;5;12m| [0m[0m                let main_handler = move |thread: ThreadBuilder| {[0m
2020-07-10T02:47:08.2902574Z [0m    [0m[0m[1m[38;5;12m| [0m[0m                                                 [0m[0m[1m[38;5;9m^^^^^^^^^^^^^[0m[0m [0m[0m[1m[38;5;9mnot found in this scope[0m
2020-07-10T02:47:08.2910253Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-10T02:47:08.2911045Z [0m[1m[38;5;14mhelp[0m[0m: consider importing this struct[0m
2020-07-10T02:47:08.2911621Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-10T02:47:08.2912948Z [0m[1m[38;5;12m1[0m[0m   [0m[0m[1m[38;5;12m| [0m[0muse rayon::ThreadBuilder;[0m
2020-07-10T02:47:08.2913528Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-10T02:47:08.2914595Z 
2020-07-10T02:47:08.7129510Z [0m[1m[38;5;9merror[0m[0m[1m: aborting due to previous error[0m
2020-07-10T02:47:08.7129786Z 
2020-07-10T02:47:08.7130355Z [0m[1mFor more information about this error, try `rustc --explain E0412`.[0m
2020-07-10T02:47:08.7322787Z [0m[0m[1m[31merror[0m[1m:[0m could not compile `rustc_interface`.
2020-07-10T02:47:08.7323085Z 
