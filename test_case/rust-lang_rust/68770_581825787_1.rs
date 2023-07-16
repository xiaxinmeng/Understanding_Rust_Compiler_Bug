
>cargo benchcmp old2.txt new4.txt --threshold 10
 name                                  old2.txt ns/iter  new4.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_seq_100              20                18                          -2  -10.00%   x 1.11
 btree::map::first_and_last_0          34                39                           5   14.71%   x 0.87
 btree::map::first_and_last_100        38                46                           8   21.05%   x 0.83
 btree::map::insert_rand_100           34                41                           7   20.59%   x 0.83
 btree::map::insert_rand_10_000        34                41                           7   20.59%   x 0.83
 btree::set::clone_100_and_pop_all     2,343             2,763                      420   17.93%   x 0.85
 btree::set::clone_10k_and_drain_half  537,490           388,947               -148,543  -27.64%   x 1.38
 btree::set::clone_10k_and_remove_all  501,280           561,470                 60,190   12.01%   x 0.89
