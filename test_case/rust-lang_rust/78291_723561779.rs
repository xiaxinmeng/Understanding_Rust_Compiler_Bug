
> cargo benchcmp bench.old bench.new
 name                  bench.old ns/iter  bench.new ns/iter  diff ns/iter   diff %  speedup 
 part_dedup_first_l1   382                1,634                     1,252  327.75%   x 0.23 
 part_dedup_first_l2   3,777              16,612                   12,835  339.82%   x 0.23 
 part_dedup_first_l3   521,583            1,773,709             1,252,126  240.06%   x 0.29 
 part_dedup_last_l1    759                518                        -241  -31.75%   x 1.47 
 part_dedup_last_l2    7,546              5,353                    -2,193  -29.06%   x 1.41 
 part_dedup_last_l3    500,212            1,007,431               507,219  101.40%   x 0.50 
 part_dedup_same_l1    261                258                          -3   -1.15%   x 1.01 
 part_dedup_same_l2    2,564              2,524                       -40   -1.56%   x 1.02 
 part_dedup_same_l3    564,641            467,216                 -97,425  -17.25%   x 1.21 
 part_dedup_unique_l1  382                512                         130   34.03%   x 0.75 
 part_dedup_unique_l2  3,778              5,346                     1,568   41.50%   x 0.71 
 part_dedup_unique_l3  524,809            1,002,029               477,220   90.93%   x 0.52 
