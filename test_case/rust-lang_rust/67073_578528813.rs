
>cargo benchcmp ol3.txt new3.txt --threshold 5
 name                                           ol3.txt ns/iter  new3.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_rand_100                      19               20                           1    5.26%   x 0.95
 btree::map::find_seq_100                       19               20                           1    5.26%   x 0.95
 btree::map::first_and_last_0                   39               43                           4   10.26%   x 0.91
 btree::map::iter_1000                          4,094            4,373                      279    6.81%   x 0.94
 btree::map::iter_100000                        482,150          524,020                 41,870    8.68%   x 0.92
 btree::map::iter_20                            66               79                          13   19.70%   x 0.84
 btree::map::iter_mut_1000                      4,723            4,391                     -332   -7.03%   x 1.08
 btree::set::build_and_clear                    5,037            4,537                     -500   -9.93%   x 1.11
 btree::set::build_and_drop                     4,904            4,650                     -254   -5.18%   x 1.05
 btree::set::build_and_into_iter                5,025            4,466                     -559  -11.12%   x 1.13
 btree::set::build_and_pop_all                  6,749            6,402                     -347   -5.14%   x 1.05
 btree::set::build_and_remove_all               8,368            7,471                     -897  -10.72%   x 1.12
 btree::set::difference_random_100_vs_100       1,558            1,698                      140    8.99%   x 0.92
 btree::set::difference_random_10k_vs_100       79,070           85,602                   6,532    8.26%   x 0.92
 btree::set::difference_random_10k_vs_10k       208,150          224,872                 16,722    8.03%   x 0.93
 btree::set::difference_staggered_100_vs_100    2,298            1,690                     -608  -26.46%   x 1.36
 btree::set::difference_staggered_100_vs_10k    2,506            2,835                      329   13.13%   x 0.88
 btree::set::difference_staggered_10k_vs_10k    222,852          171,746                -51,106  -22.93%   x 1.30
 btree::set::intersection_100_neg_vs_100_pos    29               32                           3   10.34%   x 0.91
 btree::set::intersection_100_neg_vs_10k_pos    34               36                           2    5.88%   x 0.94
 btree::set::intersection_100_pos_vs_10k_neg    34               36                           2    5.88%   x 0.94
 btree::set::intersection_10k_neg_vs_10k_pos    35               37                           2    5.71%   x 0.95
 btree::set::intersection_10k_pos_vs_10k_neg    35               41                           6   17.14%   x 0.85
 btree::set::intersection_random_100_vs_100     773              840                         67    8.67%   x 0.92
 btree::set::intersection_staggered_100_vs_100  791              897                        106   13.40%   x 0.88
 btree::set::intersection_staggered_100_vs_10k  2,275            2,497                      222    9.76%   x 0.91
 btree::set::intersection_staggered_10k_vs_10k  76,342           86,109                   9,767   12.79%   x 0.89
 btree::set::is_subset_100_vs_100               628              768                        140   22.29%   x 0.82
 btree::set::is_subset_10k_vs_10k               63,406           79,173                  15,767   24.87%   x 0.80
