
future::ready/futures   time:   [1.5667 ns 1.5712 ns 1.5762 ns]                                   
                        change: [+223.24% +232.94% +242.84%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
future::ready/async_combinators                                                                             
                        time:   [1.5699 ns 1.5743 ns 1.5793 ns]
                        change: [+279.32% +282.34% +285.20%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

future::poll_fn/futures time:   [260.26 ps 260.82 ps 261.40 ps]                                    
                        change: [-3.2265% -2.6712% -2.0788%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
Benchmarking future::poll_fn/async_combinators: Collecting 100 samples in estimated 5.0000 s (19B iterations                                                                                                            future::poll_fn/async_combinators                        
                        time:   [260.18 ps 260.91 ps 261.77 ps]
                        change: [-1.3517% -0.6763% -0.0422%] (p = 0.05 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

future::map/futures     time:   [2.1487 ns 2.1544 ns 2.1601 ns]                                 
                        change: [+296.27% +298.58% +300.54%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
future::map/async_combinators                                                                             
                        time:   [3.1330 ns 3.1393 ns 3.1458 ns]
                        change: [+233.70% +236.33% +238.68%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild

     Running target/release/deps/stream-ff2c5e87168d8d4c
stream::iter/futures    time:   [3.1216 ns 3.1275 ns 3.1341 ns]                                  
                        change: [+290.24% +293.62% +297.39%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  4 (4.00%) high severe
stream::iter/async_combinators                                                                             
                        time:   [3.1331 ns 3.1423 ns 3.1532 ns]
                        change: [+280.94% +283.80% +286.95%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe

stream::next/futures    time:   [3.3549 ns 3.4282 ns 3.5139 ns]                                  
                        change: [+319.16% +327.38% +336.10%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
stream::next/async_combinators                                                                             
                        time:   [3.1806 ns 3.2330 ns 3.2933 ns]
                        change: [+288.17% +292.97% +297.98%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe

stream::collect/futures time:   [6.3038 ns 6.3446 ns 6.3928 ns]                                     
                        change: [+298.95% +301.74% +305.36%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)
  4 (4.00%) low mild
  9 (9.00%) high mild
  4 (4.00%) high severe
Benchmarking stream::collect/async_combinators: Collecting 100 samples in estimated 5.0000 s (456M iteration                                                                                                            stream::collect/async_combinators                        
                        time:   [11.661 ns 12.051 ns 12.435 ns]
                        change: [+320.32% +329.63% +340.16%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

stream::map/futures     time:   [6.8157 ns 6.9007 ns 6.9814 ns]                                 
                        change: [+305.37% +309.91% +314.44%] (p = 0.00 < 0.05)
                        Performance has regressed.
stream::map/async_combinators                                                                             
                        time:   [7.0993 ns 7.2319 ns 7.3715 ns]
                        change: [+339.77% +348.71% +358.18%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

stream::fold/futures    time:   [4.3133 ns 4.3516 ns 4.4018 ns]                                  
                        change: [+277.61% +283.58% +291.19%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
stream::fold/async_combinators                                                                             
                        time:   [9.3900 ns 9.4145 ns 9.4431 ns]
                        change: [+259.06% +261.65% +264.17%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) high mild
  6 (6.00%) high severe
