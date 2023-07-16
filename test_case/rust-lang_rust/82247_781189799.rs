
benchcmp old new --threshold 5
 name                                           old ns/iter  new ns/iter  diff ns/iter   diff %  speedup
 btree::map::clone_fat_100_and_remove_all       190,495      179,645           -10,850   -5.70%   x 1.06
 btree::map::clone_slim_10k_and_remove_half     517,620      487,380           -30,240   -5.84%   x 1.06
 btree::map::find_rand_10_000                   59           56                     -3   -5.08%   x 1.05
 btree::map::find_seq_100                       13           12                     -1   -7.69%   x 1.08
 btree::map::find_seq_10_000                    41           35                     -6  -14.63%   x 1.17
 btree::map::first_and_last_0                   12           10                     -2  -16.67%   x 1.20
 btree::map::first_and_last_100                 41           44                      3    7.32%   x 0.93
 btree::map::first_and_last_10k                 69           60                     -9  -13.04%   x 1.15
 btree::map::iteration_1000                     4,033        4,356                 323    8.01%   x 0.93
 btree::map::iteration_100000                   514,610      473,460           -41,150   -8.00%   x 1.09
 btree::map::iteration_mut_100000               532,180      484,570           -47,610   -8.95%   x 1.10
 btree::map::range_included_included            507,554      472,265           -35,289   -6.95%   x 1.07
 btree::set::clone_100                          1,373        1,443                  70    5.10%   x 0.95
 btree::set::clone_10k_and_remove_half          468,700      439,195           -29,505   -6.30%   x 1.07
 btree::set::difference_random_100_vs_10k       3,157        2,441                -716  -22.68%   x 1.29
 btree::set::difference_staggered_100_vs_10k    2,297        2,836                 539   23.47%   x 0.81
 btree::set::intersection_random_100_vs_10k     2,823        2,276                -547  -19.38%   x 1.24
 btree::set::intersection_random_10k_vs_100     2,860        2,238                -622  -21.75%   x 1.28
 btree::set::intersection_staggered_100_vs_10k  2,024        2,669                 645   31.87%   x 0.76
 btree::set::intersection_staggered_10k_vs_10k  71,858       76,383              4,525    6.30%   x 0.94
 btree::set::is_subset_100_vs_100               578          713                   135   23.36%   x 0.81
 btree::set::is_subset_100_vs_10k               1,559        1,212                -347  -22.26%   x 1.29
 btree::set::is_subset_10k_vs_10k               59,541       71,046             11,505   19.32%   x 0.84
