
Gnuplot not found, using plotters backend
Benchmarking small slice      [no null in slice]/iter::position
Benchmarking small slice      [no null in slice]/iter::position: Warming up for 3.0000 s
Benchmarking small slice      [no null in slice]/iter::position: Collecting 100 samples in estimated 5.0000 s (37M iterations)
Benchmarking small slice      [no null in slice]/iter::position: Analyzing
small slice      [no null in slice]/iter::position
                        time:   [132.00 ns 132.52 ns 133.09 ns]
                        change: [-1.8419% -1.2738% -0.7000%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
Benchmarking small slice      [no null in slice]/wmemchr
Benchmarking small slice      [no null in slice]/wmemchr: Warming up for 3.0000 s
Benchmarking small slice      [no null in slice]/wmemchr: Collecting 100 samples in estimated 5.0003 s (66M iterations)
Benchmarking small slice      [no null in slice]/wmemchr: Analyzing
small slice      [no null in slice]/wmemchr
                        time:   [75.364 ns 75.735 ns 76.131 ns]
                        change: [-1.5130% -0.9615% -0.3817%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking small slice      [null in the middle of slice]/iter::position
Benchmarking small slice      [null in the middle of slice]/iter::position: Warming up for 3.0000 s
Benchmarking small slice      [null in the middle of slice]/iter::position: Collecting 100 samples in estimated 5.0003 s (48M iterations)
Benchmarking small slice      [null in the middle of slice]/iter::position: Analyzing
small slice      [null in the middle of slice]/iter::position
                        time:   [104.68 ns 105.02 ns 105.38 ns]
                        change: [-0.9901% -0.5467% -0.1005%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
Benchmarking small slice      [null in the middle of slice]/wmemchr
Benchmarking small slice      [null in the middle of slice]/wmemchr: Warming up for 3.0000 s
Benchmarking small slice      [null in the middle of slice]/wmemchr: Collecting 100 samples in estimated 5.0002 s (81M iterations)
Benchmarking small slice      [null in the middle of slice]/wmemchr: Analyzing
small slice      [null in the middle of slice]/wmemchr
                        time:   [61.391 ns 61.607 ns 61.838 ns]
                        change: [-0.5171% -0.1115% +0.2818%] (p = 0.60 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

Benchmarking small slice      [null at the end of slice]/iter::position
Benchmarking small slice      [null at the end of slice]/iter::position: Warming up for 3.0000 s
Benchmarking small slice      [null at the end of slice]/iter::position: Collecting 100 samples in estimated 5.0005 s (37M iterations)
Benchmarking small slice      [null at the end of slice]/iter::position: Analyzing
small slice      [null at the end of slice]/iter::position
                        time:   [134.30 ns 134.75 ns 135.25 ns]
                        change: [-1.1961% -0.7560% -0.3037%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  9 (9.00%) high mild
Benchmarking small slice      [null at the end of slice]/wmemchr
Benchmarking small slice      [null at the end of slice]/wmemchr: Warming up for 3.0000 s
Benchmarking small slice      [null at the end of slice]/wmemchr: Collecting 100 samples in estimated 5.0000 s (66M iterations)
Benchmarking small slice      [null at the end of slice]/wmemchr: Analyzing
small slice      [null at the end of slice]/wmemchr
                        time:   [76.419 ns 76.654 ns 76.929 ns]
                        change: [-1.2010% -0.7157% -0.2252%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  10 (10.00%) high mild

Benchmarking [medium slice    [no null in slice]/iter::position
Benchmarking [medium slice    [no null in slice]/iter::position: Warming up for 3.0000 s
Benchmarking [medium slice    [no null in slice]/iter::position: Collecting 100 samples in estimated 5.0980 s (162k iterations)
Benchmarking [medium slice    [no null in slice]/iter::position: Analyzing
[medium slice    [no null in slice]/iter::position
                        time:   [31.464 us 31.564 us 31.673 us]
                        change: [-0.6138% -0.1810% +0.2327%] (p = 0.41 > 0.05)
                        No change in performance detected.
Benchmarking [medium slice    [no null in slice]/wmemchr
Benchmarking [medium slice    [no null in slice]/wmemchr: Warming up for 3.0000 s
Benchmarking [medium slice    [no null in slice]/wmemchr: Collecting 100 samples in estimated 5.0530 s (318k iterations)
Benchmarking [medium slice    [no null in slice]/wmemchr: Analyzing
[medium slice    [no null in slice]/wmemchr
                        time:   [15.824 us 15.912 us 16.027 us]
                        change: [-0.4151% +0.1534% +0.7968%] (p = 0.62 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking medium slice     [null in the middle of slice]/iter::position
Benchmarking medium slice     [null in the middle of slice]/iter::position: Warming up for 3.0000 s
Benchmarking medium slice     [null in the middle of slice]/iter::position: Collecting 100 samples in estimated 5.0215 s (212k iterations)
Benchmarking medium slice     [null in the middle of slice]/iter::position: Analyzing
medium slice     [null in the middle of slice]/iter::position
                        time:   [23.649 us 23.728 us 23.813 us]
                        change: [-1.3613% -0.9745% -0.5564%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  10 (10.00%) high mild
  1 (1.00%) high severe
Benchmarking medium slice     [null in the middle of slice]/wmemchr
Benchmarking medium slice     [null in the middle of slice]/wmemchr: Warming up for 3.0000 s
Benchmarking medium slice     [null in the middle of slice]/wmemchr: Collecting 100 samples in estimated 5.0533 s (424k iterations)
Benchmarking medium slice     [null in the middle of slice]/wmemchr: Analyzing
medium slice     [null in the middle of slice]/wmemchr
                        time:   [11.838 us 11.881 us 11.928 us]
                        change: [-0.7297% -0.2970% +0.1571%] (p = 0.20 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  13 (13.00%) high mild

Benchmarking medium slice     [null at the end of slice]/iter::position
Benchmarking medium slice     [null at the end of slice]/iter::position: Warming up for 3.0000 s
Benchmarking medium slice     [null at the end of slice]/iter::position: Collecting 100 samples in estimated 5.1239 s (162k iterations)
Benchmarking medium slice     [null at the end of slice]/iter::position: Analyzing
medium slice     [null at the end of slice]/iter::position
                        time:   [31.429 us 31.551 us 31.679 us]
                        change: [-1.3095% -0.7629% -0.2579%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Benchmarking medium slice     [null at the end of slice]/wmemchr
Benchmarking medium slice     [null at the end of slice]/wmemchr: Warming up for 3.0000 s
Benchmarking medium slice     [null at the end of slice]/wmemchr: Collecting 100 samples in estimated 5.0453 s (318k iterations)
Benchmarking medium slice     [null at the end of slice]/wmemchr: Analyzing
medium slice     [null at the end of slice]/wmemchr
                        time:   [15.737 us 15.800 us 15.866 us]
                        change: [-1.1871% -0.8049% -0.4046%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  10 (10.00%) high mild

