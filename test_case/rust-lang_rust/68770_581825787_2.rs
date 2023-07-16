
name                                         old1-4.txt ns/iter  old1-5.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_rand_100                    19                  18                            -1   -5.26%   x 1.06
 btree::map::find_seq_100                     19                  18                            -1   -5.26%   x 1.06
 btree::map::insert_seq_10_000                107                 101                           -6   -5.61%   x 1.06
 btree::set::difference_random_100_vs_10k     3,088               2,920                       -168   -5.44%   x 1.06
 btree::set::difference_staggered_100_vs_100  2,206               1,820                       -386  -17.50%   x 1.21
 btree::set::difference_staggered_10k_vs_10k  218,622             181,240                  -37,382  -17.10%   x 1.21
 btree::set::intersection_100_neg_vs_100_pos  29                  27                            -2   -6.90%   x 1.07
