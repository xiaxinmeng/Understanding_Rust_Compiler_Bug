
cargo bench --features merge symmdiff >bench_symmdiff_alone.txt
cargo bench --features merge,diff,intersect symmdiff >bench_symmdiff_crowd.txt
cargo benchcmp symmdiff_old symmdiff_new bench_symmdiff_crowd.txt
 name                         symmdiff_old ns/iter  symmdiff_new ns/iter  diff ns/iter  diff %  speedup
 ::parted_100_neg_vs_100_pos  800                   558                           -242  -30.25%   x 1.43
 ::parted_100_neg_vs_10k_pos  36,280                26,173                     -10,107  -27.86%   x 1.39
 ::parted_100_pos_vs_100_neg  839                   561                           -278  -33.13%   x 1.50
 ::parted_100_pos_vs_10k_neg  43,766                26,994                     -16,772  -38.32%   x 1.62
 ::parted_10k_neg_vs_100_pos  43,648                26,983                     -16,665  -38.18%   x 1.62
 ::parted_10k_neg_vs_10k_pos  79,831                52,857                     -26,974  -33.79%   x 1.51
 ::parted_10k_pos_vs_100_neg  39,621                28,233                     -11,388  -28.74%   x 1.40
 ::parted_10k_pos_vs_10k_neg  82,888                55,308                     -27,580  -33.27%   x 1.50
 ::random_100_vs_100          995                   575                           -420  -42.21%   x 1.73
 ::random_100_vs_10k          55,121                36,346                     -18,775  -34.06%   x 1.52
 ::random_100_vs_1600         9,640                 5,902                       -3,738  -38.78%   x 1.63
 ::random_10k_vs_10k          201,682               151,113                    -50,569  -25.07%   x 1.33
 ::random_10k_vs_160k         1,093,440             820,760                   -272,680  -24.94%   x 1.33
 ::subset_100_vs_10k          39,484                26,208                     -13,276  -33.62%   x 1.51
 ::subset_10_vs_100           411                   297                           -114  -27.74%   x 1.38
