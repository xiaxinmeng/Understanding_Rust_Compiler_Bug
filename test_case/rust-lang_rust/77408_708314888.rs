benchcmp old new --threshold 3
 name                                         old ns/iter  new ns/iter  diff ns/iter   diff %  speedup
 btree::map::clone_slim_100                   2,043        2,106                  63    3.08%   x 0.97
 btree::map::clone_slim_100_and_clear         2,551        2,065                -486  -19.05%   x 1.24
 btree::map::clone_slim_100_and_drain_all     3,891        3,441                -450  -11.57%   x 1.13
 btree::map::clone_slim_100_and_drain_half    3,351        2,844                -507  -15.13%   x 1.18
 btree::map::clone_slim_100_and_into_iter     2,587        2,045                -542  -20.95%   x 1.27
 btree::map::clone_slim_100_and_pop_all       3,684        3,189                -495  -13.44%   x 1.16
 btree::map::clone_slim_100_and_remove_all    4,515        4,044                -471  -10.43%   x 1.12
 btree::map::clone_slim_100_and_remove_half   3,388        2,823                -565  -16.68%   x 1.20
 btree::set::clone_100_and_remove_all         3,397        3,792                 395   11.63%   x 0.90
 btree::set::clone_10k_and_remove_all         400,900      436,475            35,575    8.87%   x 0.92
