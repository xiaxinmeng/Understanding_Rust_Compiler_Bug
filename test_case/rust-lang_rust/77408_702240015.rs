 benchcmp old new --threshold 3
 name                                         old ns/iter  new ns/iter  diff ns/iter  diff %  speedup
 btree::map::clone_fat_100_and_into_iter      73,040       70,605             -2,435  -3.33%   x 1.03
 btree::map::insert_rand_10_000               33           31                     -2  -6.06%   x 1.06
 btree::map::insert_seq_10_000                96           90                     -6  -6.25%   x 1.07
 btree::map::iteration_mut_20                 69           66                     -3  -4.35%   x 1.05
 btree::set::clone_100_and_clear              1,833        1,776                 -57  -3.11%   x 1.03
 btree::set::clone_100_and_into_iter          1,820        1,764                 -56  -3.08%   x 1.03
 btree::set::clone_10k_and_remove_half        447,705      429,720           -17,985  -4.02%   x 1.04
 btree::set::intersection_random_10k_vs_100   2,153        2,085                 -68  -3.16%   x 1.03
