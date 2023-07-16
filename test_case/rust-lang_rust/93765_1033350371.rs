
sort_small_random       time:   [144.60 ns 144.96 ns 145.36 ns]                              
                        change: [-1.1857% -0.7782% -0.4072%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

sort_small_ascending    time:   [35.500 ns 35.614 ns 35.736 ns]                                  
                        change: [-9.6611% -9.0568% -8.2390%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe

sort_small_descending   time:   [29.978 ns 30.041 ns 30.101 ns]                                   
                        change: [-6.8975% -6.6595% -6.4402%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low mild
  7 (7.00%) high mild
  4 (4.00%) high severe

sort_small_big_random   time:   [262.48 ns 263.47 ns 264.54 ns]                                  
                        change: [-0.1157% +0.2622% +0.6097%] (p = 0.17 > 0.05)
                        No change in performance detected.
Found 35 outliers among 100 measurements (35.00%)
  9 (9.00%) low severe
  3 (3.00%) low mild
  7 (7.00%) high mild
  16 (16.00%) high severe

Benchmarking sort_small_big_ascending: Collecting 100 samples in estimated 5.0004 s (24M iterations                                                                                                   sort_small_big_ascending                        
                        time:   [207.50 ns 208.16 ns 208.85 ns]
                        change: [-3.7178% -2.8530% -2.0372%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high severe

Benchmarking sort_small_big_descending: Collecting 100 samples in estimated 5.0008 s (31M iteration                                                                                                   sort_small_big_descending                        
                        time:   [163.15 ns 163.25 ns 163.34 ns]
                        change: [+0.6363% +1.4359% +2.0507%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) low severe
  2 (2.00%) low mild

sort_medium_random      time:   [2.9320 us 2.9336 us 2.9352 us]                                
                        change: [-4.7123% -4.4968% -4.2811%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 24 outliers among 100 measurements (24.00%)
  1 (1.00%) low severe
  16 (16.00%) low mild
  1 (1.00%) high mild
  6 (6.00%) high severe

sort_medium_ascending   time:   [592.44 ns 593.33 ns 594.34 ns]                                   
                        change: [-26.795% -26.598% -26.407%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

sort_medium_descending  time:   [506.31 ns 507.14 ns 508.29 ns]                                    
                        change: [-25.662% -24.674% -23.639%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  5 (5.00%) high severe

sort_large_random       time:   [539.69 us 540.42 us 541.23 us]                              
                        change: [-5.1022% -4.7602% -4.4391%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

sort_large_ascending    time:   [326.84 us 327.91 us 329.21 us]                                 
                        change: [-7.0931% -6.7599% -6.3780%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

sort_large_descending   time:   [338.89 us 340.89 us 342.66 us]                                  
                        change: [-6.5248% -6.0734% -5.6059%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 20 outliers among 100 measurements (20.00%)
  19 (19.00%) high mild
  1 (1.00%) high severe

Benchmarking sort_large_mostly_ascending: Collecting 100 samples in estimated 5.3313 s (15k iterati                                                                                                   sort_large_mostly_ascending                        
                        time:   [351.16 us 351.54 us 351.94 us]
                        change: [-4.3671% -4.2394% -4.1178%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking sort_large_mostly_descending: Collecting 100 samples in estimated 5.4411 s (15k iterat                                                                                                   sort_large_mostly_descending                        
                        time:   [358.32 us 358.54 us 358.82 us]
                        change: [-7.7897% -7.6157% -7.4594%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) low mild
  7 (7.00%) high mild
  2 (2.00%) high severe

Benchmarking sort_large_big_random: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.0s, enable flat sampling, or reduce sample count to 60.
sort_large_big_random   time:   [1.1880 ms 1.1898 ms 1.1919 ms]                                   
                        change: [-2.0795% -1.8401% -1.6394%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 19 outliers among 100 measurements (19.00%)
  6 (6.00%) low mild
  3 (3.00%) high mild
  10 (10.00%) high severe

Benchmarking sort_large_big_ascending: Collecting 100 samples in estimated 8.8968 s (10k iterations                                                                                                   sort_large_big_ascending                        
                        time:   [878.54 us 880.05 us 882.04 us]
                        change: [-0.5682% +0.2753% +0.8210%] (p = 0.57 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

Benchmarking sort_large_big_descending: Collecting 100 samples in estimated 8.5611 s (10k iteration                                                                                                   sort_large_big_descending                        
                        time:   [840.28 us 841.62 us 843.39 us]
                        change: [-0.2935% -0.0882% +0.1958%] (p = 0.51 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
