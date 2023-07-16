
cargo benchcmp --threshold 4 old2.txt new2.txt
 name                                           old2.txt ns/iter  new2.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_rand_100                      17                18                           1    5.88%   x 0.94
 btree::map::find_seq_10_000                    42                38                          -4   -9.52%   x 1.11
 btree::map::first_and_last_10k                 50                52                           2    4.00%   x 0.96
 btree::map::insert_seq_100                     44                46                           2    4.55%   x 0.96
 btree::map::iter_1000                          2,885             2,659                     -226   -7.83%   x 1.08
 btree::map::iter_100000                        359,550           343,650                -15,900   -4.42%   x 1.05
 btree::map::iter_20                            39                43                           4   10.26%   x 0.91
 btree::map::iter_mut_20                        40                42                           2    5.00%   x 0.95
 btree::set::build_and_clear                    4,676             3,318                   -1,358  -29.04%   x 1.41
 btree::set::build_and_drop                     4,714             3,306                   -1,408  -29.87%   x 1.43
 btree::set::build_and_into_iter                4,731             3,306                   -1,425  -30.12%   x 1.43
 btree::set::build_and_pop_all                  5,832             5,045                     -787  -13.49%   x 1.16
 btree::set::build_and_remove_all               7,404             6,084                   -1,320  -17.83%   x 1.22
 btree::set::difference_random_10k_vs_100       54,058            57,564                   3,506    6.49%   x 0.94
 btree::set::intersection_100_neg_vs_100_pos    23                24                           1    4.35%   x 0.96
 btree::set::intersection_random_100_vs_100     604               642                         38    6.29%   x 0.94
 btree::set::intersection_random_10k_vs_100     2,360             2,256                     -104   -4.41%   x 1.05
 btree::set::intersection_staggered_100_vs_100  621               560                        -61   -9.82%   x 1.11
 btree::set::intersection_staggered_10k_vs_10k  59,895            54,085                  -5,810   -9.70%   x 1.11
 btree::set::is_subset_100_vs_100               414               436                         22    5.31%   x 0.95
 btree::set::is_subset_100_vs_10k               1,259             1,353                       94    7.47%   x 0.93
 btree::set::is_subset_10k_vs_10k               41,193            43,501                   2,308    5.60%   x 0.95
WARNING: benchmarks in new but not in old: btree::set::build_and_drain, btree::set::build_and_retain_nothing
