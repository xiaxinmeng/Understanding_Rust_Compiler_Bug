console
$ curl -O https://gist.githubusercontent.com/eddyb/d17d5303c544c19f78569392b635c813/raw/0aa1c947c91450f3d2e227b9d58d9ea19fe6ebf3/proj-tri.rs
$ command time -f 'took %Us' rustc proj-tri.rs --emit=metadata --cfg 'factor = "1"'
took 0.24s
$ command time -f 'took %Us' rustc proj-tri.rs --emit=metadata --cfg 'factor = "2"'
took 1.02s
$ command time -f 'took %Us' rustc proj-tri.rs --emit=metadata --cfg 'factor = "3"'
took 2.83s
$ command time -f 'took %Us' rustc proj-tri.rs --emit=metadata --cfg 'factor = "4"'
took 6.48s
