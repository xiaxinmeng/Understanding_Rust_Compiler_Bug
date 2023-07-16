
c:\Users\stein\Documents\GitHub\rust>cargo benchcmp old1.txt new1.txt --threshold 3
 name                                           old1.txt ns/iter  new1.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_seq_100                       18                17                          -1   -5.56%   x 1.06
 btree::map::first_and_last_100                 37                40                           3    8.11%   x 0.92
 btree::map::insert_rand_100                    35                27                          -8  -22.86%   x 1.30
 btree::map::insert_rand_10_000                 35                27                          -8  -22.86%   x 1.30
 btree::map::insert_seq_100                     45                40                          -5  -11.11%   x 1.12
 btree::map::iter_1000                          2,764             2,666                      -98   -3.55%   x 1.04
 btree::map::iter_20                            42                40                          -2   -4.76%   x 1.05
 btree::set::difference_random_100_vs_100       720               688                        -32   -4.44%   x 1.05
 btree::set::difference_random_100_vs_10k       2,486             2,903                      417   16.77%   x 0.86
 btree::set::intersection_100_neg_vs_10k_pos    29                30                           1    3.45%   x 0.97
 btree::set::intersection_100_pos_vs_100_neg    24                25                           1    4.17%   x 0.96
 btree::set::intersection_10k_pos_vs_10k_neg    32                31                          -1   -3.12%   x 1.03
 btree::set::intersection_random_100_vs_100     637               589                        -48   -7.54%   x 1.08
 btree::set::intersection_random_100_vs_10k     2,348             2,722                      374   15.93%   x 0.86
 btree::set::intersection_random_10k_vs_100     2,294             2,477                      183    7.98%   x 0.93
 btree::set::intersection_staggered_100_vs_10k  2,322             2,133                     -189   -8.14%   x 1.09
 btree::set::is_subset_100_vs_100               378               400                         22    5.82%   x 0.95
 btree::set::is_subset_10k_vs_10k               38,394            40,790                   2,396    6.24%   x 0.94
