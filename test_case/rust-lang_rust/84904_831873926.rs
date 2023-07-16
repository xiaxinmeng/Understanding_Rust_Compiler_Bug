
benchcmp old new --threshold 4
 name                                           old ns/iter  new ns/iter  diff ns/iter   diff %  speedup
 btree::map::clone_fat_val_100                  18,134       11,076             -7,058  -38.92%   x 1.64
 btree::map::clone_fat_val_100_and_clear        19,006       10,826             -8,180  -43.04%   x 1.76
 btree::map::clone_fat_val_100_and_drain_all    51,198       45,702             -5,496  -10.73%   x 1.12
 btree::map::clone_fat_val_100_and_drain_half   35,002       31,449             -3,553  -10.15%   x 1.11
 btree::map::clone_fat_val_100_and_into_iter    17,489       10,105             -7,384  -42.22%   x 1.73
 btree::map::clone_fat_val_100_and_pop_all      61,173       55,182             -5,991   -9.79%   x 1.11
 btree::map::clone_fat_val_100_and_remove_all   63,936       56,896             -7,040  -11.01%   x 1.12
 btree::map::clone_fat_val_100_and_remove_half  38,675       35,227             -3,448   -8.92%   x 1.10
 btree::map::clone_slim_100                     4,284        1,440              -2,844  -66.39%   x 2.97
 btree::map::clone_slim_10k_and_clear           175,385      183,814             8,429    4.81%   x 0.95
 btree::map::find_seq_100                       10           9                      -1  -10.00%   x 1.11
 btree::map::insert_seq_10_000                  94           98                      4    4.26%   x 0.96
 btree::map::iter_100                           5,021        4,358                -663  -13.20%   x 1.15
 btree::map::iter_10k                           6,526        5,740                -786  -12.04%   x 1.14
 btree::map::range_unbounded_vs_iter            52,395       45,447             -6,948  -13.26%   x 1.15
 btree::set::clone_100                          1,354        1,409                  55    4.06%   x 0.96
 btree::set::clone_10k                          163,558      171,604             8,046    4.92%   x 0.95
 btree::set::clone_10k_and_clear                164,792      171,844             7,052    4.28%   x 0.96
 btree::set::clone_10k_and_remove_all           416,160      399,170           -16,990   -4.08%   x 1.04
 btree::set::difference_random_10k_vs_100       72,431       68,975             -3,456   -4.77%   x 1.05
 btree::set::intersection_10k_neg_vs_10k_pos    19           18                     -1   -5.26%   x 1.06
 btree::set::is_subset_100_vs_100               556          585                    29    5.22%   x 0.95
