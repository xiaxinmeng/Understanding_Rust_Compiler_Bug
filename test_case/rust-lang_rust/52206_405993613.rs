
 name                                            control2 ns/iter       variable2 ns/iter       diff ns/iter  diff %  speedup 
 slice::concat                                   1,114                  1,191                             77   6.91%   x 0.94 
 slice::contains_last_element                    28                     30                                 2   7.14%   x 0.93 
 slice::ends_with_diff_one_element_at_beginning  2                      2                                  0   0.00%   x 1.00 
 slice::ends_with_same_vector                    0                      0                                  0    NaN%    x NaN 
 slice::ends_with_single_element                 0                      0                                  0    NaN%    x NaN 
 slice::iterator                                 7                      7                                  0   0.00%   x 1.00 
 slice::join                                     1,218                  1,171                            -47  -3.86%   x 1.04 
 slice::mut_iterator                             8                      8                                  0   0.00%   x 1.00 
 slice::push                                     2                      2                                  0   0.00%   x 1.00 
 slice::random_inserts                           3,383                  3,616                            233   6.89%   x 0.94 
 slice::random_removes                           3,262                  3,321                             59   1.81%   x 0.98 
 slice::reverse_simd_f64x4                       31,368 (33428 MB/s)    32,043 (32723 MB/s)              675   2.15%   x 0.98 
 slice::reverse_u128                             30,879 (33957 MB/s)    30,533 (34342 MB/s)             -346  -1.12%   x 1.01 
 slice::reverse_u16                              85,969 (12197 MB/s)    86,437 (12131 MB/s)              468   0.54%   x 0.99 
 slice::reverse_u32                              84,347 (12431 MB/s)    83,742 (12521 MB/s)             -605  -0.72%   x 1.01 
 slice::reverse_u64                              47,860 (21909 MB/s)    49,457 (21201 MB/s)            1,597   3.34%   x 0.97 
 slice::reverse_u8                               52,618 (19928 MB/s)    60,732 (17265 MB/s)            8,114  15.42%   x 0.87 
 slice::reverse_u8x3                             206,757 (5071 MB/s)    210,920 (4971 MB/s)            4,163   2.01%   x 0.98 
 slice::rotate_huge_by1                          5,354,006 (7833 MB/s)  5,199,106 (8067 MB/s)       -154,900  -2.89%   x 1.03 
 slice::rotate_huge_by1234577_big                9,123,162 (4597 MB/s)  8,641,313 (4853 MB/s)       -481,849  -5.28%   x 1.06 
 slice::rotate_huge_by1234577_bytes              8,817,380 (4756 MB/s)  8,724,509 (4807 MB/s)        -92,871  -1.05%   x 1.01 
 slice::rotate_huge_by1234577_strings            9,354,997 (4483 MB/s)  8,792,301 (4770 MB/s)       -562,696  -6.01%   x 1.06 
 slice::rotate_huge_by1234577_u64                8,662,283 (4842 MB/s)  13,311,606 (3150 MB/s)     4,649,323  53.67%   x 0.65 
 slice::rotate_huge_by9199_big                   5,292,371 (7925 MB/s)  5,440,271 (7709 MB/s)        147,900   2.79%   x 0.97 
 slice::rotate_huge_by9199_bytes                 5,302,024 (7910 MB/s)  5,274,796 (7951 MB/s)        -27,228  -0.51%   x 1.01 
 slice::rotate_huge_by9199_strings               5,282,433 (7940 MB/s)  5,281,289 (7941 MB/s)         -1,144  -0.02%   x 1.00 
 slice::rotate_huge_by9199_u64                   5,439,123 (7711 MB/s)  5,251,565 (7986 MB/s)       -187,558  -3.45%   x 1.04 
 slice::rotate_huge_half                         5,908,147 (7099 MB/s)  5,378,122 (7798 MB/s)       -530,025  -8.97%   x 1.10 
 slice::rotate_huge_half_plus_one                8,336,232 (5031 MB/s)  7,992,677 (5247 MB/s)       -343,555  -4.12%   x 1.04 
 slice::rotate_medium_by1                        1,125 (65123 MB/s)     1,321 (55461 MB/s)               196  17.42%   x 0.85 
 slice::rotate_medium_by727_bytes                3,270 (22404 MB/s)     3,250 (22542 MB/s)               -20  -0.61%   x 1.01 
 slice::rotate_medium_by727_strings              3,226 (22705 MB/s)     3,349 (21871 MB/s)               123   3.81%   x 0.96 
 slice::rotate_medium_by727_u64                  3,201 (22887 MB/s)     3,308 (22147 MB/s)               107   3.34%   x 0.97 
 slice::rotate_medium_half                       2,231 (32839 MB/s)     2,226 (32912 MB/s)                -5  -0.22%   x 1.00 
 slice::rotate_medium_half_plus_one              2,380 (30783 MB/s)     2,406 (30450 MB/s)                26   1.09%   x 0.99 
 slice::rotate_tiny_by1                          17 (7529 MB/s)         17 (7529 MB/s)                     0   0.00%   x 1.00 
 slice::rotate_tiny_half                         16 (8000 MB/s)         16 (8000 MB/s)                     0   0.00%   x 1.00 
 slice::rotate_tiny_half_plus_one                17 (7529 MB/s)         17 (7529 MB/s)                     0   0.00%   x 1.00 
 slice::sort_by_cached_key_lexicographic         2,422,180 (33 MB/s)    2,430,887 (32 MB/s)            8,707   0.36%   x 1.00 
 slice::sort_by_key_lexicographic                16,280,240 (4 MB/s)    15,702,589 (5 MB/s)         -577,651  -3.55%   x 1.04 
 slice::sort_large_ascending                     6,036 (13253 MB/s)     7,546 (10601 MB/s)             1,510  25.02%   x 0.80 
 slice::sort_large_big                           882,470 (1450 MB/s)    867,494 (1475 MB/s)          -14,976  -1.70%   x 1.02 
 slice::sort_large_descending                    9,649 (8291 MB/s)      9,522 (8401 MB/s)               -127  -1.32%   x 1.01 
 slice::sort_large_expensive                     20,612,173 (3 MB/s)    20,447,440 (3 MB/s)         -164,733  -0.80%   x 1.01 
 slice::sort_large_mostly_ascending              139,018 (575 MB/s)     129,340 (618 MB/s)            -9,678  -6.96%   x 1.07 
 slice::sort_large_mostly_descending             144,625 (553 MB/s)     141,640 (564 MB/s)            -2,985  -2.06%   x 1.02 
 slice::sort_large_random                        522,945 (152 MB/s)     511,354 (156 MB/s)           -11,591  -2.22%   x 1.02 
 slice::sort_large_strings                       1,639,057 (97 MB/s)    1,611,141 (99 MB/s)          -27,916  -1.70%   x 1.02 
 slice::sort_medium_random                       748 (1069 MB/s)        686 (1166 MB/s)                  -62  -8.29%   x 1.09 
 slice::sort_small_ascending                     28 (2857 MB/s)         27 (2962 MB/s)                    -1  -3.57%   x 1.04 
 slice::sort_small_big                           104 (12307 MB/s)       103 (12427 MB/s)                  -1  -0.96%   x 1.01 
 slice::sort_small_descending                    46 (1739 MB/s)         51 (1568 MB/s)                     5  10.87%   x 0.90 
 slice::sort_small_random                        34 (2352 MB/s)         32 (2500 MB/s)                    -2  -5.88%   x 1.06 
 slice::sort_unstable_by_key_lexicographic       17,735,897 (4 MB/s)    17,216,278 (4 MB/s)         -519,619  -2.93%   x 1.03 
 slice::sort_unstable_large_ascending            5,516 (14503 MB/s)     5,555 (14401 MB/s)                39   0.71%   x 0.99 
 slice::sort_unstable_large_big                  643,272 (1989 MB/s)    634,969 (2015 MB/s)           -8,303  -1.29%   x 1.01 
 slice::sort_unstable_large_descending           8,689 (9207 MB/s)      8,755 (9137 MB/s)                 66   0.76%   x 0.99 
 slice::sort_unstable_large_expensive            14,603,530 (5 MB/s)    14,462,198 (5 MB/s)         -141,332  -0.97%   x 1.01 
 slice::sort_unstable_large_mostly_ascending     66,480 (1203 MB/s)     66,717 (1199 MB/s)               237   0.36%   x 1.00 
 slice::sort_unstable_large_mostly_descending    72,947 (1096 MB/s)     70,074 (1141 MB/s)            -2,873  -3.94%   x 1.04 
 slice::sort_unstable_large_random               210,033 (380 MB/s)     215,013 (372 MB/s)             4,980   2.37%   x 0.98 
 slice::sort_unstable_large_strings              1,354,090 (118 MB/s)   1,322,046 (121 MB/s)         -32,044  -2.37%   x 1.02 
 slice::sort_unstable_medium_random              595 (1344 MB/s)        537 (1489 MB/s)                  -58  -9.75%   x 1.11 
 slice::sort_unstable_small_ascending            26 (3076 MB/s)         27 (2962 MB/s)                     1   3.85%   x 0.96 
 slice::sort_unstable_small_big                  87 (14712 MB/s)        93 (13763 MB/s)                    6   6.90%   x 0.94 
 slice::sort_unstable_small_descending           48 (1666 MB/s)         44 (1818 MB/s)                    -4  -8.33%   x 1.09 
 slice::sort_unstable_small_random               29 (2758 MB/s)         30 (2666 MB/s)                     1   3.45%   x 0.97 
 slice::starts_with_diff_one_element_at_end      7                      7                                  0   0.00%   x 1.00 
 slice::starts_with_same_vector                  0                      0                                  0    NaN%    x NaN 
 slice::starts_with_single_element               0                      0                                  0    NaN%    x NaN 
 slice::zero_1kb_from_elem                       54                     51                                -3  -5.56%   x 1.06 
 slice::zero_1kb_loop_set                        0                      0                                  0    NaN%    x NaN 
 slice::zero_1kb_mut_iter                        20                     19                                -1  -5.00%   x 1.05 
 slice::zero_1kb_set_memory                      20                     19                                -1  -5.00%   x 1.05 
 str::split_slice                                108                    109                                1   0.93%   x 0.99 
 vec::bench_from_slice_0000                      16                     15                                -1  -6.25%   x 1.07 
 vec::bench_from_slice_0010                      40 (250 MB/s)          40 (250 MB/s)                      0   0.00%   x 1.00 
 vec::bench_from_slice_0100                      82 (1219 MB/s)         84 (1190 MB/s)                     2   2.44%   x 0.98 
 vec::bench_from_slice_1000                      593 (1686 MB/s)        638 (1567 MB/s)                   45   7.59%   x 0.93 
