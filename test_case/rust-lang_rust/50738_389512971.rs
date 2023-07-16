shell
 name                                vec_bench_master ns/iter  vec_bench_exc ns/iter  diff ns/iter   diff %  speedup 
 slice::ends_with_same_vector        0                         0                                 0     NaN%    x NaN 
 slice::starts_with_same_vector      0                         0                                 0     NaN%    x NaN 
 vec::bench_clone_0000               14                        13                               -1   -7.14%   x 1.08 
 vec::bench_clone_0010               51 (196 MB/s)             51 (196 MB/s)                     0    0.00%   x 1.00 
 vec::bench_clone_0100               149 (671 MB/s)            139 (719 MB/s)                  -10   -6.71%   x 1.07 
 vec::bench_clone_1000               1,085 (921 MB/s)          1,057 (946 MB/s)                -28   -2.58%   x 1.03 
 vec::bench_clone_from_01_0000_0000  25                        24                               -1   -4.00%   x 1.04 
 vec::bench_clone_from_01_0000_0010  80 (125 MB/s)             77 (129 MB/s)                    -3   -3.75%   x 1.04 
 vec::bench_clone_from_01_0000_0100  212 (471 MB/s)            204 (490 MB/s)                   -8   -3.77%   x 1.04 
 vec::bench_clone_from_01_0000_1000  1,631 (613 MB/s)          1,616 (618 MB/s)                -15   -0.92%   x 1.01 
 vec::bench_clone_from_01_0010_0000  24                        27                                3   12.50%   x 0.89 
 vec::bench_clone_from_01_0010_0010  81 (123 MB/s)             72 (138 MB/s)                    -9  -11.11%   x 1.12 
 vec::bench_clone_from_01_0010_0100  236 (423 MB/s)            204 (490 MB/s)                  -32  -13.56%   x 1.16 
 vec::bench_clone_from_01_0100_0010  81 (123 MB/s)             70 (142 MB/s)                   -11  -13.58%   x 1.16 
 vec::bench_clone_from_01_0100_0100  213 (469 MB/s)            210 (476 MB/s)                   -3   -1.41%   x 1.01 
 vec::bench_clone_from_01_0100_1000  1,715 (583 MB/s)          1,660 (602 MB/s)                -55   -3.21%   x 1.03 
 vec::bench_clone_from_01_1000_0100  230 (434 MB/s)            226 (442 MB/s)                   -4   -1.74%   x 1.02 
 vec::bench_clone_from_01_1000_1000  1,745 (573 MB/s)          1,658 (603 MB/s)                -87   -4.99%   x 1.05 
 vec::bench_clone_from_10_0000_0000  152                       146                              -6   -3.95%   x 1.04 
 vec::bench_clone_from_10_0000_0010  347 (288 MB/s)            321 (311 MB/s)                  -26   -7.49%   x 1.08 
 vec::bench_clone_from_10_0000_0100  1,619 (617 MB/s)          1,513 (660 MB/s)               -106   -6.55%   x 1.07 
 vec::bench_clone_from_10_0000_1000  14,540 (687 MB/s)         14,150 (706 MB/s)              -390   -2.68%   x 1.03 
 vec::bench_clone_from_10_0010_0000  148                       143                              -5   -3.38%   x 1.03 
 vec::bench_clone_from_10_0010_0010  373 (268 MB/s)            321 (311 MB/s)                  -52  -13.94%   x 1.16 
 vec::bench_clone_from_10_0010_0100  1,711 (584 MB/s)          1,513 (660 MB/s)               -198  -11.57%   x 1.13 
 vec::bench_clone_from_10_0100_0010  356 (280 MB/s)            323 (309 MB/s)                  -33   -9.27%   x 1.10 
 vec::bench_clone_from_10_0100_0100  1,719 (581 MB/s)          1,510 (662 MB/s)               -209  -12.16%   x 1.14 
 vec::bench_clone_from_10_0100_1000  14,240 (702 MB/s)         14,201 (704 MB/s)               -39   -0.27%   x 1.00 
 vec::bench_clone_from_10_1000_0100  1,732 (577 MB/s)          1,511 (661 MB/s)               -221  -12.76%   x 1.15 
 vec::bench_clone_from_10_1000_1000  15,541 (643 MB/s)         15,025 (665 MB/s)              -516   -3.32%   x 1.03 
 vec::bench_extend_0000_0000         40                        34                               -6  -15.00%   x 1.18 
 vec::bench_extend_0000_0010         119 (84 MB/s)             110 (90 MB/s)                    -9   -7.56%   x 1.08 
 vec::bench_extend_0000_0100         227 (440 MB/s)            220 (454 MB/s)                   -7   -3.08%   x 1.03 
 vec::bench_extend_0000_1000         1,373 (728 MB/s)          1,332 (750 MB/s)                -41   -2.99%   x 1.03 
 vec::bench_extend_0010_0010         177 (56 MB/s)             180 (55 MB/s)                     3    1.69%   x 0.98 
 vec::bench_extend_0100_0100         410 (243 MB/s)            396 (252 MB/s)                  -14   -3.41%   x 1.04 
 vec::bench_extend_1000_1000         3,133 (319 MB/s)          3,002 (333 MB/s)               -131   -4.18%   x 1.04 
 vec::bench_from_elem_0000           10                        10                                0    0.00%   x 1.00 
 vec::bench_from_elem_0010           52 (192 MB/s)             51 (196 MB/s)                    -1   -1.92%   x 1.02 
 vec::bench_from_elem_0100           168 (595 MB/s)            192 (520 MB/s)                   24   14.29%   x 0.88 
 vec::bench_from_elem_1000           1,284 (778 MB/s)          1,264 (791 MB/s)                -20   -1.56%   x 1.02 
 vec::bench_from_fn_0000             11                        11                                0    0.00%   x 1.00 
 vec::bench_from_fn_0010             54 (185 MB/s)             60 (166 MB/s)                     6   11.11%   x 0.90 
 vec::bench_from_fn_0100             207 (483 MB/s)            228 (438 MB/s)                   21   10.14%   x 0.91 
 vec::bench_from_fn_1000             1,627 (614 MB/s)          1,600 (625 MB/s)                -27   -1.66%   x 1.02 
 vec::bench_from_iter_0000           15                        15                                0    0.00%   x 1.00 
 vec::bench_from_iter_0010           55 (181 MB/s)             51 (196 MB/s)                    -4   -7.27%   x 1.08 
 vec::bench_from_iter_0100           142 (704 MB/s)            142 (704 MB/s)                    0    0.00%   x 1.00 
 vec::bench_from_iter_1000           1,110 (900 MB/s)          1,057 (946 MB/s)                -53   -4.77%   x 1.05 
 vec::bench_from_slice_0000          23                        23                                0    0.00%   x 1.00 
 vec::bench_from_slice_0010          94 (106 MB/s)             110 (90 MB/s)                    16   17.02%   x 0.85 
 vec::bench_from_slice_0100          202 (495 MB/s)            228 (438 MB/s)                   26   12.87%   x 0.89 
 vec::bench_from_slice_1000          1,357 (736 MB/s)          1,320 (757 MB/s)                -37   -2.73%   x 1.03 
 vec::bench_new                      0                         0                                 0     NaN%    x NaN 
 vec::bench_push_all_0000_0000       21                        21                                0    0.00%   x 1.00 
 vec::bench_push_all_0000_0010       61 (163 MB/s)             58 (172 MB/s)                    -3   -4.92%   x 1.05 
 vec::bench_push_all_0000_0100       164 (609 MB/s)            147 (680 MB/s)                  -17  -10.37%   x 1.12 
 vec::bench_push_all_0000_1000       1,089 (918 MB/s)          1,072 (932 MB/s)                -17   -1.56%   x 1.02 
 vec::bench_push_all_0010_0010       139 (71 MB/s)             127 (78 MB/s)                   -12   -8.63%   x 1.09 
 vec::bench_push_all_0100_0100       338 (295 MB/s)            334 (299 MB/s)                   -4   -1.18%   x 1.01 
 vec::bench_push_all_1000_1000       2,979 (335 MB/s)          3,086 (324 MB/s)                107    3.59%   x 0.97 
 vec::bench_push_all_move_0000_0000  36                        37                                1    2.78%   x 0.97 
 vec::bench_push_all_move_0000_0010  118 (84 MB/s)             116 (86 MB/s)                    -2   -1.69%   x 1.02 
 vec::bench_push_all_move_0000_0100  224 (446 MB/s)            234 (427 MB/s)                   10    4.46%   x 0.96 
 vec::bench_push_all_move_0000_1000  1,400 (714 MB/s)          1,352 (739 MB/s)                -48   -3.43%   x 1.04 
 vec::bench_push_all_move_0010_0010  178 (56 MB/s)             196 (51 MB/s)                    18   10.11%   x 0.91 
 vec::bench_push_all_move_0100_0100  399 (250 MB/s)            399 (250 MB/s)                    0    0.00%   x 1.00 
 vec::bench_push_all_move_1000_1000  3,218 (310 MB/s)          3,334 (299 MB/s)                116    3.60%   x 0.97 
 vec::bench_with_capacity_0000       2                         2                                 0    0.00%   x 1.00 
 vec::bench_with_capacity_0010       31 (322 MB/s)             31 (322 MB/s)                     0    0.00%   x 1.00 
 vec::bench_with_capacity_0100       31 (3225 MB/s)            30 (3333 MB/s)                   -1   -3.23%   x 1.03 
 vec::bench_with_capacity_1000       31 (32258 MB/s)           34 (29411 MB/s)                   3    9.68%   x 0.91 
 vec_deque::bench_grow_1025          5,754                     5,546                          -208   -3.61%   x 1.04 
 vec_deque::bench_iter_1000          1,002                     910                             -92   -9.18%   x 1.10 
 vec_deque::bench_mut_iter_1000      983                       924                             -59   -6.00%   x 1.06 
 vec_deque::bench_new                37                        36                               -1   -2.70%   x 1.03 
