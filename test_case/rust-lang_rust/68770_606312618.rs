
>cargo benchcmp old4.txt new4.txt --threshold 5
 name                                           old4.txt ns/iter  new4.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_seq_10_000                    44                40                          -4   -9.09%   x 1.10
 btree::map::first_and_last_0                   31                34                           3    9.68%   x 0.91
 btree::map::insert_rand_100                    35                42                           7   20.00%   x 0.83
 btree::map::insert_rand_10_000                 35                42                           7   20.00%   x 0.83
 btree::map::insert_seq_100                     47                51                           4    8.51%   x 0.92
 btree::map::iter_mut_1000                      3,943             4,352                      409   10.37%   x 0.91
 btree::map::range_included_included            394,675           415,040                 20,365    5.16%   x 0.95
 btree::map::range_included_unbounded           1,786             1,626                     -160   -8.96%   x 1.10
 btree::map::range_unbounded_unbounded          3                 2                           -1  -33.33%   x 1.50
 btree::set::clone_100_and_drain_half           3,564             2,720                     -844  -23.68%   x 1.31
 btree::set::clone_10k_and_drain_half           494,560           310,650               -183,910  -37.19%   x 1.59
 btree::set::intersection_staggered_100_vs_10k  2,137             2,294                      157    7.35%   x 0.93
WARNING: benchmarks in new but not in old: btree::set::clone_100_and_drain_all, btree::set::clone_10k_and_drain_all
