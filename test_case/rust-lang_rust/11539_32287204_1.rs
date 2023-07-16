
run: x86_64-unknown-linux-gnu/stage1/test/stdtest-x86_64-unknown-linux-gnu

running 5 tests
test vec::bench::zero_1kb_fixed_repeat                          ... bench:        74 ns/iter (+/- 3)
test vec::bench::zero_1kb_from_elem                             ... bench:        73 ns/iter (+/- 4)
test vec::bench::zero_1kb_loop_set                              ... bench:       550 ns/iter (+/- 3)
test vec::bench::zero_1kb_mut_iter                              ... bench:        72 ns/iter (+/- 3)
test vec::bench::zero_1kb_set_memory                            ... bench:        72 ns/iter (+/- 3)

metrics saved to: tmp/check-stage1-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-std-metrics.json
test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured
