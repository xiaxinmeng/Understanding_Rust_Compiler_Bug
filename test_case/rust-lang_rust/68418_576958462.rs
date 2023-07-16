
>cargo benchcmp master3.txt safe3.txt --threshold 3
 name                                           master3.txt ns/iter  safe3.txt ns/iter  diff ns/iter  diff %  speedup
 btree::map::find_seq_10_000                    41                   45                            4   9.76%   x 0.91
 btree::map::first_and_last_0                   38                   42                            4  10.53%   x 0.90
 btree::map::first_and_last_100                 73                   70                           -3  -4.11%   x 1.04
 btree::map::first_and_last_10k                 87                   95                            8   9.20%   x 0.92
 btree::map::insert_seq_10_000                  99                   102                           3   3.03%   x 0.97
 btree::set::difference_random_100_vs_100       1,620                1,570                       -50  -3.09%   x 1.03
 btree::set::difference_random_100_vs_10k       2,775                2,674                      -101  -3.64%   x 1.04
 btree::set::intersection_100_neg_vs_100_pos    30                   29                           -1  -3.33%   x 1.03
 btree::set::intersection_100_pos_vs_10k_neg    36                   34                           -2  -5.56%   x 1.06
 btree::set::intersection_random_10k_vs_100     2,341                2,760                       419  17.90%   x 0.85
 btree::set::intersection_staggered_100_vs_100  791                  879                          88  11.13%   x 0.90
 btree::set::intersection_staggered_10k_vs_10k  76,247               83,194                    6,947   9.11%   x 0.92
 btree::set::is_subset_10k_vs_100               2                    3                             1  50.00%   x 0.67
