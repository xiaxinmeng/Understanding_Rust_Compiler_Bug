
benchcmp old new --threshold 5
 name                                           old ns/iter  new ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_rand_100                      18           17                     -1   -5.56%   x 1.06
 btree::map::find_seq_100                       18           17                     -1   -5.56%   x 1.06
 btree::map::find_seq_10_000                    40           42                      2    5.00%   x 0.95
 btree::map::first_and_last_0                   32           35                      3    9.38%   x 0.91
 btree::map::first_and_last_100                 46           50                      4    8.70%   x 0.92
 btree::map::insert_rand_100                    42           39                     -3   -7.14%   x 1.08
 btree::map::insert_rand_10_000                 42           39                     -3   -7.14%   x 1.08
 btree::map::insert_seq_100                     50           47                     -3   -6.00%   x 1.06
 btree::map::iter_0                             1,580        1,976                 396   25.06%   x 0.80
 btree::map::iter_1                             2,257        1,817                -440  -19.49%   x 1.24
 btree::map::iteration_1000                     4,114        4,351                 237    5.76%   x 0.95
 btree::map::range_unbounded_unbounded          28,935       36,986              8,051   27.82%   x 0.78
 btree::set::clone_100                          1,908        1,775                -133   -6.97%   x 1.07
 btree::set::clone_100_and_clear                1,904        1,773                -131   -6.88%   x 1.07
 btree::set::clone_100_and_remove_half          2,804        2,663                -141   -5.03%   x 1.05
 btree::set::clone_10k                          202,421      188,552           -13,869   -6.85%   x 1.07
 btree::set::clone_10k_and_clear                203,383      190,601           -12,782   -6.28%   x 1.07
 btree::set::difference_random_100_vs_10k       2,500        2,309                -191   -7.64%   x 1.08
 btree::set::difference_staggered_100_vs_100    728          683                   -45   -6.18%   x 1.07
 btree::set::difference_staggered_100_vs_10k    2,407        2,140                -267  -11.09%   x 1.12
 btree::set::intersection_100_neg_vs_100_pos    26           17                     -9  -34.62%   x 1.53
 btree::set::intersection_100_neg_vs_10k_pos    31           18                    -13  -41.94%   x 1.72
 btree::set::intersection_100_pos_vs_100_neg    26           17                     -9  -34.62%   x 1.53
 btree::set::intersection_100_pos_vs_10k_neg    32           18                    -14  -43.75%   x 1.78
 btree::set::intersection_10k_neg_vs_100_pos    30           18                    -12  -40.00%   x 1.67
 btree::set::intersection_10k_neg_vs_10k_pos    33           19                    -14  -42.42%   x 1.74
 btree::set::intersection_10k_pos_vs_100_neg    30           18                    -12  -40.00%   x 1.67
 btree::set::intersection_10k_pos_vs_10k_neg    33           19                    -14  -42.42%   x 1.74
 btree::set::intersection_random_100_vs_10k     2,355        2,137                -218   -9.26%   x 1.10
 btree::set::intersection_random_10k_vs_100     2,325        2,122                -203   -8.73%   x 1.10
 btree::set::intersection_staggered_100_vs_10k  2,205        1,979                -226  -10.25%   x 1.11
 btree::set::is_subset_100_vs_10k               1,239        1,123                -116   -9.36%   x 1.10
