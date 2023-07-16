
master:

test vec_deque::bench_into_iter                          ... bench:       2,615 ns/iter (+/- 320)
test vec_deque::bench_into_iter_fold                     ... bench:       1,756 ns/iter (+/- 80)
test vec_deque::bench_into_iter_next_chunk               ... bench:       2,458 ns/iter (+/- 2,441)
test vec_deque::bench_into_iter_try_fold                 ... bench:       1,433 ns/iter (+/- 992)

This PR:

test vec_deque::bench_into_iter                          ... bench:       2,560 ns/iter (+/- 161)
test vec_deque::bench_into_iter_fold                     ... bench:       1,751 ns/iter (+/- 45)
test vec_deque::bench_into_iter_next_chunk               ... bench:         546 ns/iter (+/- 17)
test vec_deque::bench_into_iter_try_fold                 ... bench:         640 ns/iter (+/- 30)
