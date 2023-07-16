
 name                                           old ns/iter  newest ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_rand_100                      17           18                         1    5.88%   x 0.94
 btree::map::first_and_last_100                 47           42                        -5  -10.64%   x 1.12
 btree::map::first_and_last_10k                 64           74                        10   15.62%   x 0.86
 btree::map::insert_rand_100                    41           36                        -5  -12.20%   x 1.14
 btree::map::insert_rand_10_000                 41           36                        -5  -12.20%   x 1.14
 btree::map::insert_seq_100                     49           45                        -4   -8.16%   x 1.09
 btree::map::insert_seq_10_000                  94           99                         5    5.32%   x 0.95
 btree::map::range_included_excluded            390,220      410,650               20,430    5.24%   x 0.95
 btree::map::range_unbounded_unbounded          28,255       37,597                 9,342   33.06%   x 0.75
 btree::set::difference_random_100_vs_10k       2,521        2,745                    224    8.89%   x 0.92
 btree::set::intersection_random_100_vs_10k     2,322        2,581                    259   11.15%   x 0.90
 btree::set::intersection_random_10k_vs_100     2,305        2,649                    344   14.92%   x 0.87
 btree::set::intersection_staggered_100_vs_10k  2,125        2,233                    108    5.08%   x 0.95
