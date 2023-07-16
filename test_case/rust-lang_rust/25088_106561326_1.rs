
$ rustc foo.rs --bench -Copt-level=3
$ ./bench --bench
running 2 tests                                              
test libstd   ... bench:        21 ns/iter (+/- 1)           
test pthreads ... bench:        18 ns/iter (+/- 4)           

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured   
$ rustc foo.rs --bench -Copt-level=3 -Clto
$ ./bench --bench
running 2 tests                                              
test libstd   ... bench:        16 ns/iter (+/- 1)           
test pthreads ... bench:        15 ns/iter (+/- 4)           

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured   
