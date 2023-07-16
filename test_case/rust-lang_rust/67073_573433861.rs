
cargo benchcmp old.txt new.txt --threshold 5
 name                                            old.txt ns/iter  new.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_seq_10_000                   43            38                              -5  -11.63%   x 1.13
 btree::map::first_and_last_0                  14            37                              23  164.29%   x 0.38
 btree::map::first_and_last_100                39            42                               3    7.69%   x 0.93
 btree::map::first_and_last_10k                52            79                              27   51.92%   x 0.66
 btree::map::iter_20                           40            47                               7   17.50%   x 0.85
 btree::map::iter_mut_1000                     2,607         2,786                          179    6.87%   x 0.94
 btree::map::iter_mut_20                       40            47                               7   17.50%   x 0.85
 btree::set::difference_random_100_vs_100      760           709                            -51   -6.71%   x 1.07
 btree::set::difference_random_10k_vs_100      60,502        56,121                      -4,381   -7.24%   x 1.08
 btree::set::intersection_100_neg_vs_100_pos   24            26                               2    8.33%   x 0.92
 btree::set::intersection_100_neg_vs_10k_pos   29            32                               3   10.34%   x 0.91
 btree::set::intersection_100_pos_vs_100_neg   24            26                               2    8.33%   x 0.92
 btree::set::intersection_100_pos_vs_10k_neg   29            32                               3   10.34%   x 0.91
 btree::set::intersection_10k_neg_vs_100_pos   28            30                               2    7.14%   x 0.93
 btree::set::intersection_10k_neg_vs_10k_pos   30            32                               2    6.67%   x 0.94
 btree::set::intersection_10k_pos_vs_100_neg   28            30                               2    7.14%   x 0.93
 btree::set::intersection_10k_pos_vs_10k_neg   30            32                               2    6.67%   x 0.94
 btree::set::intersection_random_100_vs_10k    2,262         2,439                          177    7.82%   x 0.93
 btree::set::intersection_staggered_100_vs_100 697           635                            -62   -8.90%   x 1.10
 btree::set::intersection_staggered_10k_vs_10k 66,367        59,583                      -6,784  -10.22%   x 1.11
 btree::set::is_subset_100_vs_100              455           414                            -41   -9.01%   x 1.10
 btree::set::is_subset_10k_vs_10k              44,768        42,434                      -2,334   -5.21%   x 1.06
