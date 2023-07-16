
$ time rustdoc +nightly no-assoc-items.rs 

real	0m0.481s
user	0m0.412s
sys	0m0.049s
$ time rustdoc +stage1 no-assoc-items.rs 

real	0m0.748s
user	0m0.655s
sys	0m0.072s
