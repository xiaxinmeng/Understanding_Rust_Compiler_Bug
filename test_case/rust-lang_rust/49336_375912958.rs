
$ cargo +nightly-2017-12-27 bench --bench my_benchmark -- --verbose
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/my_benchmark-c6ffda0acddb4b02
Benchmarking fib 1
Benchmarking fib 1: Warming up for 3.0000 s
Benchmarking fib 1: Collecting 100 samples in estimated 1.0000 s (1316272400 iterations)
Benchmarking fib 1: Analyzing
fib 1                   time:   [767.18 ps 778.06 ps 789.83 ps]
                        change: [-3.1129% -1.4570% +0.2070%] (p = 0.09 > 0.05)
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) high mild
  15 (15.00%) high severe
slope  [767.18 ps 789.83 ps] R^2            [0.7407196 0.7387507]
mean   [764.74 ps 782.08 ps] std. dev.      [29.638 ps 57.337 ps]
median [752.23 ps 756.69 ps] med. abs. dev. [3.0748 ps 9.0096 ps]

Benchmarking fib 2
Benchmarking fib 2: Warming up for 3.0000 s
Benchmarking fib 2: Collecting 100 samples in estimated 1.0000 s (224447250 iterations)
Benchmarking fib 2: Analyzing
fib 2                   time:   [4.3392 ns 4.3744 ns 4.4208 ns]
                        change: [-0.6421% +1.1830% +2.7979%] (p = 0.19 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
slope  [4.3392 ns 4.4208 ns] R^2            [0.8011943 0.7965177]
mean   [4.3847 ns 4.4583 ns] std. dev.      [130.47 ps 247.30 ps]
median [4.3365 ns 4.4123 ns] med. abs. dev. [111.97 ps 189.57 ps]
