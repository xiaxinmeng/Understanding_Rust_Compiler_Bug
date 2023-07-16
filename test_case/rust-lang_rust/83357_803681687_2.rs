
 name                                    old ns/iter          new ns/iter          diff ns/iter   diff %  speedup
 vec::bench_chain_chain_collect          1,551                1,192                        -359  -23.15%   x 1.30
 vec::bench_chain_collect                1,671                1,149                        -522  -31.24%   x 1.45
 vec::bench_chain_extend_ref             1,669                1,155                        -514  -30.80%   x 1.45
 vec::bench_chain_extend_value           1,647                1,152                        -495  -30.05%   x 1.43
 vec::bench_clone_0100                   61 (1639 MB/s)       55 (1818 MB/s)                 -6   -9.84%   x 1.11
 vec::bench_clone_from_01_0000_0000      10                   9                              -1  -10.00%   x 1.11
 vec::bench_clone_from_01_0000_0010      22 (454 MB/s)        23 (434 MB/s)                   1    4.55%   x 0.96
 vec::bench_clone_from_01_0000_0100      74 (1351 MB/s)       72 (1388 MB/s)                 -2   -2.70%   x 1.03
 vec::bench_clone_from_01_0000_1000      484 (2066 MB/s)      507 (1972 MB/s)                23    4.75%   x 0.95
 vec::bench_clone_from_01_0010_0010      22 (454 MB/s)        21 (476 MB/s)                  -1   -4.55%   x 1.05
 vec::bench_clone_from_01_0010_0100      73 (1369 MB/s)       72 (1388 MB/s)                 -1   -1.37%   x 1.01
 vec::bench_clone_from_01_0100_0010      22 (454 MB/s)        23 (434 MB/s)                   1    4.55%   x 0.96
 vec::bench_clone_from_01_0100_0100      73 (1369 MB/s)       72 (1388 MB/s)                 -1   -1.37%   x 1.01
 vec::bench_clone_from_01_0100_1000      493 (2028 MB/s)      509 (1964 MB/s)                16    3.25%   x 0.97
 vec::bench_clone_from_01_1000_1000      495 (2020 MB/s)      509 (1964 MB/s)                14    2.83%   x 0.97
 vec::bench_clone_from_10_0000_0000      77                   67                            -10  -12.99%   x 1.15
 vec::bench_clone_from_10_0000_0010      117 (854 MB/s)       108 (925 MB/s)                 -9   -7.69%   x 1.08
 vec::bench_clone_from_10_0000_0100      559 (1788 MB/s)      543 (1841 MB/s)               -16   -2.86%   x 1.03
 vec::bench_clone_from_10_0010_0000      76                   67                             -9  -11.84%   x 1.13
 vec::bench_clone_from_10_0010_0010      118 (847 MB/s)       106 (943 MB/s)                -12  -10.17%   x 1.11
 vec::bench_clone_from_10_0010_0100      569 (1757 MB/s)      555 (1801 MB/s)               -14   -2.46%   x 1.03
 vec::bench_clone_from_10_0100_0010      121 (826 MB/s)       112 (892 MB/s)                 -9   -7.44%   x 1.08
 vec::bench_clone_from_10_0100_0100      568 (1760 MB/s)      560 (1785 MB/s)                -8   -1.41%   x 1.01
 vec::bench_clone_from_10_0100_1000      4,109 (2433 MB/s)    4,177 (2394 MB/s)              68    1.65%   x 0.98
 vec::bench_clone_from_10_1000_0100      553 (1808 MB/s)      573 (1745 MB/s)                20    3.62%   x 0.97
 vec::bench_clone_from_10_1000_1000      4,125 (2424 MB/s)    4,183 (2390 MB/s)              58    1.41%   x 0.99
 vec::bench_dedup_new_100                49 (8163 MB/s)       55 (7272 MB/s)                  6   12.24%   x 0.89
 vec::bench_dedup_new_1000               527 (7590 MB/s)      767 (5215 MB/s)               240   45.54%   x 0.69
 vec::bench_dedup_new_10000              5,014 (7977 MB/s)    6,200 (6451 MB/s)           1,186   23.65%   x 0.81
 vec::bench_dedup_old_100                102 (3921 MB/s)      77 (5194 MB/s)                -25  -24.51%   x 1.32
 vec::bench_dedup_old_1000               782 (5115 MB/s)      551 (7259 MB/s)              -231  -29.54%   x 1.42
 vec::bench_dedup_old_10000              10,994 (3638 MB/s)   8,669 (4614 MB/s)          -2,325  -21.15%   x 1.27
 vec::bench_dedup_old_100000             365,853 (1093 MB/s)  344,084 (1162 MB/s)       -21,769   -5.95%   x 1.06
 vec::bench_extend_0000_0000             10                   9                              -1  -10.00%   x 1.11
 vec::bench_extend_0000_0010             35 (285 MB/s)        36 (277 MB/s)                   1    2.86%   x 0.97
 vec::bench_extend_0000_0100             99 (1010 MB/s)       90 (1111 MB/s)                 -9   -9.09%   x 1.10
 vec::bench_extend_0000_1000             534 (1872 MB/s)      586 (1706 MB/s)                52    9.74%   x 0.91
 vec::bench_extend_0010_0010             85 (117 MB/s)        93 (107 MB/s)                   8    9.41%   x 0.91
 vec::bench_extend_0100_0100             185 (540 MB/s)       201 (497 MB/s)                 16    8.65%   x 0.92
 vec::bench_extend_from_slice_0000_0000  8                    6                              -2  -25.00%   x 1.33
 vec::bench_extend_from_slice_0000_0010  24 (416 MB/s)        23 (434 MB/s)                  -1   -4.17%   x 1.04
 vec::bench_extend_from_slice_0000_0100  70 (1428 MB/s)       68 (1470 MB/s)                 -2   -2.86%   x 1.03
 vec::bench_extend_from_slice_0000_1000  451 (2217 MB/s)      461 (2169 MB/s)                10    2.22%   x 0.98
 vec::bench_extend_from_slice_0010_0010  64 (156 MB/s)        74 (135 MB/s)                  10   15.62%   x 0.86
 vec::bench_extend_from_slice_0100_0100  172 (581 MB/s)       178 (561 MB/s)                  6    3.49%   x 0.97
 vec::bench_extend_from_slice_1000_1000  915 (1092 MB/s)      941 (1062 MB/s)                26    2.84%   x 0.97
 vec::bench_from_elem_0000               3                    2                              -1  -33.33%   x 1.50
 vec::bench_from_elem_0010               19 (526 MB/s)        17 (588 MB/s)                  -2  -10.53%   x 1.12
 vec::bench_from_elem_0100               66 (1515 MB/s)       62 (1612 MB/s)                 -4   -6.06%   x 1.06
 vec::bench_from_elem_1000               512 (1953 MB/s)      518 (1930 MB/s)                 6    1.17%   x 0.99
 vec::bench_from_fn_0000                 3                    2                              -1  -33.33%   x 1.50
 vec::bench_from_fn_0100                 73 (1369 MB/s)       79 (1265 MB/s)                  6    8.22%   x 0.92
 vec::bench_from_fn_1000                 620 (1612 MB/s)      636 (1572 MB/s)                16    2.58%   x 0.97
 vec::bench_from_iter_1000               456 (2192 MB/s)      451 (2217 MB/s)                -5   -1.10%   x 1.01
 vec::bench_from_slice_0010              28 (357 MB/s)        27 (370 MB/s)                  -1   -3.57%   x 1.04
 vec::bench_from_slice_0100              79 (1265 MB/s)       78 (1282 MB/s)                 -1   -1.27%   x 1.01
 vec::bench_from_slice_1000              535 (1869 MB/s)      557 (1795 MB/s)                22    4.11%   x 0.96
 vec::bench_in_place_collect_droppable   1,838                1,764                         -74   -4.03%   x 1.04
 vec::bench_in_place_u128_0010_i0        49                   53                              4    8.16%   x 0.92
 vec::bench_in_place_u128_0100_i0        70                   73                              3    4.29%   x 0.96
 vec::bench_in_place_u128_0100_i1        112                  103                            -9   -8.04%   x 1.09
 vec::bench_in_place_xu32_0010_i1        16                   11                             -5  -31.25%   x 1.45
 vec::bench_in_place_xu32_0100_i0        58                   56                             -2   -3.45%   x 1.04
 vec::bench_in_place_xu32_0100_i1        27                   22                             -5  -18.52%   x 1.23
 vec::bench_in_place_xu32_1000_i0        147                  138                            -9   -6.12%   x 1.07
 vec::bench_in_place_xu32_1000_i1        167                  163                            -4   -2.40%   x 1.02
 vec::bench_in_place_xxu8_0100_i0        21                   24                              3   14.29%   x 0.88
 vec::bench_in_place_xxu8_1000_i0        60                   68                              8   13.33%   x 0.88
 vec::bench_in_place_xxu8_1000_i1        36                   38                              2    5.56%   x 0.95
 vec::bench_in_place_zip_iter_mut        123                  139                            16   13.01%   x 0.88
 vec::bench_with_capacity_1000           44 (22727 MB/s)      36 (27777 MB/s)                -8  -18.18%   x 1.22
