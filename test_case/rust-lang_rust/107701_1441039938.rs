
$ cargo benchcmp --threshold 2 stage0_*
 name                                 stage0_1-7a98053 ns/iter  stage0_4-beb9614 ns/iter  diff ns/iter   diff %  speedup
 iter::bench_chain_partial_cmp        74,959                    63,734                         -11,225  -14.97%   x 1.18
 iter::bench_enumerate_chain_ref_sum  1,201,570                 2,087,672                      886,102   73.75%   x 0.58
 iter::bench_enumerate_chain_sum      668,359                   758,861                         90,502   13.54%   x 0.88
 iter::bench_filter_chain_ref_count   1,306,004                 1,793,094                      487,090   37.30%   x 0.73
 iter::bench_filter_map_chain_sum     1,038,151                 891,097                       -147,054  -14.16%   x 1.17
 iter::bench_flat_map_chain_ref_sum   6,476,104                 5,009,639                   -1,466,465  -22.64%   x 1.29
 iter::bench_for_each_chain_loop      1,284,855                 1,475,270                      190,415   14.82%   x 0.87
 iter::bench_for_each_chain_ref_fold  1,217,257                 1,678,344                      461,087   37.88%   x 0.73
 iter::bench_fuse_chain_ref_sum       2,132,799                 1,712,501                     -420,298  -19.71%   x 1.25
 iter::bench_inspect_chain_ref_sum    1,453,806                 1,699,276                      245,470   16.88%   x 0.86
 iter::bench_peekable_chain_ref_sum   1,233,076                 1,677,335                      444,259   36.03%   x 0.74
 iter::bench_skip_chain_ref_sum       1,459,566                 1,690,239                      230,673   15.80%   x 0.86
 iter::bench_slice_chain_ref_sum      491                       303                               -188  -38.29%   x 1.62
 iter::bench_take_while_chain_sum     449,027                   258,876                       -190,151  -42.35%   x 1.73
