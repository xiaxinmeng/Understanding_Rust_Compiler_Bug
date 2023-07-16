
cargo benchcmp old3.txt new3.txt --threshold 5
 name                                           old3.txt ns/iter  new3.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_seq_10_000                    42                38                          -4   -9.52%   x 1.11
 btree::map::insert_seq_100                     45                48                           3    6.67%   x 0.94
 btree::map::iter_1000                          2,782             3,007                      225    8.09%   x 0.93
 btree::map::iter_mut_100000                    343,485           366,545                 23,060    6.71%   x 0.94
 btree::map::iter_mut_20                        39                43                           4   10.26%   x 0.91
 btree::set::build_and_remove_all               8,218             7,744                     -474   -5.77%   x 1.06
 btree::set::difference_random_100_vs_100       676               721                         45    6.66%   x 0.94
 btree::set::difference_random_100_vs_10k       2,419             2,558                      139    5.75%   x 0.95
 btree::set::difference_random_10k_vs_100       54,627            59,222                   4,595    8.41%   x 0.92
 btree::set::difference_staggered_100_vs_100    802               723                        -79   -9.85%   x 1.11
 btree::set::difference_staggered_100_vs_10k    2,266             2,387                      121    5.34%   x 0.95
 btree::set::difference_staggered_10k_vs_10k    76,828            70,304                  -6,524   -8.49%   x 1.09
 btree::set::intersection_staggered_100_vs_100  677               604                        -73  -10.78%   x 1.12
 btree::set::intersection_staggered_100_vs_10k  2,111             2,222                      111    5.26%   x 0.95
 btree::set::intersection_staggered_10k_vs_10k  67,190            54,518                 -12,672  -18.86%   x 1.23
 btree::set::is_subset_100_vs_100               403               425                         22    5.46%   x 0.95
 btree::set::is_subset_10k_vs_10k               40,746            42,860                   2,114    5.19%   x 0.95