
benchcmp old new --threshold 10
 name                                           old ns/iter  new ns/iter  diff ns/iter   diff %  speedup
 btree::map::first_and_last_0                   10           12                      2   20.00%   x 0.83
 btree::map::first_and_last_10k                 60           68                      8   13.33%   x 0.88
 btree::map::iter_1                             11,139       7,022              -4,117  -36.96%   x 1.59
 btree::map::iter_100                           12,293       7,775              -4,518  -36.75%   x 1.58
 btree::map::iter_10k                           13,294       9,029              -4,265  -32.08%   x 1.47
 btree::map::iter_1m                            14,408       9,924              -4,484  -31.12%   x 1.45
 btree::map::range_included_included            418,226      503,045            84,819   20.28%   x 0.83
 btree::map::range_unbounded_vs_iter            126,138      81,175            -44,963  -35.65%   x 1.55
 btree::set::difference_random_100_vs_10k       3,179        2,443                -736  -23.15%   x 1.30
 btree::set::difference_staggered_100_vs_10k    2,236        2,743                 507   22.67%   x 0.82
 btree::set::intersection_random_100_vs_10k     2,779        2,271                -508  -18.28%   x 1.22
 btree::set::intersection_random_10k_vs_100     2,785        2,251                -534  -19.17%   x 1.24
 btree::set::intersection_staggered_100_vs_10k  2,039        2,550                 511   25.06%   x 0.80
 btree::set::is_subset_100_vs_10k               1,180        1,639                 459   38.90%   x 0.72
