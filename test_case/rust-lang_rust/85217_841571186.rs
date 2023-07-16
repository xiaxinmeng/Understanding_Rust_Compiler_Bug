
benchcmp a c --threshold 5
 name                                         a ns/iter  c ns/iter  diff ns/iter  diff %  speedup
 btree::map::clone_fat_val_100_and_clear      10,761     11,459              698   6.49%   x 0.94
 btree::map::clone_fat_val_100_and_into_iter  10,151     11,173            1,022  10.07%   x 0.91
 btree::map::clone_slim_100_and_into_iter     4,269      4,790               521  12.20%   x 0.89
 btree::map::clone_slim_10k_and_into_iter     183,788    242,990          59,202  32.21%   x 0.76
 btree::map::first_and_last_0                 10         12                    2  20.00%   x 0.83
 btree::map::first_and_last_10k               56         67                   11  19.64%   x 0.84
 btree::map::insert_rand_10_000               38         42                    4  10.53%   x 0.90
 btree::map::iteration_mut_1000               4,304      4,576               272   6.32%   x 0.94
 btree::map::iteration_mut_100000             534,390    603,730          69,340  12.98%   x 0.89
 btree::map::iteration_mut_20                 68         77                    9  13.24%   x 0.88
 btree::set::clone_100_and_into_iter          1,413      1,933               520  36.80%   x 0.73
 btree::set::clone_10k                        159,812    173,932          14,120   8.84%   x 0.92
 btree::set::clone_10k_and_into_iter          168,626    227,017          58,391  34.63%   x 0.74
 btree::set::difference_random_10k_vs_100     68,320     73,951            5,631   8.24%   x 0.92
 btree::set::is_subset_100_vs_100             533        630                  97  18.20%   x 0.85
 btree::set::is_subset_100_vs_10k             1,185      1,258                73   6.16%   x 0.94
