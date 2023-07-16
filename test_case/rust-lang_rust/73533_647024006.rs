
 name                                             old ns/iter        new ns/iter        diff ns/iter   diff %  speedup
 ascii::long::case10_mask_mult_bool_lookup_table  36,285 (192 MB/s)  33,184 (210 MB/s)        -3,101   -8.55%   x 1.09
 ascii::medium::is_ascii_alphanumeric             150 (213 MB/s)     162 (197 MB/s)               12    8.00%   x 0.93
 ascii::medium::is_ascii_control                  136 (235 MB/s)     126 (253 MB/s)              -10   -7.35%   x 1.08
 ascii::medium::is_ascii_uppercase                136 (235 MB/s)     129 (248 MB/s)               -7   -5.15%   x 1.05
 ascii::medium::is_ascii_whitespace               134 (238 MB/s)     127 (251 MB/s)               -7   -5.22%   x 1.06
 ascii::short::case00_alloc_only                  133 (52 MB/s)      126 (55 MB/s)                -7   -5.26%   x 1.06
 ascii::short::case03_branch_and_subtract         157 (44 MB/s)      146 (47 MB/s)               -11   -7.01%   x 1.08
 ascii::short::is_ascii_alphabetic                148 (47 MB/s)      138 (50 MB/s)               -10   -6.76%   x 1.07
 ascii::short::is_ascii_digit                     180 (38 MB/s)      133 (52 MB/s)               -47  -26.11%   x 1.35
 ascii::short::is_ascii_punctuation               145 (48 MB/s)      132 (53 MB/s)               -13   -8.97%   x 1.10
 fmt::write_vec_macro2                            154,290            166,191                  11,901    7.71%   x 0.93
 iter::bench_enumerate_ref_sum                    20,468,050         22,914,835            2,446,785   11.95%   x 0.89
 iter::bench_filter_map_ref_sum                   24,912,476         22,021,140           -2,891,336  -11.61%   x 1.13
 iter::bench_flat_map_chain_ref_sum               136,840,693        166,930,595          30,089,902   21.99%   x 0.82
 iter::bench_flat_map_sum                         16,049,100         16,852,320              803,220    5.00%   x 0.95
 iter::bench_inspect_chain_sum                    32,036,133         36,677,341            4,641,208   14.49%   x 0.87
 iter::bench_take_while_chain_ref_sum             25,880,655         27,220,579            1,339,924    5.18%   x 0.95
 iter::bench_zip_copy                             1,710              1,964                       254   14.85%   x 0.87
 num::bench_i32_from_str_radix_36                 272,111            293,915                  21,804    8.01%   x 0.93
 num::bench_u16_from_str_radix_36                 197,457            224,978                  27,521   13.94%   x 0.88
 num::dec2flt::bench_short_decimal                217                233                          16    7.37%   x 0.93
 slice::rotate_16_usize_5                         18,022             19,973                    1,951   10.83%   x 0.90
 slice::rotate_64_usize_5                         318,033            347,852                  29,819    9.38%   x 0.91
 slice::rotate_u8                                 1,941              2,108                       167    8.60%   x 0.92
