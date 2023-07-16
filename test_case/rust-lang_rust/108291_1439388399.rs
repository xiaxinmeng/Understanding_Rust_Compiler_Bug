diff
gh-chenyukang@dev-desktop-us-2:~/2rust$ cargo benchcmp i1.perf i2.perf
 name                                      i1.perf ns/iter  i2.perf ns/iter  diff ns/iter  diff %  speedup
 num::dec2flt::bench_0                     50               62                         12  24.00%   x 0.81
 num::dec2flt::bench_1e150                 51               61                         10  19.61%   x 0.84
 num::dec2flt::bench_42                    36               47                         11  30.56%   x 0.77
 num::dec2flt::bench_huge_int              190              196                         6   3.16%   x 0.97
 num::dec2flt::bench_long_decimal_and_exp  206              212                         6   2.91%   x 0.97
 num::dec2flt::bench_max                   74               84                         10  13.51%   x 0.88
 num::dec2flt::bench_min_normal            75               85                         10  13.33%   x 0.88
 num::dec2flt::bench_min_subnormal         52               61                          9  17.31%   x 0.85
 num::dec2flt::bench_pi_long               168              172                         4   2.38%   x 0.98
 num::dec2flt::bench_pi_short              62               77                         15  24.19%   x 0.81
 num::dec2flt::bench_short_decimal         56               71                         15  26.79%   x 0.79
 num::flt2dec::bench_big_shortest          213              213                         0   0.00%   x 1.00
 num::flt2dec::bench_small_shortest        152              153                         1   0.66%   x 0.99
