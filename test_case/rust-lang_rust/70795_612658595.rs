
 name                                           old ns/iter  new ns/iter  diff ns/iter  diff %  speedup
 btree::map::first_and_last_100                 39           42                      3   7.69%   x 0.93
 btree::map::first_and_last_10k                 62           67                      5   8.06%   x 0.93
 btree::map::insert_rand_100                    41           43                      2   4.88%   x 0.95
 btree::map::insert_rand_10_000                 41           42                      1   2.44%   x 0.98
 btree::map::insert_seq_100                     49           50                      1   2.04%   x 0.98
 btree::map::iter_1000                          4,214        4,049                -165  -3.92%   x 1.04
 btree::map::iter_100000                        483,375      472,160           -11,215  -2.32%   x 1.02
 btree::map::iter_mut_100000                    475,045      456,035           -19,010  -4.00%   x 1.04
 btree::set::clone_100_and_drain_all            2,470        2,818                 348  14.09%   x 0.88
 btree::set::clone_100_and_pop_all              2,734        2,814                  80   2.93%   x 0.97
 btree::set::clone_10k_and_drain_all            251,810      288,665            36,855  14.64%   x 0.87
 btree::set::clone_10k_and_drain_half           291,425      283,066            -8,359  -2.87%   x 1.03
 btree::set::clone_10k_and_pop_all              286,456      296,030             9,574   3.34%   x 0.97
 btree::set::difference_random_100_vs_10k       2,550        2,473                 -77  -3.02%   x 1.03
 btree::set::intersection_100_pos_vs_100_neg    27           26                     -1  -3.70%   x 1.04
 btree::set::intersection_100_pos_vs_10k_neg    32           31                     -1  -3.12%   x 1.03
 btree::set::intersection_10k_pos_vs_10k_neg    33           32                     -1  -3.03%   x 1.03
 btree::set::intersection_staggered_100_vs_10k  2,180        2,112                 -68  -3.12%   x 1.03
