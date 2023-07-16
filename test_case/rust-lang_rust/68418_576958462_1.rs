
>cargo benchcmp master3.txt assert3.txt --threshold 5
 name                                           master3.txt ns/iter  assert3.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_seq_100                       19                   20                              1    5.26%   x 0.95
 btree::map::first_and_last_0                   38                   16                            -22  -57.89%   x 2.38
 btree::map::first_and_last_100                 73                   157                            84  115.07%   x 0.46
 btree::map::first_and_last_10k                 87                   168                            81   93.10%   x 0.52
 btree::map::iter_1000                          3,968                9,188                       5,220  131.55%   x 0.43
 btree::map::iter_100000                        477,620              962,420                   484,800  101.50%   x 0.50
 btree::map::iter_20                            68                   188                           120  176.47%   x 0.36
 btree::map::iter_mut_100000                    530,520              566,330                    35,810    6.75%   x 0.94
 btree::set::build_and_clear                    4,983                5,290                         307    6.16%   x 0.94
 btree::set::build_and_into_iter                4,972                5,359                         387    7.78%   x 0.93
 btree::set::build_and_pop_all                  6,763                7,120                         357    5.28%   x 0.95
 btree::set::difference_random_100_vs_10k       2,775                2,922                         147    5.30%   x 0.95
 btree::set::difference_random_10k_vs_100       79,029               83,015                      3,986    5.04%   x 0.95
 btree::set::difference_staggered_100_vs_10k    2,537                2,766                         229    9.03%   x 0.92
 btree::set::intersection_100_neg_vs_100_pos    30                   33                              3   10.00%   x 0.91
 btree::set::intersection_100_pos_vs_100_neg    30                   32                              2    6.67%   x 0.94
 btree::set::intersection_10k_neg_vs_100_pos    34                   36                              2    5.88%   x 0.94
 btree::set::intersection_10k_pos_vs_100_neg    34                   36                              2    5.88%   x 0.94
 btree::set::intersection_10k_pos_vs_10k_neg    35                   37                              2    5.71%   x 0.95
 btree::set::intersection_random_10k_vs_100     2,341                2,635                         294   12.56%   x 0.89
 btree::set::intersection_staggered_100_vs_10k  2,332                2,545                         213    9.13%   x 0.92
 btree::set::is_subset_100_vs_10k               1,923                2,150                         227   11.80%   x 0.89
