
benchcmp o o1 --threshold 5
 name                                           o ns/iter  o1 ns/iter  diff ns/iter  diff %  speedup
 btree::map::clone_fat_100_and_drain_all        172,930    155,862          -17,068  -9.87%   x 1.11
 btree::map::clone_fat_100_and_remove_all       173,350    218,235           44,885  25.89%   x 0.79
 btree::map::clone_fat_100_and_remove_half      118,970    138,497           19,527  16.41%   x 0.86
 btree::map::clone_fat_val_100_and_remove_all   81,776     96,849            15,073  18.43%   x 0.84
 btree::map::clone_fat_val_100_and_remove_half  57,137     64,924             7,787  13.63%   x 0.88
 btree::map::clone_slim_100_and_clear           2,258      2,137               -121  -5.36%   x 1.06
 btree::map::clone_slim_100_and_remove_all      4,873      5,172                299   6.14%   x 0.94
 btree::map::first_and_last_0                   30         32                     2   6.67%   x 0.94
 btree::map::insert_rand_100                    41         44                     3   7.32%   x 0.93
 btree::map::insert_rand_10_000                 41         45                     4   9.76%   x 0.91
 btree::map::insert_seq_10_000                  95         100                    5   5.26%   x 0.95
 btree::map::iter_0                             1,485      1,734                249  16.77%   x 0.86
 btree::set::is_subset_100_vs_10k               1,223      1,292                 69   5.64%   x 0.95
