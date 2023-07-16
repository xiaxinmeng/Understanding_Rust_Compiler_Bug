
 name                                   c1 ns/iter  c2 ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_rand_100              18          17                    -1   -5.56%   x 1.06
 btree::map::first_and_last_100         40          43                     3    7.50%   x 0.93
 btree::map::iter_0                     3,012       1,774             -1,238  -41.10%   x 1.70
 btree::map::iter_1                     7,016       2,255             -4,761  -67.86%   x 3.11
 btree::map::iter_100                   8,770       3,510             -5,260  -59.98%   x 2.50
 btree::map::iter_10k                   9,523       4,017             -5,506  -57.82%   x 2.37
 btree::map::iter_1m                    10,525      5,612             -4,913  -46.68%   x 1.88
 btree::map::iteration_20               86          81                    -5   -5.81%   x 1.06
 btree::map::iteration_mut_100000       490,270     464,320          -25,950   -5.29%   x 1.06
 btree::map::range_included_excluded    408,180     379,105          -29,075   -7.12%   x 1.08
 btree::map::range_included_included    431,697     407,905          -23,792   -5.51%   x 1.06
 btree::map::range_included_unbounded   116,462     105,607          -10,855   -9.32%   x 1.10
 btree::map::range_unbounded_unbounded  28,880      37,183             8,303   28.75%   x 0.78
 btree::map::range_unbounded_vs_iter    91,573      33,270           -58,303  -63.67%   x 2.75
 btree::set::clone_100_and_remove_all   4,900       4,237               -663  -13.53%   x 1.16
 btree::set::clone_10k_and_remove_all   543,730     506,790          -36,940   -6.79%   x 1.07
