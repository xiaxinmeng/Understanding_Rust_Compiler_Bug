
benchcmp old new --threshold 10
 name                                           old ns/iter  new ns/iter  diff ns/iter   diff %  speedup
 btree::map::clone_fat_val_100_and_into_iter    10,882       14,365              3,483   32.01%   x 0.76
 btree::map::clone_slim_100                     4,424        1,470              -2,954  -66.77%   x 3.01
 btree::map::clone_slim_10k_and_into_iter       185,360      211,358            25,998   14.03%   x 0.88
 btree::map::find_seq_100                       14           11                     -3  -21.43%   x 1.27
 btree::map::first_and_last_10k_nightly         61           68                      7   11.48%   x 0.90
 btree::map::iteration_20                       53           60                      7   13.21%   x 0.88
 btree::map::iteration_mut_1000                 3,723        4,173                 450   12.09%   x 0.89
 btree::map::iteration_mut_20                   57           67                     10   17.54%   x 0.85
 btree::set::difference_random_100_vs_10k       2,538        2,867                 329   12.96%   x 0.89
 btree::set::difference_staggered_100_vs_10k    2,208        2,724                 516   23.37%   x 0.81
 btree::set::intersection_random_100_vs_100     786          702                   -84  -10.69%   x 1.12
 btree::set::intersection_random_100_vs_10k     2,290        2,576                 286   12.49%   x 0.89
 btree::set::intersection_random_10k_vs_100     2,270        2,620                 350   15.42%   x 0.87
 btree::set::intersection_staggered_100_vs_100  779          676                  -103  -13.22%   x 1.15
 btree::set::intersection_staggered_100_vs_10k  1,984        2,484                 500   25.20%   x 0.80
 btree::set::intersection_staggered_10k_vs_10k  75,290       65,717             -9,573  -12.71%   x 1.15
 btree::set::is_subset_100_vs_100               731          643                   -88  -12.04%   x 1.14
 btree::set::is_subset_100_vs_10k               1,771        1,546                -225  -12.70%   x 1.15
 btree::set::is_subset_10k_vs_10k               72,535       64,088             -8,447  -11.65%   x 1.13
