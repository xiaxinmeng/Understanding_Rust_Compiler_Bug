
$ cargo benchcmp old new --threshold 2
 name                                         old ns/iter  new ns/iter  diff ns/iter   diff %  speedup 
 btree::map::find_rand_100                    26           27                      1    3.85%   x 0.96 
 btree::map::find_rand_10_000                 88           86                     -2   -2.27%   x 1.02 
 btree::map::find_seq_100                     26           25                     -1   -3.85%   x 1.04 
 btree::map::first_and_last_0                 43           42                     -1   -2.33%   x 1.02 
 btree::map::iter_20                          131          118                   -13   -9.92%   x 1.11 
 btree::set::clone_100                        1,833        1,870                  37    2.02%   x 0.98 
 btree::set::clone_100_and_clear              1,862        1,902                  40    2.15%   x 0.98 
 btree::set::clone_100_and_drain_all          3,037        2,960                 -77   -2.54%   x 1.03 
 btree::set::clone_100_and_into_iter          1,810        1,881                  71    3.92%   x 0.96 
 btree::set::clone_10k                        222,512      232,939            10,427    4.69%   x 0.96 
 btree::set::clone_10k_and_clear              226,932      235,056             8,124    3.58%   x 0.97 
 btree::set::clone_10k_and_drain_all          336,353      326,820            -9,533   -2.83%   x 1.03 
 btree::set::clone_10k_and_drain_half         334,862      324,618           -10,244   -3.06%   x 1.03 
 btree::set::clone_10k_and_into_iter          228,999      237,989             8,990    3.93%   x 0.96 
 btree::set::clone_10k_and_remove_half        615,486      600,998           -14,488   -2.35%   x 1.02 
 btree::set::difference_random_10k_vs_10k     259,846      265,218             5,372    2.07%   x 0.98 
 btree::set::intersection_100_neg_vs_100_pos  24           26                      2    8.33%   x 0.92 
 btree::set::intersection_100_pos_vs_10k_neg  28           27                     -1   -3.57%   x 1.04 
 btree::set::intersection_10k_neg_vs_100_pos  32           28                     -4  -12.50%   x 1.14 
 btree::set::intersection_10k_neg_vs_10k_pos  30           31                      1    3.33%   x 0.97 
 btree::set::intersection_10k_pos_vs_10k_neg  33           29                     -4  -12.12%   x 1.14 
 btree::set::intersection_random_100_vs_100   885          906                    21    2.37%   x 0.98 
 btree::set::intersection_random_10k_vs_100   4,383        4,029                -354   -8.08%   x 1.09 
