
>cargo benchcmp old4.txt new4.txt --threshold 2
 name                                         old4.txt ns/iter  new4.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_rand_100                    18                17                          -1   -5.56%   x 1.06
 btree::map::first_and_last_0                 34                32                          -2   -5.88%   x 1.06
 btree::map::first_and_last_100               45                40                          -5  -11.11%   x 1.12
 btree::map::first_and_last_10k               77                64                         -13  -16.88%   x 1.20
 btree::map::insert_rand_100                  44                45                           1    2.27%   x 0.98
 btree::map::insert_seq_100                   50                51                           1    2.00%   x 0.98
 btree::map::iter_mut_1000                    4,333             3,989                     -344   -7.94%   x 1.09
 btree::map::iter_mut_100000                  489,685           467,505                -22,180   -4.53%   x 1.05
 btree::map::iter_mut_20                      82                79                          -3   -3.66%   x 1.04
 btree::map::range_excluded_excluded          459,417           444,490                -14,927   -3.25%   x 1.03
 btree::map::range_included_unbounded         1,494             1,690                      196   13.12%   x 0.88
 btree::map::range_unbounded_unbounded        2                 3                            1   50.00%   x 0.67
 btree::set::clone_100_and_drain_all          2,568             2,950                      382   14.88%   x 0.87
 btree::set::clone_100_and_drain_half         2,660             2,714                       54    2.03%   x 0.98
 btree::set::clone_100_and_into_iter          1,929             1,976                       47    2.44%   x 0.98
 btree::set::clone_100_and_pop_all            2,753             2,930                      177    6.43%   x 0.94
 btree::set::clone_100_and_remove_half        2,791             2,935                      144    5.16%   x 0.95
 btree::set::clone_10k_and_clear              209,983           203,808                 -6,175   -2.94%   x 1.03
 btree::set::clone_10k_and_drain_all          260,393           302,350                 41,957   16.11%   x 0.86
 btree::set::clone_10k_and_drain_half         298,840           292,800                 -6,040   -2.02%   x 1.02
 btree::set::clone_10k_and_pop_all            300,640           308,470                  7,830    2.60%   x 0.97
 btree::set::difference_staggered_100_vs_10k  2,385             2,475                       90    3.77%   x 0.96
 btree::set::intersection_100_neg_vs_100_pos  28                26                          -2   -7.14%   x 1.08
 btree::set::intersection_100_pos_vs_100_neg  28                27                          -1   -3.57%   x 1.04
 btree::set::intersection_10k_neg_vs_100_pos  30                31                           1    3.33%   x 0.97
 btree::set::intersection_10k_neg_vs_10k_pos  34                33                          -1   -2.94%   x 1.03
 btree::set::intersection_10k_pos_vs_100_neg  30                33                           3   10.00%   x 0.91
 btree::set::intersection_10k_pos_vs_10k_neg  33                34                           1    3.03%   x 0.97
 btree::set::intersection_random_100_vs_100   679               652                        -27   -3.98%   x 1.04
 btree::set::intersection_random_100_vs_10k   2,283             2,474                      191    8.37%   x 0.92
