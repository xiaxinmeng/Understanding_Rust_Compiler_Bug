
 name                                            control ns/iter        variable ns/iter        diff ns/iter   diff %  speedup 
 slice::binary_search_l1                         55                     55                                 0    0.00%   x 1.00 
 slice::binary_search_l1_with_dups               43                     51                                 8   18.60%   x 0.84 
 slice::binary_search_l2                         73                     73                                 0    0.00%   x 1.00 
 slice::binary_search_l2_with_dups               73                     74                                 1    1.37%   x 0.99 
 slice::binary_search_l3                         162                    169                                7    4.32%   x 0.96 
 slice::binary_search_l3_with_dups               161                    164                                3    1.86%   x 0.98 
 slice::concat                                   1,114                  1,120                              6    0.54%   x 0.99 
 slice::contains_last_element                    28                     28                                 0    0.00%   x 1.00 
 slice::ends_with_diff_one_element_at_beginning  2                      2                                  0    0.00%   x 1.00 
 slice::ends_with_same_vector                    0                      0                                  0     NaN%    x NaN 
 slice::ends_with_single_element                 0                      0                                  0     NaN%    x NaN 
 slice::iter_nth_usize                           948                    940                               -8   -0.84%   x 1.01 
 slice::iter_nth_zst                             719                    594                             -125  -17.39%   x 1.21 
 slice::iter_usize                               292                    292                                0    0.00%   x 1.00 
 slice::iter_zst                                 286                    385                               99   34.62%   x 0.74 
 slice::iterator                                 7                      7                                  0    0.00%   x 1.00 
 slice::join                                     1,218                  1,180                            -38   -3.12%   x 1.03 
 slice::mut_iterator                             8                      8                                  0    0.00%   x 1.00 
 slice::push                                     2                      2                                  0    0.00%   x 1.00 
 slice::random_inserts                           3,383                  3,431                             48    1.42%   x 0.99 
 slice::random_removes                           3,262                  3,310                             48    1.47%   x 0.99 
 slice::reverse_simd_f64x4                       31,368 (33428 MB/s)    34,479 (30411 MB/s)            3,111    9.92%   x 0.91 
 slice::reverse_u128                             30,879 (33957 MB/s)    30,486 (34395 MB/s)             -393   -1.27%   x 1.01 
 slice::reverse_u16                              85,969 (12197 MB/s)    86,840 (12074 MB/s)              871    1.01%   x 0.99 
 slice::reverse_u32                              84,347 (12431 MB/s)    85,098 (12321 MB/s)              751    0.89%   x 0.99 
 slice::reverse_u64                              47,860 (21909 MB/s)    48,253 (21730 MB/s)              393    0.82%   x 0.99 
 slice::reverse_u8                               52,618 (19928 MB/s)    52,066 (20139 MB/s)             -552   -1.05%   x 1.01 
 slice::reverse_u8x3                             206,757 (5071 MB/s)    215,208 (4872 MB/s)            8,451    4.09%   x 0.96 
 slice::rotate_huge_by1                          5,354,006 (7833 MB/s)  6,628,860 (6327 MB/s)      1,274,854   23.81%   x 0.81 
 slice::rotate_huge_by1234577_big                9,123,162 (4597 MB/s)  9,521,004 (4405 MB/s)        397,842    4.36%   x 0.96 
 slice::rotate_huge_by1234577_bytes              8,817,380 (4756 MB/s)  11,642,391 (3602 MB/s)     2,825,011   32.04%   x 0.76 
 slice::rotate_huge_by1234577_strings            9,354,997 (4483 MB/s)  10,555,414 (3973 MB/s)     1,200,417   12.83%   x 0.89 
 slice::rotate_huge_by1234577_u64                8,662,283 (4842 MB/s)  9,364,781 (4478 MB/s)        702,498    8.11%   x 0.92 
 slice::rotate_huge_by9199_big                   5,292,371 (7925 MB/s)  6,695,713 (6264 MB/s)      1,403,342   26.52%   x 0.79 
 slice::rotate_huge_by9199_bytes                 5,302,024 (7910 MB/s)  6,362,843 (6591 MB/s)      1,060,819   20.01%   x 0.83 
 slice::rotate_huge_by9199_strings               5,282,433 (7940 MB/s)  5,565,033 (7536 MB/s)        282,600    5.35%   x 0.95 
 slice::rotate_huge_by9199_u64                   5,439,123 (7711 MB/s)  5,337,006 (7858 MB/s)       -102,117   -1.88%   x 1.02 
 slice::rotate_huge_half                         5,908,147 (7099 MB/s)  5,312,889 (7894 MB/s)       -595,258  -10.08%   x 1.11 
 slice::rotate_huge_half_plus_one                8,336,232 (5031 MB/s)  7,743,591 (5416 MB/s)       -592,641   -7.11%   x 1.08 
 slice::rotate_medium_by1                        1,125 (65123 MB/s)     1,405 (52145 MB/s)               280   24.89%   x 0.80 
 slice::rotate_medium_by727_bytes                3,270 (22404 MB/s)     3,506 (20896 MB/s)               236    7.22%   x 0.93 
 slice::rotate_medium_by727_strings              3,226 (22705 MB/s)     3,322 (22049 MB/s)                96    2.98%   x 0.97 
 slice::rotate_medium_by727_u64                  3,201 (22887 MB/s)     3,278 (22350 MB/s)                77    2.41%   x 0.98 
 slice::rotate_medium_half                       2,231 (32839 MB/s)     2,279 (32147 MB/s)                48    2.15%   x 0.98 
 slice::rotate_medium_half_plus_one              2,380 (30783 MB/s)     2,367 (30952 MB/s)               -13   -0.55%   x 1.01 
 slice::rotate_tiny_by1                          17 (7529 MB/s)         17 (7529 MB/s)                     0    0.00%   x 1.00 
 slice::rotate_tiny_half                         16 (8000 MB/s)         17 (7529 MB/s)                     1    6.25%   x 0.94 
 slice::rotate_tiny_half_plus_one                17 (7529 MB/s)         18 (7111 MB/s)                     1    5.88%   x 0.94 
 slice::sort_by_cached_key_lexicographic         2,422,180 (33 MB/s)    2,626,632 (30 MB/s)          204,452    8.44%   x 0.92 
 slice::sort_by_key_lexicographic                16,280,240 (4 MB/s)    16,811,305 (4 MB/s)          531,065    3.26%   x 0.97 
 slice::sort_large_ascending                     6,036 (13253 MB/s)     7,437 (10757 MB/s)             1,401   23.21%   x 0.81 
 slice::sort_large_big                           882,470 (1450 MB/s)    897,715 (1425 MB/s)           15,245    1.73%   x 0.98 
 slice::sort_large_descending                    9,649 (8291 MB/s)      9,704 (8244 MB/s)                 55    0.57%   x 0.99 
 slice::sort_large_expensive                     20,612,173 (3 MB/s)    20,852,778 (3 MB/s)          240,605    1.17%   x 0.99 
 slice::sort_large_mostly_ascending              139,018 (575 MB/s)     131,333 (609 MB/s)            -7,685   -5.53%   x 1.06 
 slice::sort_large_mostly_descending             144,625 (553 MB/s)     135,915 (588 MB/s)            -8,710   -6.02%   x 1.06 
 slice::sort_large_random                        522,945 (152 MB/s)     581,100 (137 MB/s)            58,155   11.12%   x 0.90 
 slice::sort_large_strings                       1,639,057 (97 MB/s)    1,751,978 (91 MB/s)          112,921    6.89%   x 0.94 
 slice::sort_medium_random                       748 (1069 MB/s)        696 (1149 MB/s)                  -52   -6.95%   x 1.07 
 slice::sort_small_ascending                     28 (2857 MB/s)         30 (2666 MB/s)                     2    7.14%   x 0.93 
 slice::sort_small_big                           104 (12307 MB/s)       111 (11531 MB/s)                   7    6.73%   x 0.94 
 slice::sort_small_descending                    46 (1739 MB/s)         57 (1403 MB/s)                    11   23.91%   x 0.81 
 slice::sort_small_random                        34 (2352 MB/s)         35 (2285 MB/s)                     1    2.94%   x 0.97 
 slice::sort_unstable_by_key_lexicographic       17,735,897 (4 MB/s)    18,901,218 (4 MB/s)        1,165,321    6.57%   x 0.94 
 slice::sort_unstable_large_ascending            5,516 (14503 MB/s)     6,203 (12896 MB/s)               687   12.45%   x 0.89 
 slice::sort_unstable_large_big                  643,272 (1989 MB/s)    669,096 (1913 MB/s)           25,824    4.01%   x 0.96 
 slice::sort_unstable_large_descending           8,689 (9207 MB/s)      10,415 (7681 MB/s)             1,726   19.86%   x 0.83 
 slice::sort_unstable_large_expensive            14,603,530 (5 MB/s)    15,734,764 (5 MB/s)        1,131,234    7.75%   x 0.93 
 slice::sort_unstable_large_mostly_ascending     66,480 (1203 MB/s)     72,032 (1110 MB/s)             5,552    8.35%   x 0.92 
 slice::sort_unstable_large_mostly_descending    72,947 (1096 MB/s)     78,420 (1020 MB/s)             5,473    7.50%   x 0.93 
 slice::sort_unstable_large_random               210,033 (380 MB/s)     233,715 (342 MB/s)            23,682   11.28%   x 0.90 
 slice::sort_unstable_large_strings              1,354,090 (118 MB/s)   1,391,308 (114 MB/s)          37,218    2.75%   x 0.97 
 slice::sort_unstable_medium_random              595 (1344 MB/s)        597 (1340 MB/s)                    2    0.34%   x 1.00 
 slice::sort_unstable_small_ascending            26 (3076 MB/s)         29 (2758 MB/s)                     3   11.54%   x 0.90 
 slice::sort_unstable_small_big                  87 (14712 MB/s)        95 (13473 MB/s)                    8    9.20%   x 0.92 
 slice::sort_unstable_small_descending           48 (1666 MB/s)         50 (1600 MB/s)                     2    4.17%   x 0.96 
 slice::sort_unstable_small_random               29 (2758 MB/s)         32 (2500 MB/s)                     3   10.34%   x 0.91 
 slice::starts_with_diff_one_element_at_end      7                      7                                  0    0.00%   x 1.00 
 slice::starts_with_same_vector                  0                      0                                  0     NaN%    x NaN 
 slice::starts_with_single_element               0                      0                                  0     NaN%    x NaN 
 slice::zero_1kb_from_elem                       54                     60                                 6   11.11%   x 0.90 
 slice::zero_1kb_loop_set                        0                      0                                  0     NaN%    x NaN 
 slice::zero_1kb_mut_iter                        20                     27                                 7   35.00%   x 0.74 
 slice::zero_1kb_set_memory                      20                     27                                 7   35.00%   x 0.74 
 str::split_slice                                108                    115                                7    6.48%   x 0.94 
 vec::bench_from_slice_0000                      16                     16                                 0    0.00%   x 1.00 
 vec::bench_from_slice_0010                      40 (250 MB/s)          41 (243 MB/s)                      1    2.50%   x 0.98 
 vec::bench_from_slice_0100                      82 (1219 MB/s)         94 (1063 MB/s)                    12   14.63%   x 0.87 
 vec::bench_from_slice_1000                      593 (1686 MB/s)        652 (1533 MB/s)                   59    9.95%   x 0.91
