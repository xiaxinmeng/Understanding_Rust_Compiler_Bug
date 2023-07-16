
2019-07-04T15:13:25.6174709Z [0m[1m[38;5;9merror[0m[0m[1m: use of deprecated item 'core::mem::uninitialized': use `mem::MaybeUninit` instead[0m
2019-07-04T15:13:25.6176343Z [0m  [0m[0m[1m[38;5;12m--> [0m[0msrc/libstd/io/util.rs:47:54[0m
2019-07-04T15:13:25.6176801Z [0m   [0m[0m[1m[38;5;12m|[0m
2019-07-04T15:13:25.6177324Z [0m[1m[38;5;12m47[0m[0m [0m[0m[1m[38;5;12m| [0m[0m        let mut buf: [u8; super::DEFAULT_BUF_SIZE] = mem::uninitialized();[0m
2019-07-04T15:13:25.6177845Z [0m   [0m[0m[1m[38;5;12m| [0m[0m                                                     [0m[0m[1m[38;5;9m^^^^^^^^^^^^^^^^^^[0m
2019-07-04T15:13:25.6178286Z [0m   [0m[0m[1m[38;5;12m|[0m
2019-07-04T15:13:25.6178733Z [0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: `-D deprecated` implied by `-D warnings`[0m2019-07-04T15:13:25.6174709Z [0m[1m[38;5;9merror[0m[0m[1m: use of deprecated item 'core::mem::uninitialized': use `mem::MaybeUninit` instead[0m
2019-07-04T15:13:25.6176343Z [0m  [0m[0m[1m[38;5;12m--> [0m[0msrc/libstd/io/util.rs:47:54[0m
2019-07-04T15:13:25.6176801Z [0m   [0m[0m[1m[38;5;12m|[0m
2019-07-04T15:13:25.6177324Z [0m[1m[38;5;12m47[0m[0m [0m[0m[1m[38;5;12m| [0m[0m        let mut buf: [u8; super::DEFAULT_BUF_SIZE] = mem::uninitialized();[0m
2019-07-04T15:13:25.6177845Z [0m   [0m[0m[1m[38;5;12m| [0m[0m                                                     [0m[0m[1m[38;5;9m^^^^^^^^^^^^^^^^^^[0m
2019-07-04T15:13:25.6178286Z [0m   [0m[0m[1m[38;5;12m|[0m
2019-07-04T15:13:25.6178733Z [0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: `-D deprecated` implied by `-D warnings`[0m
