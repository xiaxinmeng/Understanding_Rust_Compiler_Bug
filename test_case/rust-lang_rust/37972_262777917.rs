
name                            before-2 ns/iter  after-2 ns/iter  diff ns/iter   diff % 
 count_vec_10k                   8,361             8,382                      21    0.25% 
 count_vec_10k_maybe_incomplete  8,182             8,196                      14    0.17% 
 count_vec_1k                    867               865                        -2   -0.23% 
 float_f32                       26,866            27,112                    246    0.92% 
 float_f64                       1,052             1,068                      16    1.52% 
 float_no_conversion             92                64                        -28  -30.43% 
 float_small_f32                 54                54                          0    0.00% 
 float_small_f64                 55                54                         -1   -1.82% 
 float_small_no_conversion       22                23                          1    4.55% 
 from_str_f32                    28,417            27,089                 -1,328   -4.67% 
 from_str_f64                    1,007             987                       -20   -1.99% 
 from_str_small_f32              29                29                          0    0.00% 
 from_str_small_f64              30                29                         -1   -3.33% 
 many1_vec_10k                   10,057            10,070                     13    0.13% 
 many1_vec_10k_maybe_incomplete  10,126            10,115                    -11   -0.11% 
 many1_vec_1k                    1,422             1,415                      -7   -0.49% 
 many_vec_10k                    9,082             9,086                       4    0.04% 
 many_vec_10k_maybe_incomplete   9,355             9,138                    -217   -2.32% 
 many_vec_1k                     1,272             1,236                     -36   -2.83% 
 match_float                     70                43                        -27  -38.57% 
 multiple_requests               53,834            46,884                 -6,950  -12.91% 
 single_request                  786               678                      -108  -13.74% 
 single_request_large            1,195             1,008                    -187  -15.65% 
 single_request_minimal          117               118                         1    0.85%
