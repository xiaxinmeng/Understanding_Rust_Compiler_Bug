
>cargo benchcmp ol2.txt newer3.txt --threshold 5
 name                                           ol2.txt ns/iter  newer3.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::insert_seq_10_000                  98               107                            9    9.18%   x 0.92
 btree::map::iter_1000                          4,129            3,818                       -311   -7.53%   x 1.08
 btree::map::iter_mut_1000                      4,647            4,361                       -286   -6.15%   x 1.07
 btree::set::build_and_remove_all               8,705            7,991                       -714   -8.20%   x 1.09
 btree::set::intersection_random_100_vs_100     783              737                          -46   -5.87%   x 1.06
 btree::set::intersection_random_10k_vs_10k     169,476          151,990                  -17,486  -10.32%   x 1.12
 btree::set::intersection_staggered_100_vs_100  792              845                           53    6.69%   x 0.94
 btree::set::intersection_staggered_100_vs_10k  2,308            2,533                        225    9.75%   x 0.91
 btree::set::is_subset_100_vs_100               628              680                           52    8.28%   x 0.92
