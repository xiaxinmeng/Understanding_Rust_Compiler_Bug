
Dedup/Unique num        time:   [5.8258 us 5.8452 us 5.8690 us]
                        change: [-41.080% -40.406% -39.715%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking Dedup/Duplicate num: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.9s, enable flat sampling, or reduce sample count to 60.
Dedup/Duplicate num     time:   [46.327 us 46.383 us 46.439 us]
                        change: [-4.1701% -3.5954% -2.9625%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
Benchmarking Dedup/Unique string: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.6s, enable flat sampling, or reduce sample count to 50.
Dedup/Unique string     time:   [114.63 us 115.15 us 115.74 us]
                        change: [+0.9217% +1.2801% +1.6846%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe
Dedup/With duplicate string
                        time:   [275.14 us 275.25 us 275.38 us]
                        change: [+0.1959% +0.2757% +0.3485%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Dedup/Duplicate ZSTs    time:   [1.0923 ns 1.0948 ns 1.0980 ns]
                        change: [-3.3687% +0.3632% +4.2486%] (p = 0.86 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
