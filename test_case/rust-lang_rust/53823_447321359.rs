
% rustc +nightly-2018-02-12 -O -C llvm-args=-x86-cmov-converter=false --test lib.rs -o ../benches-2018-02-12.nocmovconv
% rustc +nightly-2018-02-12 -O                                        --test lib.rs -o ../benches-2018-02-12
% rustc +nightly-2018-02-09 -O                                        --test lib.rs -o ../benches-2018-02-09
% ../benches-2018-02-09 --bench slice

running 6 tests
test slice::binary_search_l1                               ... bench:          21 ns/iter (+/- 0)
test slice::binary_search_l1_with_dups                     ... bench:          21 ns/iter (+/- 0)
test slice::binary_search_l2                               ... bench:          31 ns/iter (+/- 0)
test slice::binary_search_l2_with_dups                     ... bench:          31 ns/iter (+/- 0)
test slice::binary_search_l3                               ... bench:         110 ns/iter (+/- 1)
test slice::binary_search_l3_with_dups                     ... bench:         112 ns/iter (+/- 7)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 142 filtered out

% ../benches-2018-02-12 --bench slice

running 6 tests
test slice::binary_search_l1                               ... bench:          51 ns/iter (+/- 0)
test slice::binary_search_l1_with_dups                     ... bench:          46 ns/iter (+/- 0)
test slice::binary_search_l2                               ... bench:          67 ns/iter (+/- 1)
test slice::binary_search_l2_with_dups                     ... bench:          67 ns/iter (+/- 0)
test slice::binary_search_l3                               ... bench:         143 ns/iter (+/- 1)
test slice::binary_search_l3_with_dups                     ... bench:         142 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 142 filtered out

% ../benches-2018-02-12.nocmovconv --bench slice

running 6 tests
test slice::binary_search_l1                               ... bench:          21 ns/iter (+/- 0)
test slice::binary_search_l1_with_dups                     ... bench:          21 ns/iter (+/- 0)
test slice::binary_search_l2                               ... bench:          31 ns/iter (+/- 1)
test slice::binary_search_l2_with_dups                     ... bench:          31 ns/iter (+/- 0)
test slice::binary_search_l3                               ... bench:         114 ns/iter (+/- 2)
test slice::binary_search_l3_with_dups                     ... bench:         115 ns/iter (+/- 1)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 142 filtered out
