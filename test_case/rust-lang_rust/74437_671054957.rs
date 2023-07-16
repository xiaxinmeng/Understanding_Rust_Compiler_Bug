
benchcmp old new --threshold 10
 name                                           old ns/iter  new ns/iter  diff ns/iter   diff %  speedup
 btree::map::clone_fat_100_and_remove_all       180,600      208,270            27,670   15.32%   x 0.87
 btree::map::clone_fat_100_and_remove_half      122,685      138,852            16,167   13.18%   x 0.88
 btree::map::clone_fat_val_100_and_remove_all   80,896       96,982             16,086   19.88%   x 0.83
 btree::map::clone_fat_val_100_and_remove_half  56,726       64,687              7,961   14.03%   x 0.88
 btree::map::clone_slim_100_and_drain_all       4,111        3,389                -722  -17.56%   x 1.21
 btree::map::clone_slim_100_and_drain_half      3,334        2,837                -497  -14.91%   x 1.18
 btree::map::clone_slim_10k_and_drain_half      381,785      335,345           -46,440  -12.16%   x 1.14
 btree::map::find_seq_100                       18           16                     -2  -11.11%   x 1.12
 btree::map::first_and_last_0                   34           10                    -24  -70.59%   x 3.40
 btree::map::first_and_last_100                 51           36                    -15  -29.41%   x 1.42
 btree::map::first_and_last_10k                 69           58                    -11  -15.94%   x 1.19
 btree::map::iter_1                             2,220        1,756                -464  -20.90%   x 1.26
 btree::map::iter_100                           3,019        3,432                 413   13.68%   x 0.88
 btree::set::clone_100_and_drain_half           2,934        2,434                -500  -17.04%   x 1.21
 btree::set::clone_10k_and_drain_half           329,685      279,106           -50,579  -15.34%   x 1.18
 btree::set::difference_staggered_100_vs_10k    2,398        2,117                -281  -11.72%   x 1.13
 btree::set::intersection_100_neg_vs_100_pos    26           16                    -10  -38.46%   x 1.62
 btree::set::intersection_100_neg_vs_10k_pos    31           17                    -14  -45.16%   x 1.82
 btree::set::intersection_100_pos_vs_100_neg    26           16                    -10  -38.46%   x 1.62
 btree::set::intersection_100_pos_vs_10k_neg    31           16                    -15  -48.39%   x 1.94
 btree::set::intersection_10k_neg_vs_100_pos    30           17                    -13  -43.33%   x 1.76
 btree::set::intersection_10k_neg_vs_10k_pos    32           18                    -14  -43.75%   x 1.78
 btree::set::intersection_10k_pos_vs_100_neg    29           17                    -12  -41.38%   x 1.71
 btree::set::intersection_10k_pos_vs_10k_neg    32           17                    -15  -46.88%   x 1.88
 btree::set::is_subset_100_vs_10k               1,300        1,167                -133  -10.23%   x 1.11
