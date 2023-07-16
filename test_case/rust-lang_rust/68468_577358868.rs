
>cargo benchcmp mastr2.txt middle2.txt --threshold 3
 name                                           mastr2.txt ns/iter  middle2.txt ns/iter  diff ns/iter  diff %  speedup
 btree::map::find_rand_100                      18                  17                             -1  -5.56%   x 1.06
 btree::map::find_rand_10_000                   58                  56                             -2  -3.45%   x 1.04
 btree::map::insert_seq_100                     45                  47                              2   4.44%   x 0.96
 btree::map::iter_1000                          2,641               2,880                         239   9.05%   x 0.92
 btree::map::iter_mut_1000                      2,705               2,823                         118   4.36%   x 0.96
 btree::set::build_and_pop_all                  6,065               5,828                        -237  -3.91%   x 1.04
 btree::set::difference_staggered_100_vs_100    855                 795                           -60  -7.02%   x 1.08
 btree::set::difference_staggered_100_vs_10k    2,405               2,311                         -94  -3.91%   x 1.04
 btree::set::difference_staggered_10k_vs_10k    78,083              75,095                     -2,988  -3.83%   x 1.04
 btree::set::intersection_100_neg_vs_100_pos    25                  24                             -1  -4.00%   x 1.04
 btree::set::intersection_10k_neg_vs_10k_pos    30                  31                              1   3.33%   x 0.97
 btree::set::intersection_random_100_vs_100     591                 646                            55   9.31%   x 0.91
 btree::set::intersection_random_100_vs_10k     2,251               2,361                         110   4.89%   x 0.95
 btree::set::intersection_random_10k_vs_100     2,276               2,362                          86   3.78%   x 0.96
 btree::set::intersection_staggered_100_vs_100  652                 717                            65   9.97%   x 0.91
 btree::set::intersection_staggered_100_vs_10k  2,075               2,201                         126   6.07%   x 0.94
 btree::set::intersection_staggered_10k_vs_10k  63,366              67,628                      4,262   6.73%   x 0.94
 btree::set::is_subset_100_vs_100               426                 459                            33   7.75%   x 0.93
 btree::set::is_subset_100_vs_10k               1,363               1,458                          95   6.97%   x 0.93
 btree::set::is_subset_10k_vs_10k               40,384              45,252                      4,868  12.05%   x 0.89
