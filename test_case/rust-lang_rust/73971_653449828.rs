
 name                                   old ns/iter  newer ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_rand_100              17           18                        1    5.88%   x 0.94
 btree::map::find_seq_10_000            39           43                        4   10.26%   x 0.91
 btree::map::insert_rand_100            41           36                       -5  -12.20%   x 1.14
 btree::map::insert_rand_10_000         41           36                       -5  -12.20%   x 1.14
 btree::map::insert_seq_100             49           45                       -4   -8.16%   x 1.09
 btree::map::insert_seq_10_000          94           102                       8    8.51%   x 0.92
 btree::map::iter_0                     1,724        1,509                  -215  -12.47%   x 1.14
 btree::map::iter_100                   2,706        3,294                   588   21.73%   x 0.82
 btree::map::iteration_mut_1000         3,916        4,215                   299    7.64%   x 0.93
 btree::map::iteration_mut_100000       458,680      487,070              28,390    6.19%   x 0.94
 btree::map::range_included_excluded    390,220      410,035              19,815    5.08%   x 0.95
 btree::map::range_included_included    434,763      397,835             -36,928   -8.49%   x 1.09
 btree::map::range_unbounded_unbounded  28,255       36,724                8,469   29.97%   x 0.77
 btree::map::range_unbounded_vs_iter    28,810       34,102                5,292   18.37%   x 0.84
 btree::set::is_subset_100_vs_10k       1,269        1,394                   125    9.85%   x 0.91
