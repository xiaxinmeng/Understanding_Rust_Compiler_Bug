
benchcmp r0 r4qua --threshold 10
 name                                   r0 ns/iter  r4qua ns/iter  diff ns/iter   diff %  speedup
 btree::map::iter_0                     1,760       1,509                  -251  -14.26%   x 1.17
 btree::map::iter_100                   2,722       3,671                   949   34.86%   x 0.74
 btree::map::iter_10k                   3,737       4,269                   532   14.24%   x 0.88
 btree::map::range_unbounded_unbounded  28,928      37,173                8,245   28.50%   x 0.78
 btree::map::range_unbounded_vs_iter    28,808      38,526                9,718   33.73%   x 0.75
