
benchcmp a b --threshold 5
 name                                           a ns/iter  b ns/iter  diff ns/iter   diff %  speedup
 btree::map::clone_slim_100                     1,483      4,356             2,873  193.73%   x 0.34
 btree::map::find_seq_100                       9          10                    1   11.11%   x 0.90
 btree::map::first_and_last_0                   10         12                    2   20.00%   x 0.83
 btree::map::first_and_last_10k                 56         65                    9   16.07%   x 0.86
 btree::map::insert_rand_10_000                 38         44                    6   15.79%   x 0.86
 btree::map::iter_100                           4,292      4,523               231    5.38%   x 0.95
 btree::map::iter_10k                           5,646      6,001               355    6.29%   x 0.94
 btree::map::iteration_1000                     4,235      3,564              -671  -15.84%   x 1.19
 btree::map::iteration_20                       68         62                   -6   -8.82%   x 1.10
 btree::map::iteration_mut_1000                 4,304      4,013              -291   -6.76%   x 1.07
 btree::set::difference_random_100_vs_100       997        771                -226  -22.67%   x 1.29
 btree::set::difference_random_10k_vs_100       68,320     59,853           -8,467  -12.39%   x 1.14
 btree::set::difference_random_10k_vs_10k       218,385    191,578         -26,807  -12.28%   x 1.14
 btree::set::difference_staggered_100_vs_100    1,146      841                -305  -26.61%   x 1.36
 btree::set::difference_staggered_100_vs_10k    2,352      2,179              -173   -7.36%   x 1.08
 btree::set::difference_staggered_10k_vs_10k    113,332    82,685          -30,647  -27.04%   x 1.37
 btree::set::intersection_100_pos_vs_100_neg    16         17                    1    6.25%   x 0.94
 btree::set::intersection_10k_neg_vs_100_pos    17         18                    1    5.88%   x 0.94
 btree::set::intersection_10k_neg_vs_10k_pos    18         19                    1    5.56%   x 0.95
 btree::set::intersection_10k_pos_vs_10k_neg    18         19                    1    5.56%   x 0.95
 btree::set::intersection_random_100_vs_100     713        626                 -87  -12.20%   x 1.14
 btree::set::intersection_staggered_100_vs_100  661        617                 -44   -6.66%   x 1.07
 btree::set::intersection_staggered_10k_vs_10k  64,105     60,344           -3,761   -5.87%   x 1.06
 btree::set::is_subset_100_vs_100               533        599                  66   12.38%   x 0.89
 btree::set::is_subset_10k_vs_10k               62,611     59,448           -3,163   -5.05%   x 1.05
