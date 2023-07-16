
benchcmp old new --threshold 5
 name                                           old ns/iter  new ns/iter  diff ns/iter   diff %  speedup
 btree::map::insert_rand_100                    41           36                     -5  -12.20%   x 1.14
 btree::map::insert_rand_10_000                 41           35                     -6  -14.63%   x 1.17
 btree::map::insert_seq_100                     50           45                     -5  -10.00%   x 1.11
 btree::map::iter_0                             1,263        1,760                 497   39.35%   x 0.72
 btree::map::range_included_included            404,113      455,713            51,600   12.77%   x 0.89
 btree::set::difference_staggered_100_vs_10k    2,309        2,527                 218    9.44%   x 0.91
 btree::set::intersection_staggered_100_vs_10k  2,171        2,343                 172    7.92%   x 0.93
 btree::set::is_subset_100_vs_10k               1,239        1,423                 184   14.85%   x 0.87
