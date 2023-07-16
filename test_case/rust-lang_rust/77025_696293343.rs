benchcmp r3 r4 --threshold 10
 name                                           r3 ns/iter  r4 ns/iter  diff ns/iter   diff %  speedup
 btree::map::clone_fat_100                      45,305      52,676             7,371   16.27%   x 0.86
 btree::map::clone_fat_100_and_clear            45,755      51,880             6,125   13.39%   x 0.88
 btree::map::clone_fat_val_100                  23,963      27,126             3,163   13.20%   x 0.88
 btree::map::clone_fat_val_100_and_clear        23,618      27,295             3,677   15.57%   x 0.87
 btree::map::clone_fat_val_100_and_into_iter    34,394      38,174             3,780   10.99%   x 0.90
 btree::map::clone_slim_10k                     231,170     207,235          -23,935  -10.35%   x 1.12
 btree::map::clone_slim_10k_and_into_iter       230,603     207,045          -23,558  -10.22%   x 1.11
 btree::map::first_and_last_10k                 68          59                    -9  -13.24%   x 1.15
 btree::map::range_included_included            473,273     418,305          -54,968  -11.61%   x 1.13
 btree::set::difference_staggered_100_vs_100    694         905                  211   30.40%   x 0.77
 btree::set::difference_staggered_10k_vs_10k    68,588      85,926            17,338   25.28%   x 0.80
 btree::set::intersection_staggered_10k_vs_10k  61,094      68,490             7,396   12.11%   x 0.89
 btree::set::is_subset_100_vs_10k               1,197       1,338                141   11.78%   x 0.89
