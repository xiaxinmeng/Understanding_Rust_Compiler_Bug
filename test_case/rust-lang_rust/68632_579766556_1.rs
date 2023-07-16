
Benchmarking max_subarray([..]) N = 1000000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 17.1s or reduce sample count to 30.
max_subarray([..]) N = 1000000
                        time:   [3.3745 ms 3.3775 ms 3.3808 ms]
                        change: [+1.8443% +2.0076% +2.1392%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

Benchmarking max_subarray([..]) N = 1000000 #2: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 13.4s or reduce sample count to 40.
max_subarray([..]) N = 1000000 #2
                        time:   [2.6242 ms 2.6262 ms 2.6286 ms]
                        change: [-20.425% -20.281% -20.087%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
