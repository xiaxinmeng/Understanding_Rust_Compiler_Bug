
benchcmp old new --threshold 5
 name                                           old ns/iter  new ns/iter  diff ns/iter   diff %  speedup
 btree::map::clone_fat_100_and_drain_all        135,825      123,130           -12,695   -9.35%   x 1.10
 btree::map::clone_fat_100_and_pop_all          155,275      142,557           -12,718   -8.19%   x 1.09
 btree::map::clone_fat_100_and_remove_all       190,490      179,100           -11,390   -5.98%   x 1.06
 btree::map::clone_fat_val_100_and_drain_all    48,842       56,050              7,208   14.76%   x 0.87
 btree::map::clone_fat_val_100_and_pop_all      59,313       66,690              7,377   12.44%   x 0.89
 btree::map::clone_fat_val_100_and_remove_all   66,970       74,564              7,594   11.34%   x 0.90
 btree::map::clone_slim_10k_and_remove_half     517,690      487,620           -30,070   -5.81%   x 1.06
 btree::map::find_rand_100                      13           12                     -1   -7.69%   x 1.08
 btree::map::find_rand_10_000                   60           56                     -4   -6.67%   x 1.07
 btree::map::find_seq_100                       13           12                     -1   -7.69%   x 1.08
 btree::map::find_seq_10_000                    41           36                     -5  -12.20%   x 1.14
 btree::map::first_and_last_100                 42           48                      6   14.29%   x 0.88
 btree::map::iteration_100000                   522,490      489,535           -32,955   -6.31%   x 1.07
 btree::map::iteration_mut_100000               543,440      485,065           -58,375  -10.74%   x 1.12
 btree::map::range_included_excluded            488,475      427,105           -61,370  -12.56%   x 1.14
 btree::map::range_included_unbounded           147,245      160,521            13,276    9.02%   x 0.92
 btree::set::difference_random_100_vs_10k       3,172        2,834                -338  -10.66%   x 1.12
 btree::set::difference_staggered_100_vs_10k    2,255        3,030                 775   34.37%   x 0.74
 btree::set::intersection_staggered_100_vs_10k  2,003        2,632                 629   31.40%   x 0.76
 btree::set::is_subset_100_vs_100               592          555                   -37   -6.25%   x 1.07
 btree::set::is_subset_100_vs_10k               1,589        1,718                 129    8.12%   x 0.92
 btree::set::is_subset_10k_vs_10k               58,993       64,393              5,400    9.15%   x 0.92
