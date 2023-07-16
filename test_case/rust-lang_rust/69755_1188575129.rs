bash
$ time rustc +1.59.0 y.rs -C opt-level=3
real	0m0.354s
user	0m0.429s
sys	0m0.042s

$ time rustc +1.58.0 y.rs -C opt-level=3
real	6m55.526s
user	6m54.649s
sys	0m0.968s
