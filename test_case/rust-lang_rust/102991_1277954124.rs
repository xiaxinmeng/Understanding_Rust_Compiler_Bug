
master
collections::vec_deque::tests::bench_pop_back_100       ... bench:         187 ns/iter (+/- 5)
test collections::vec_deque::tests::bench_pop_front_100      ... bench:         165 ns/iter (+/- 2)
test collections::vec_deque::tests::bench_push_back_100      ... bench:         165 ns/iter (+/- 6)
test collections::vec_deque::tests::bench_push_front_100     ... bench:         165 ns/iter (+/- 2)
test collections::vec_deque::tests::bench_retain_half_10000  ... bench:     381,092 ns/iter (+/- 5,772)
test collections::vec_deque::tests::bench_retain_odd_10000   ... bench:     383,285 ns/iter (+/- 10,536)
test collections::vec_deque::tests::bench_retain_whole_10000 ... bench:     355,320 ns/iter (+/- 7,691)

test vec_deque::bench_extend_bytes                       ... bench:          10 ns/iter (+/- 0)
test vec_deque::bench_extend_chained_bytes               ... bench:       1,661 ns/iter (+/- 59)
test vec_deque::bench_extend_chained_trustedlen          ... bench:       1,617 ns/iter (+/- 58)
test vec_deque::bench_extend_trustedlen                  ... bench:       1,238 ns/iter (+/- 51)
test vec_deque::bench_extend_vec                         ... bench:          26 ns/iter (+/- 1)
test vec_deque::bench_from_array_1000                    ... bench:         192 ns/iter (+/- 7)
test vec_deque::bench_grow_1025                          ... bench:       1,986 ns/iter (+/- 90)
test vec_deque::bench_iter_1000                          ... bench:         465 ns/iter (+/- 17)
test vec_deque::bench_mut_iter_1000                      ... bench:         467 ns/iter (+/- 10)
test vec_deque::bench_new                                ... bench:          15 ns/iter (+/- 0)
test vec_deque::bench_try_fold                           ... bench:          34 ns/iter (+/- 0)

custom-bench vec_deque_append 311562 ns/iter

This PR
collections::vec_deque::tests::bench_pop_back_100       ... bench:         148 ns/iter (+/- 1)
test collections::vec_deque::tests::bench_pop_front_100      ... bench:         146 ns/iter (+/- 1)
test collections::vec_deque::tests::bench_push_back_100      ... bench:         169 ns/iter (+/- 4)
test collections::vec_deque::tests::bench_push_front_100     ... bench:         194 ns/iter (+/- 4)
test collections::vec_deque::tests::bench_retain_half_10000  ... bench:     255,548 ns/iter (+/- 5,597)
test collections::vec_deque::tests::bench_retain_odd_10000   ... bench:     257,342 ns/iter (+/- 8,091)
test collections::vec_deque::tests::bench_retain_whole_10000 ... bench:     231,784 ns/iter (+/- 6,616)

test vec_deque::bench_extend_bytes                       ... bench:         695 ns/iter (+/- 16)
test vec_deque::bench_extend_chained_bytes               ... bench:         813 ns/iter (+/- 32)
test vec_deque::bench_extend_chained_trustedlen          ... bench:         735 ns/iter (+/- 30)
test vec_deque::bench_extend_trustedlen                  ... bench:          15 ns/iter (+/- 0)
test vec_deque::bench_extend_vec                         ... bench:          27 ns/iter (+/- 0)
test vec_deque::bench_from_array_1000                    ... bench:         195 ns/iter (+/- 5)
test vec_deque::bench_grow_1025                          ... bench:       2,074 ns/iter (+/- 418)
test vec_deque::bench_iter_1000                          ... bench:         328 ns/iter (+/- 18)
test vec_deque::bench_mut_iter_1000                      ... bench:         327 ns/iter (+/- 5)
test vec_deque::bench_new                                ... bench:           2 ns/iter (+/- 0)
test vec_deque::bench_try_fold                           ... bench:          34 ns/iter (+/- 2)

custom-bench vec_deque_append 144851 ns/iter
