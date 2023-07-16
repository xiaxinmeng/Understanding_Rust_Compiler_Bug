
     Running target/release/deps/validate_utf8-6efda746d5fb1b3a
Gnuplot not found, disabling plotting
random_bytes/libcore    time:   [4.9977 ns 5.0048 ns 5.0117 ns]                                  
                        thrpt:  [ 97428 GiB/s  97562 GiB/s  97702 GiB/s]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high mild
random_bytes/lemire_sse time:   [69.970 us 69.985 us 70.000 us]                                    
                        thrpt:  [6.9755 GiB/s 6.9769 GiB/s 6.9784 GiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
random_bytes/lemire_avx time:   [62.055 us 62.084 us 62.109 us]                                    
                        thrpt:  [7.8617 GiB/s 7.8649 GiB/s 7.8685 GiB/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild
random_bytes/lemire_avx_ascii_path                                                                            
                        time:   [62.549 us 62.582 us 62.615 us]
                        thrpt:  [7.7982 GiB/s 7.8023 GiB/s 7.8064 GiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
random_bytes/range_sse  time:   [62.763 us 62.772 us 62.782 us]                                   
                        thrpt:  [7.7774 GiB/s 7.7787 GiB/s 7.7798 GiB/s]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low severe
  2 (2.00%) high mild
random_bytes/range_avx  time:   [45.737 us 45.746 us 45.755 us]                                    
                        thrpt:  [10.672 GiB/s 10.674 GiB/s 10.676 GiB/s]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

mostly_ascii/libcore    time:   [166.94 ns 167.23 ns 167.54 ns]                                 
                        thrpt:  [17.988 GiB/s 18.022 GiB/s 18.053 GiB/s]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
mostly_ascii/lemire_sse time:   [430.20 ns 430.45 ns 430.68 ns]                                    
                        thrpt:  [6.9977 GiB/s 7.0014 GiB/s 7.0054 GiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
mostly_ascii/lemire_avx time:   [382.89 ns 383.07 ns 383.25 ns]                                    
                        thrpt:  [7.8637 GiB/s 7.8673 GiB/s 7.8711 GiB/s]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe
mostly_ascii/lemire_avx_ascii_path                                                                            
                        time:   [65.208 ns 65.255 ns 65.311 ns]
                        thrpt:  [46.145 GiB/s 46.184 GiB/s 46.218 GiB/s]
Found 24 outliers among 100 measurements (24.00%)
  2 (2.00%) low severe
  12 (12.00%) low mild
  2 (2.00%) high mild
  8 (8.00%) high severe
mostly_ascii/range_sse  time:   [384.43 ns 384.64 ns 384.87 ns]                                   
                        thrpt:  [7.8307 GiB/s 7.8352 GiB/s 7.8395 GiB/s]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
mostly_ascii/range_avx  time:   [286.05 ns 286.26 ns 286.47 ns]                                   
                        thrpt:  [10.521 GiB/s 10.528 GiB/s 10.536 GiB/s]

ascii/libcore           time:   [72.975 ns 73.076 ns 73.189 ns]                          
                        thrpt:  [39.307 GiB/s 39.368 GiB/s 39.422 GiB/s]
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe
ascii/lemire_sse        time:   [423.11 ns 423.35 ns 423.62 ns]                             
                        thrpt:  [6.7912 GiB/s 6.7954 GiB/s 6.7993 GiB/s]
ascii/lemire_avx        time:   [373.82 ns 374.45 ns 375.43 ns]                             
                        thrpt:  [7.6628 GiB/s 7.6830 GiB/s 7.6958 GiB/s]
Found 10 outliers among 100 measurements (10.00%)
  9 (9.00%) low mild
  1 (1.00%) high severe
ascii/lemire_avx_ascii_path                                                                            
                        time:   [50.353 ns 50.588 ns 50.925 ns]
                        thrpt:  [56.492 GiB/s 56.869 GiB/s 57.133 GiB/s]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
ascii/range_sse         time:   [375.11 ns 375.87 ns 376.96 ns]                            
                        thrpt:  [7.6318 GiB/s 7.6538 GiB/s 7.6695 GiB/s]
Found 35 outliers among 100 measurements (35.00%)
  23 (23.00%) low severe
  8 (8.00%) high mild
  4 (4.00%) high severe
ascii/range_avx         time:   [272.39 ns 272.59 ns 272.82 ns]                            
                        thrpt:  [10.545 GiB/s 10.554 GiB/s 10.562 GiB/s]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe

utf8/libcore            time:   [9.0154 us 9.0263 us 9.0389 us]                          
                        thrpt:  [1.7096 GiB/s 1.7119 GiB/s 1.7140 GiB/s]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
utf8/lemire_sse         time:   [2.1554 us 2.1568 us 2.1581 us]                             
                        thrpt:  [7.1601 GiB/s 7.1645 GiB/s 7.1693 GiB/s]
Found 15 outliers among 100 measurements (15.00%)
  11 (11.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe
utf8/lemire_avx         time:   [1.9184 us 1.9188 us 1.9192 us]                             
                        thrpt:  [8.0515 GiB/s 8.0530 GiB/s 8.0547 GiB/s]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
utf8/lemire_avx_ascii_path                                                                             
                        time:   [1.4670 us 1.4679 us 1.4691 us]
                        thrpt:  [10.518 GiB/s 10.527 GiB/s 10.534 GiB/s]
utf8/range_sse          time:   [1.9426 us 1.9452 us 1.9491 us]                            
                        thrpt:  [7.9280 GiB/s 7.9439 GiB/s 7.9544 GiB/s]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe
utf8/range_avx          time:   [1.4656 us 1.4733 us 1.4833 us]                            
                        thrpt:  [10.418 GiB/s 10.489 GiB/s 10.544 GiB/s]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  9 (9.00%) high severe

Benchmarking all_utf8/libcore: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.7s or reduce sample count to 50
all_utf8/libcore        time:   [1.8757 ms 1.8790 ms 1.8832 ms]                              
                        thrpt:  [2.1672 GiB/s 2.1721 GiB/s 2.1759 GiB/s]
all_utf8/lemire_sse     time:   [586.47 us 586.93 us 587.47 us]                                
                        thrpt:  [6.9474 GiB/s 6.9538 GiB/s 6.9593 GiB/s]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
all_utf8/lemire_avx     time:   [517.80 us 518.97 us 521.39 us]                                
                        thrpt:  [7.8279 GiB/s 7.8644 GiB/s 7.8822 GiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
all_utf8/lemire_avx_ascii_path                                                                            
                        time:   [523.97 us 524.27 us 524.63 us]
                        thrpt:  [7.7796 GiB/s 7.7849 GiB/s 7.7894 GiB/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
all_utf8/range_sse      time:   [525.94 us 527.21 us 528.57 us]                               
                        thrpt:  [7.7216 GiB/s 7.7415 GiB/s 7.7601 GiB/s]
Found 21 outliers among 100 measurements (21.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  8 (8.00%) high mild
  11 (11.00%) high severe
all_utf8/range_avx      time:   [392.25 us 392.91 us 393.80 us]                               
                        thrpt:  [10.364 GiB/s 10.388 GiB/s 10.405 GiB/s]
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high severe

all_utf8_with_garbage/libcore                                                                             
                        time:   [3.6752 ns 3.7353 ns 3.8034 ns]
                        thrpt:  [1137275 GiB/s 1158024 GiB/s 1176952 GiB/s]
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  8 (8.00%) high severe
all_utf8_with_garbage/lemire_sse                                                                            
                        time:   [616.73 us 616.89 us 617.03 us]
                        thrpt:  [7.0103 GiB/s 7.0119 GiB/s 7.0136 GiB/s]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high severe
all_utf8_with_garbage/lemire_avx                                                                            
                        time:   [551.11 us 552.09 us 554.06 us]
                        thrpt:  [7.8070 GiB/s 7.8349 GiB/s 7.8488 GiB/s]
Found 17 outliers among 100 measurements (17.00%)
  5 (5.00%) low severe
  9 (9.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe
all_utf8_with_garbage/lemire_avx_ascii_path                                                                            
                        time:   [557.08 us 557.28 us 557.51 us]
                        thrpt:  [7.7587 GiB/s 7.7618 GiB/s 7.7646 GiB/s]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  7 (7.00%) high mild
all_utf8_with_garbage/range_sse                                                                            
                        time:   [554.79 us 555.10 us 555.42 us]
                        thrpt:  [7.7879 GiB/s 7.7924 GiB/s 7.7967 GiB/s]
all_utf8_with_garbage/range_avx                                                                            
                        time:   [417.05 us 417.38 us 417.74 us]
                        thrpt:  [10.355 GiB/s 10.364 GiB/s 10.372 GiB/s]
Found 13 outliers among 100 measurements (13.00%)
  13 (13.00%) low mild
