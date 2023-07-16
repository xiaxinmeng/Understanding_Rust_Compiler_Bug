
➜  hashmap2 git:(layout) ✗ cargo benchcmp hhkkvv:: hhkvkv:: bench.txt                                           
 name                           hhkkvv:: ns/iter  hhkvkv:: ns/iter  diff ns/iter   diff % 
 iter_keys_100_000              391,677           382,839                 -8,838   -2.26% 
 iter_keys_1_000_000            10,797,360        10,209,898            -587,462   -5.44% 
 iter_keys_big_value_100_000    414,736           662,255                247,519   59.68% 
 iter_keys_big_value_1_000_000  10,147,837        12,067,938           1,920,101   18.92% 
 iter_values_100_000            440,445           377,080                -63,365  -14.39% 
 iter_values_1_000_000          10,931,844        9,979,173             -952,671   -8.71% 
 iterate_100_000                428,644           388,509                -40,135   -9.36% 
 iterate_1_000_000              11,065,419        10,042,427          -1,022,992   -9.24%
