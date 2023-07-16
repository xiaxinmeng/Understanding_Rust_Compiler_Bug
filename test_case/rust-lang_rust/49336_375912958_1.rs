
$ cargo +nightly-2017-12-28 bench --bench my_benchmark -- --verbose
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/my_benchmark-08eeef1a988add44
Benchmarking fib 1
Benchmarking fib 1: Warming up for 3.0000 s
Benchmarking fib 1: Collecting 100 samples in estimated 1.0000 s (3763674100 iterations)
Benchmarking fib 1: Analyzing
fib 1                   time:   [266.56 ps 267.97 ps 269.51 ps]
                        change: [-65.621% -65.152% -64.693%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe
slope  [266.56 ps 269.51 ps] R^2            [0.9507674 0.9500646]
mean   [267.64 ps 271.29 ps] std. dev.      [5.0293 ps 13.253 ps]
median [267.92 ps 269.51 ps] med. abs. dev. [2.5710 ps 5.5603 ps]

Benchmarking fib 2
Benchmarking fib 2: Warming up for 3.0000 s
Benchmarking fib 2: Collecting 100 samples in estimated 1.0000 s (3787974700 iterations)
Benchmarking fib 2: Analyzing
fib 2                   time:   [4.9535 ns 4.9735 ns 4.9952 ns]
                        change: [+11.531% +12.592% +13.578%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
slope  [4.9535 ns 4.9952 ns] R^2            [0.9572156 0.9568266]
mean   [4.9577 ns 4.9946 ns] std. dev.      [76.681 ps 110.54 ps]
median [4.9522 ns 4.9900 ns] med. abs. dev. [55.560 ps 100.25 ps]
