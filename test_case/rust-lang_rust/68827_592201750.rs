
cargo benchcmp old4.txt newer2.txt --threshold 5
 name                                           old4.txt ns/iter  newer2.txt ns/iter  diff ns/iter   diff %  speedup
 btree::map::find_seq_100                       20                18                            -2  -10.00%   x 1.11
 btree::map::first_and_last_100                 42                46                             4    9.52%   x 0.91
 btree::map::first_and_last_10k                 76                66                           -10  -13.16%   x 1.15
 btree::map::iter_1000                          8,988             960                       -8,028  -89.32%   x 9.36
 btree::map::iter_100000                        1,042,720         277,573                 -765,147  -73.38%   x 3.76
 btree::map::iter_20                            170               24                          -146  -85.88%   x 7.08
 btree::map::iter_mut_1000                      8,401             929                       -7,472  -88.94%   x 9.04
 btree::map::iter_mut_100000                    1,033,090         277,953                 -755,137  -73.09%   x 3.72
 btree::map::iter_mut_20                        166               21                          -145  -87.35%   x 7.90
 btree::set::clone_100                          1,812             1,623                       -189  -10.43%   x 1.12
 btree::set::clone_100_and_clear                1,844             1,585                       -259  -14.05%   x 1.16
 btree::set::clone_100_and_into_iter            1,904             1,564                       -340  -17.86%   x 1.22
 btree::set::clone_100_and_remove_all           4,567             3,869                       -698  -15.28%   x 1.18
 btree::set::clone_10k                          197,038           170,053                  -26,985  -13.70%   x 1.16
 btree::set::clone_10k_and_clear                187,530           168,030                  -19,500  -10.40%   x 1.12
 btree::set::clone_10k_and_into_iter            201,240           166,126                  -35,114  -17.45%   x 1.21
 btree::set::clone_10k_and_remove_all           502,770           459,700                  -43,070   -8.57%   x 1.09
 btree::set::difference_random_100_vs_100       1,764             617                       -1,147  -65.02%   x 2.86
 btree::set::difference_random_100_vs_10k       2,988             2,635                       -353  -11.81%   x 1.13
 btree::set::difference_random_10k_vs_100       110,336           45,705                   -64,631  -58.58%   x 2.41
 btree::set::difference_random_10k_vs_10k       244,187           156,345                  -87,842  -35.97%   x 1.56
 btree::set::difference_staggered_100_vs_100    1,780             643                       -1,137  -63.88%   x 2.77
 btree::set::difference_staggered_100_vs_10k    2,817             2,425                       -392  -13.92%   x 1.16
 btree::set::difference_staggered_10k_vs_10k    176,294           61,357                  -114,937  -65.20%   x 2.87
 btree::set::intersection_random_100_vs_100     1,537             333                       -1,204  -78.33%   x 4.62
 btree::set::intersection_random_100_vs_10k     2,723             2,262                       -461  -16.93%   x 1.20
 btree::set::intersection_random_10k_vs_100     2,837             2,382                       -455  -16.04%   x 1.19
 btree::set::intersection_random_10k_vs_10k     224,545           117,043                 -107,502  -47.88%   x 1.92
 btree::set::intersection_staggered_100_vs_100  1,440             358                       -1,082  -75.14%   x 4.02
 btree::set::intersection_staggered_100_vs_10k  2,559             2,101                       -458  -17.90%   x 1.22
 btree::set::intersection_staggered_10k_vs_10k  144,611           32,502                  -112,109  -77.52%   x 4.45
 btree::set::is_subset_100_vs_100               1,348             280                       -1,068  -79.23%   x 4.81
 btree::set::is_subset_100_vs_10k               1,751             1,276                       -475  -27.13%   x 1.37
 btree::set::is_subset_10k_vs_10k               134,618           26,777                  -107,841  -80.11%   x 5.03
