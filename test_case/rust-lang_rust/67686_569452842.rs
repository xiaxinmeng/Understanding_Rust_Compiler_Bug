
cargo benchcmp old2.txt new2.txt --threshold 5
 name                                           old2.txt ns/iter  new2.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_rand_100                      17                18                           1    5.88%   x 0.94
 btree::map::find_seq_100                       17                18                           1    5.88%   x 0.94
 btree::map::find_seq_10_000                    40                43                           3    7.50%   x 0.93
 btree::map::insert_seq_10_000                  94                99                           5    5.32%   x 0.95
 btree::map::iter_20                            40                42                           2    5.00%   x 0.95
 btree::set::difference_random_100_vs_100       738               690                        -48   -6.50%   x 1.07
 btree::set::difference_staggered_100_vs_100    765               721                        -44   -5.75%   x 1.06
 btree::set::intersection_random_100_vs_100     669               617                        -52   -7.77%   x 1.08
 btree::set::intersection_staggered_100_vs_100  716               640                        -76  -10.61%   x 1.12
 btree::set::intersection_staggered_10k_vs_10k  68,719            57,891                 -10,828  -15.76%   x 1.19
 btree::set::is_subset_100_vs_100               451               426                        -25   -5.54%   x 1.06
 btree::set::is_subset_10k_vs_10k               44,665            40,234                  -4,431   -9.92%   x 1.11
