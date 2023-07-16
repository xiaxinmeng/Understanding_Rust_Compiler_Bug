
$ cargo benchcmp --threshold 2 bench1-7a98053 bench4-beb9614
 name                                  bench1-7a98053 ns/iter  bench4-beb9614 ns/iter  diff ns/iter   diff %  speedup
 iter::bench_enumerate_chain_ref_sum   1,234,866               1,699,550                    464,684   37.63%   x 0.73
 iter::bench_filter_chain_ref_count    1,305,865               1,797,868                    492,003   37.68%   x 0.73
 iter::bench_filter_map_chain_sum      881,822                 1,047,292                    165,470   18.76%   x 0.84
 iter::bench_for_each_chain_loop       1,284,725               1,696,164                    411,439   32.03%   x 0.76
 iter::bench_for_each_chain_ref_fold   1,285,018               1,696,214                    411,196   32.00%   x 0.76
 iter::bench_fuse_chain_ref_sum        1,712,768               2,150,406                    437,638   25.55%   x 0.80
 iter::bench_inspect_chain_ref_sum     1,226,965               1,696,249                    469,284   38.25%   x 0.72
 iter::bench_peekable_chain_ref_sum    1,208,392               1,696,345                    487,953   40.38%   x 0.71
 iter::bench_skip_chain_ref_sum        1,284,179               1,695,163                    410,984   32.00%   x 0.76
 iter::bench_skip_chain_sum            635,490                 427,988                     -207,502  -32.65%   x 1.48
 iter::bench_slice_chain_ref_sum       483                     310                             -173  -35.82%   x 1.56
 iter::bench_slice_chain_sum           270                     362                               92   34.07%   x 0.75
 iter::bench_take_while_chain_ref_sum  914,385                 714,054                     -200,331  -21.91%   x 1.28
 iter::bench_take_while_chain_sum      235,815                 451,774                      215,959   91.58%   x 0.52
