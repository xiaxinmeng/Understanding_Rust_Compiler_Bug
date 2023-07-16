benchcmp old new --threshold 5
 name                                        old ns/iter  new ns/iter  diff ns/iter   diff %  speedup
 btree::map::clone_slim_100_and_clear        2,526        2,020                -506  -20.03%   x 1.25
 btree::map::clone_slim_100_and_drain_all    3,859        3,345                -514  -13.32%   x 1.15
 btree::map::clone_slim_100_and_drain_half   3,293        2,773                -520  -15.79%   x 1.19
 btree::map::clone_slim_100_and_into_iter    2,575        1,953                -622  -24.16%   x 1.32
 btree::map::clone_slim_100_and_pop_all      3,669        3,130                -539  -14.69%   x 1.17
 btree::map::clone_slim_100_and_remove_all   4,496        3,983                -513  -11.41%   x 1.13
 btree::map::clone_slim_100_and_remove_half  3,346        2,797                -549  -16.41%   x 1.20
 btree::map::find_seq_10_000                 38           40                      2    5.26%   x 0.95
 btree::map::range_included_excluded         422,740      449,350            26,610    6.29%   x 0.94
 btree::map::range_included_included         426,430      464,617            38,187    8.96%   x 0.92
 btree::set::is_subset_100_vs_10k            1,164        1,242                  78    6.70%   x 0.94
