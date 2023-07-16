

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Benchmarking small slice      [no null in slice]/iter::position
Benchmarking small slice      [no null in slice]/iter::position: Warming up for 3.0000 s
Benchmarking small slice      [no null in slice]/iter::position: Collecting 100 samples in estimated 5.0002 s (64M iterations)
Benchmarking small slice      [no null in slice]/iter::position: Analyzing
small slice      [no null in slice]/iter::position
                        time:   [72.035 ns 72.096 ns 72.170 ns]
Benchmarking small slice      [no null in slice]/wmemchr
Benchmarking small slice      [no null in slice]/wmemchr: Warming up for 3.0000 s
Benchmarking small slice      [no null in slice]/wmemchr: Collecting 100 samples in estimated 5.0002 s (67M iterations)
Benchmarking small slice      [no null in slice]/wmemchr: Analyzing
small slice      [no null in slice]/wmemchr
                        time:   [74.066 ns 74.123 ns 74.191 ns]
Found 26 outliers among 100 measurements (26.00%)
  13 (13.00%) low mild
  9 (9.00%) high mild
  4 (4.00%) high severe
Benchmarking small slice      [no null in slice]/unrolled
Benchmarking small slice      [no null in slice]/unrolled: Warming up for 3.0000 s
Benchmarking small slice      [no null in slice]/unrolled: Collecting 100 samples in estimated 5.0001 s (130M iterations)
Benchmarking small slice      [no null in slice]/unrolled: Analyzing
small slice      [no null in slice]/unrolled
                        time:   [38.224 ns 38.261 ns 38.319 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking small slice      [null in the middle of slice]/iter::position
Benchmarking small slice      [null in the middle of slice]/iter::position: Warming up for 3.0000 s
Benchmarking small slice      [null in the middle of slice]/iter::position: Collecting 100 samples in estimated 5.0001 s (85M iterations)
Benchmarking small slice      [null in the middle of slice]/iter::position: Analyzing
small slice      [null in the middle of slice]/iter::position
                        time:   [58.422 ns 58.495 ns 58.597 ns]
Found 24 outliers among 100 measurements (24.00%)
  8 (8.00%) low severe
  3 (3.00%) low mild
  4 (4.00%) high mild
  9 (9.00%) high severe
Benchmarking small slice      [null in the middle of slice]/wmemchr
Benchmarking small slice      [null in the middle of slice]/wmemchr: Warming up for 3.0000 s
Benchmarking small slice      [null in the middle of slice]/wmemchr: Collecting 100 samples in estimated 5.0001 s (82M iterations)
Benchmarking small slice      [null in the middle of slice]/wmemchr: Analyzing
small slice      [null in the middle of slice]/wmemchr
                        time:   [60.681 ns 60.768 ns 60.872 ns]
Found 19 outliers among 100 measurements (19.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  13 (13.00%) high severe
Benchmarking small slice      [null in the middle of slice]/unrolled
Benchmarking small slice      [null in the middle of slice]/unrolled: Warming up for 3.0000 s
Benchmarking small slice      [null in the middle of slice]/unrolled: Collecting 100 samples in estimated 5.0001 s (173M iterations)
Benchmarking small slice      [null in the middle of slice]/unrolled: Analyzing
small slice      [null in the middle of slice]/unrolled
                        time:   [28.801 ns 28.828 ns 28.863 ns]
Found 24 outliers among 100 measurements (24.00%)
  2 (2.00%) low severe
  6 (6.00%) low mild
  3 (3.00%) high mild
  13 (13.00%) high severe

Benchmarking small slice      [null at the end of slice]/iter::position
Benchmarking small slice      [null at the end of slice]/iter::position: Warming up for 3.0000 s
Benchmarking small slice      [null at the end of slice]/iter::position: Collecting 100 samples in estimated 5.0003 s (68M iterations)
Benchmarking small slice      [null at the end of slice]/iter::position: Analyzing
small slice      [null at the end of slice]/iter::position
                        time:   [73.022 ns 73.115 ns 73.227 ns]
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) low mild
  5 (5.00%) high mild
  4 (4.00%) high severe
Benchmarking small slice      [null at the end of slice]/wmemchr
Benchmarking small slice      [null at the end of slice]/wmemchr: Warming up for 3.0000 s
Benchmarking small slice      [null at the end of slice]/wmemchr: Collecting 100 samples in estimated 5.0001 s (65M iterations)
Benchmarking small slice      [null at the end of slice]/wmemchr: Analyzing
small slice      [null at the end of slice]/wmemchr
                        time:   [76.097 ns 76.297 ns 76.520 ns]
Benchmarking small slice      [null at the end of slice]/unrolled
Benchmarking small slice      [null at the end of slice]/unrolled: Warming up for 3.0000 s
Benchmarking small slice      [null at the end of slice]/unrolled: Collecting 100 samples in estimated 5.0001 s (130M iterations)
Benchmarking small slice      [null at the end of slice]/unrolled: Analyzing
small slice      [null at the end of slice]/unrolled
                        time:   [38.251 ns 38.301 ns 38.366 ns]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

Benchmarking [medium slice    [no null in slice]/iter::position
Benchmarking [medium slice    [no null in slice]/iter::position: Warming up for 3.0000 s
Benchmarking [medium slice    [no null in slice]/iter::position: Collecting 100 samples in estimated 5.0054 s (323k iterations)
Benchmarking [medium slice    [no null in slice]/iter::position: Analyzing
[medium slice    [no null in slice]/iter::position
                        time:   [15.424 us 15.437 us 15.451 us]
Found 17 outliers among 100 measurements (17.00%)
  5 (5.00%) low mild
  8 (8.00%) high mild
  4 (4.00%) high severe
Benchmarking [medium slice    [no null in slice]/wmemchr
Benchmarking [medium slice    [no null in slice]/wmemchr: Warming up for 3.0000 s
Benchmarking [medium slice    [no null in slice]/wmemchr: Collecting 100 samples in estimated 5.0184 s (323k iterations)
Benchmarking [medium slice    [no null in slice]/wmemchr: Analyzing
[medium slice    [no null in slice]/wmemchr
                        time:   [15.527 us 15.564 us 15.606 us]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
Benchmarking [medium slice    [no null in slice]/unrolled
Benchmarking [medium slice    [no null in slice]/unrolled: Warming up for 3.0000 s
Benchmarking [medium slice    [no null in slice]/unrolled: Collecting 100 samples in estimated 5.0231 s (520k iterations)
Benchmarking [medium slice    [no null in slice]/unrolled: Analyzing
[medium slice    [no null in slice]/unrolled
                        time:   [9.6392 us 9.6740 us 9.7123 us]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

Benchmarking medium slice     [null in the middle of slice]/iter::position
Benchmarking medium slice     [null in the middle of slice]/iter::position: Warming up for 3.0000 s
Benchmarking medium slice     [null in the middle of slice]/iter::position: Collecting 100 samples in estimated 5.0310 s (434k iterations)
Benchmarking medium slice     [null in the middle of slice]/iter::position: Analyzing
medium slice     [null in the middle of slice]/iter::position
                        time:   [11.540 us 11.563 us 11.593 us]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Benchmarking medium slice     [null in the middle of slice]/wmemchr
Benchmarking medium slice     [null in the middle of slice]/wmemchr: Warming up for 3.0000 s
Benchmarking medium slice     [null in the middle of slice]/wmemchr: Collecting 100 samples in estimated 5.0271 s (434k iterations)
Benchmarking medium slice     [null in the middle of slice]/wmemchr: Analyzing
medium slice     [null in the middle of slice]/wmemchr
                        time:   [11.563 us 11.580 us 11.602 us]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
Benchmarking medium slice     [null in the middle of slice]/unrolled
Benchmarking medium slice     [null in the middle of slice]/unrolled: Warming up for 3.0000 s
Benchmarking medium slice     [null in the middle of slice]/unrolled: Collecting 100 samples in estimated 5.0178 s (692k iterations)
Benchmarking medium slice     [null in the middle of slice]/unrolled: Analyzing
medium slice     [null in the middle of slice]/unrolled
                        time:   [7.2313 us 7.2489 us 7.2777 us]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking medium slice     [null at the end of slice]/iter::position
Benchmarking medium slice     [null at the end of slice]/iter::position: Warming up for 3.0000 s
Benchmarking medium slice     [null at the end of slice]/iter::position: Collecting 100 samples in estimated 5.0141 s (323k iterations)
Benchmarking medium slice     [null at the end of slice]/iter::position: Analyzing
medium slice     [null at the end of slice]/iter::position
                        time:   [15.472 us 15.501 us 15.539 us]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high severe
Benchmarking medium slice     [null at the end of slice]/wmemchr
Benchmarking medium slice     [null at the end of slice]/wmemchr: Warming up for 3.0000 s
Benchmarking medium slice     [null at the end of slice]/wmemchr: Collecting 100 samples in estimated 5.0022 s (323k iterations)
Benchmarking medium slice     [null at the end of slice]/wmemchr: Analyzing
medium slice     [null at the end of slice]/wmemchr
                        time:   [15.418 us 15.436 us 15.455 us]
Found 23 outliers among 100 measurements (23.00%)
  7 (7.00%) low mild
  10 (10.00%) high mild
  6 (6.00%) high severe
Benchmarking medium slice     [null at the end of slice]/unrolled
Benchmarking medium slice     [null at the end of slice]/unrolled: Warming up for 3.0000 s
Benchmarking medium slice     [null at the end of slice]/unrolled: Collecting 100 samples in estimated 5.0318 s (520k iterations)
Benchmarking medium slice     [null at the end of slice]/unrolled: Analyzing
medium slice     [null at the end of slice]/unrolled
                        time:   [9.6401 us 9.6443 us 9.6490 us]
Found 28 outliers among 100 measurements (28.00%)
  11 (11.00%) low mild
  10 (10.00%) high mild
  7 (7.00%) high severe
