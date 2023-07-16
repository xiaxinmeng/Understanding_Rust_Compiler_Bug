
➜  hashmap2 git:(layout) ✗ cargo benchcmp hhkkvv:: hhkvkv:: bench.txt                                              
 name                             hhkkvv:: ns/iter  hhkvkv:: ns/iter  diff ns/iter   diff % 
 grow_10_000                      826,495           850,066                 23,571    2.85% 
 grow_fnv_10_000                  446,361           409,896                -36,465   -8.17% 
 insert_100                       2,396             2,175                     -221   -9.22% 
 insert_1000                      23,036            22,050                    -986   -4.28% 
 insert_100_000                   4,722,417         3,715,894           -1,006,523  -21.31% 
 insert_10_000                    309,795           287,247                -22,548   -7.28% 
 insert_int_bigvalue_10_000       739,677           408,217               -331,460  -44.81% 
 insert_str_10_000                335,957           332,453                 -3,504   -1.04% 
 insert_string_10_000             783,955           788,231                  4,276    0.55% 
 iter_keys_10_000                 61,065            54,332                  -6,733  -11.03% 
 iter_values_10_000               58,822            53,421                  -5,401   -9.18% 
 iterate_10_000                   62,323            54,329                  -7,994  -12.83% 
 lookup_100_000_multi             190,192           185,332                 -4,860   -2.56% 
 lookup_100_000_multi_bigvalue    193,631           190,318                 -3,313   -1.71% 
 lookup_10_000_exist              125,509           133,545                  8,036    6.40% 
 lookup_10_000_multi              149,410           143,082                 -6,328   -4.24% 
 lookup_10_000_multi_bigvalue     157,097           149,678                 -7,419   -4.72% 
 lookup_10_000_noexist            144,818           145,341                    523    0.36% 
 lookup_1_000_000_multi           135,634           133,530                 -2,104   -1.55% 
 lookup_1_000_000_multi_bigvalue  140,722           135,215                 -5,507   -3.91% 
 merge_shuffle                    1,208,160         1,264,382               56,222    4.65% 
 merge_simple                     40,589,417        38,375,331          -2,214,086   -5.45% 
 new                              6                 5                           -1  -16.67% 
 with_capacity_10e5               3,248             3,199                      -49   -1.51%
