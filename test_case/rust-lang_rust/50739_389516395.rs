shell
 name                                vec_bench_master ns/iter  vec_bench_reserve ns/iter  diff ns/iter   diff %  speedup 
 vec::bench_clone_0000               14                        13                                   -1   -7.14%   x 1.08 
 vec::bench_clone_0010               51 (196 MB/s)             51 (196 MB/s)                         0    0.00%   x 1.00 
 vec::bench_clone_0100               149 (671 MB/s)            154 (649 MB/s)                        5    3.36%   x 0.97 
 vec::bench_clone_1000               1,085 (921 MB/s)          1,066 (938 MB/s)                    -19   -1.75%   x 1.02 
 vec::bench_clone_from_01_0000_0000  25                        25                                    0    0.00%   x 1.00 
 vec::bench_clone_from_01_0000_0010  80 (125 MB/s)             77 (129 MB/s)                        -3   -3.75%   x 1.04 
 vec::bench_clone_from_01_0000_0100  212 (471 MB/s)            218 (458 MB/s)                        6    2.83%   x 0.97 
 vec::bench_clone_from_01_0000_1000  1,631 (613 MB/s)          1,650 (606 MB/s)                     19    1.16%   x 0.99 
 vec::bench_clone_from_01_0010_0000  24                        24                                    0    0.00%   x 1.00 
 vec::bench_clone_from_01_0010_0010  81 (123 MB/s)             78 (128 MB/s)                        -3   -3.70%   x 1.04 
 vec::bench_clone_from_01_0010_0100  236 (423 MB/s)            228 (438 MB/s)                       -8   -3.39%   x 1.04 
 vec::bench_clone_from_01_0100_0010  81 (123 MB/s)             77 (129 MB/s)                        -4   -4.94%   x 1.05 
 vec::bench_clone_from_01_0100_0100  213 (469 MB/s)            236 (423 MB/s)                       23   10.80%   x 0.90 
 vec::bench_clone_from_01_0100_1000  1,715 (583 MB/s)          1,643 (608 MB/s)                    -72   -4.20%   x 1.04 
 vec::bench_clone_from_01_1000_0100  230 (434 MB/s)            217 (460 MB/s)                      -13   -5.65%   x 1.06 
 vec::bench_clone_from_01_1000_1000  1,745 (573 MB/s)          1,646 (607 MB/s)                    -99   -5.67%   x 1.06 
 vec::bench_clone_from_10_0000_0000  152                       144                                  -8   -5.26%   x 1.06 
 vec::bench_clone_from_10_0000_0010  347 (288 MB/s)            341 (293 MB/s)                       -6   -1.73%   x 1.02 
 vec::bench_clone_from_10_0000_0100  1,619 (617 MB/s)          1,563 (639 MB/s)                    -56   -3.46%   x 1.04 
 vec::bench_clone_from_10_0000_1000  14,540 (687 MB/s)         14,173 (705 MB/s)                  -367   -2.52%   x 1.03 
 vec::bench_clone_from_10_0010_0000  148                       143                                  -5   -3.38%   x 1.03 
 vec::bench_clone_from_10_0010_0010  373 (268 MB/s)            348 (287 MB/s)                      -25   -6.70%   x 1.07 
 vec::bench_clone_from_10_0010_0100  1,711 (584 MB/s)          1,619 (617 MB/s)                    -92   -5.38%   x 1.06 
 vec::bench_clone_from_10_0100_0010  356 (280 MB/s)            349 (286 MB/s)                       -7   -1.97%   x 1.02 
 vec::bench_clone_from_10_0100_0100  1,719 (581 MB/s)          1,648 (606 MB/s)                    -71   -4.13%   x 1.04 
 vec::bench_clone_from_10_0100_1000  14,240 (702 MB/s)         14,319 (698 MB/s)                    79    0.55%   x 0.99 
 vec::bench_clone_from_10_1000_0100  1,732 (577 MB/s)          1,613 (619 MB/s)                   -119   -6.87%   x 1.07 
 vec::bench_clone_from_10_1000_1000  15,541 (643 MB/s)         15,080 (663 MB/s)                  -461   -2.97%   x 1.03 
 vec::bench_extend_0000_0000         40                        39                                   -1   -2.50%   x 1.03 
 vec::bench_extend_0000_0010         119 (84 MB/s)             122 (81 MB/s)                         3    2.52%   x 0.98 
 vec::bench_extend_0000_0100         227 (440 MB/s)            240 (416 MB/s)                       13    5.73%   x 0.95 
 vec::bench_extend_0000_1000         1,373 (728 MB/s)          1,410 (709 MB/s)                     37    2.69%   x 0.97 
 vec::bench_extend_0010_0010         177 (56 MB/s)             185 (54 MB/s)                         8    4.52%   x 0.96 
 vec::bench_extend_0100_0100         410 (243 MB/s)            418 (239 MB/s)                        8    1.95%   x 0.98 
 vec::bench_extend_1000_1000         3,133 (319 MB/s)          3,190 (313 MB/s)                     57    1.82%   x 0.98 
 vec::bench_from_elem_0000           10                        7                                    -3  -30.00%   x 1.43 
 vec::bench_from_elem_0010           52 (192 MB/s)             54 (185 MB/s)                         2    3.85%   x 0.96 
 vec::bench_from_elem_0100           168 (595 MB/s)            166 (602 MB/s)                       -2   -1.19%   x 1.01 
 vec::bench_from_elem_1000           1,284 (778 MB/s)          1,262 (792 MB/s)                    -22   -1.71%   x 1.02 
 vec::bench_from_fn_0000             11                        11                                    0    0.00%   x 1.00 
 vec::bench_from_fn_0010             54 (185 MB/s)             58 (172 MB/s)                         4    7.41%   x 0.93 
 vec::bench_from_fn_0100             207 (483 MB/s)            215 (465 MB/s)                        8    3.86%   x 0.96 
 vec::bench_from_fn_1000             1,627 (614 MB/s)          1,645 (607 MB/s)                     18    1.11%   x 0.99 
 vec::bench_from_iter_0000           15                        15                                    0    0.00%   x 1.00 
 vec::bench_from_iter_0010           55 (181 MB/s)             58 (172 MB/s)                         3    5.45%   x 0.95 
 vec::bench_from_iter_0100           142 (704 MB/s)            149 (671 MB/s)                        7    4.93%   x 0.95 
 vec::bench_from_iter_1000           1,110 (900 MB/s)          1,121 (892 MB/s)                     11    0.99%   x 0.99 
 vec::bench_from_slice_0000          23                        23                                    0    0.00%   x 1.00 
 vec::bench_from_slice_0010          94 (106 MB/s)             93 (107 MB/s)                        -1   -1.06%   x 1.01 
 vec::bench_from_slice_0100          202 (495 MB/s)            199 (502 MB/s)                       -3   -1.49%   x 1.02 
 vec::bench_from_slice_1000          1,357 (736 MB/s)          1,345 (743 MB/s)                    -12   -0.88%   x 1.01 
 vec::bench_new                      0                         0                                     0     NaN%    x NaN 
 vec::bench_push_all_0000_0000       21                        21                                    0    0.00%   x 1.00 
 vec::bench_push_all_0000_0010       61 (163 MB/s)             63 (158 MB/s)                         2    3.28%   x 0.97 
 vec::bench_push_all_0000_0100       164 (609 MB/s)            153 (653 MB/s)                      -11   -6.71%   x 1.07 
 vec::bench_push_all_0000_1000       1,089 (918 MB/s)          1,127 (887 MB/s)                     38    3.49%   x 0.97 
 vec::bench_push_all_0010_0010       139 (71 MB/s)             130 (76 MB/s)                        -9   -6.47%   x 1.07 
 vec::bench_push_all_0100_0100       338 (295 MB/s)            347 (288 MB/s)                        9    2.66%   x 0.97 
 vec::bench_push_all_1000_1000       2,979 (335 MB/s)          2,994 (334 MB/s)                     15    0.50%   x 0.99 
 vec::bench_push_all_move_0000_0000  36                        36                                    0    0.00%   x 1.00 
 vec::bench_push_all_move_0000_0010  118 (84 MB/s)             118 (84 MB/s)                         0    0.00%   x 1.00 
 vec::bench_push_all_move_0000_0100  224 (446 MB/s)            233 (429 MB/s)                        9    4.02%   x 0.96 
 vec::bench_push_all_move_0000_1000  1,400 (714 MB/s)          1,350 (740 MB/s)                    -50   -3.57%   x 1.04 
 vec::bench_push_all_move_0010_0010  178 (56 MB/s)             177 (56 MB/s)                        -1   -0.56%   x 1.01 
 vec::bench_push_all_move_0100_0100  399 (250 MB/s)            398 (251 MB/s)                       -1   -0.25%   x 1.00 
 vec::bench_push_all_move_1000_1000  3,218 (310 MB/s)          3,044 (328 MB/s)                   -174   -5.41%   x 1.06 
 vec::bench_with_capacity_0000       2                         2                                     0    0.00%   x 1.00 
 vec::bench_with_capacity_0010       31 (322 MB/s)             31 (322 MB/s)                         0    0.00%   x 1.00 
 vec::bench_with_capacity_0100       31 (3225 MB/s)            31 (3225 MB/s)                        0    0.00%   x 1.00 
 vec::bench_with_capacity_1000       31 (32258 MB/s)           31 (32258 MB/s)                       0    0.00%   x 1.00 
 vec_deque::bench_grow_1025          5,754                     5,485                              -269   -4.68%   x 1.05 
 vec_deque::bench_iter_1000          1,002                     994                                  -8   -0.80%   x 1.01 
 vec_deque::bench_mut_iter_1000      983                       990                                   7    0.71%   x 0.99 
 vec_deque::bench_new                37                        36                                   -1   -2.70%   x 1.03 
