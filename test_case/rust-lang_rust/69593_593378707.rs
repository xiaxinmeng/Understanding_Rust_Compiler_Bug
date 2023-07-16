
$ perf stat -d target/release/struct
1999999999 in 1012 ms

 Performance counter stats for 'target/release/struct':

          1,007.20 msec task-clock:u              #    0.994 CPUs utilized          
                 0      context-switches:u        #    0.000 K/sec                  
                 0      cpu-migrations:u          #    0.000 K/sec                  
                77      page-faults:u             #    0.076 K/sec                  
     3,187,890,051      cycles:u                  #    3.165 GHz                      (49.87%)
    11,991,292,654      instructions:u            #    3.76  insn per cycle           (62.40%)
     1,998,678,625      branches:u                # 1984.399 M/sec                    (62.51%)
            10,936      branch-misses:u           #    0.00% of all branches          (62.51%)
           211,202      L1-dcache-loads:u         #    0.210 M/sec                    (62.60%)
             9,782      L1-dcache-load-misses:u   #    4.63% of all L1-dcache hits    (62.57%)
             1,063      LLC-loads:u               #    0.001 M/sec                    (50.01%)
                46      LLC-load-misses:u         #    4.33% of all LL-cache hits     (49.93%)

       1.013577794 seconds time elapsed

       0.997534000 seconds user
       0.003969000 seconds sys


$ perf stat -d target/release/trait
1999999999 in 1283 ms

 Performance counter stats for 'target/release/trait':

          1,283.87 msec task-clock:u              #    1.000 CPUs utilized          
                 0      context-switches:u        #    0.000 K/sec                  
                 0      cpu-migrations:u          #    0.000 K/sec                  
                75      page-faults:u             #    0.058 K/sec                  
     4,039,253,928      cycles:u                  #    3.146 GHz                      (49.88%)
    12,000,040,451      instructions:u            #    2.97  insn per cycle           (62.42%)
     2,000,434,927      branches:u                # 1558.131 M/sec                    (62.49%)
            14,333      branch-misses:u           #    0.00% of all branches          (62.57%)
           210,909      L1-dcache-loads:u         #    0.164 M/sec                    (62.61%)
             9,571      L1-dcache-load-misses:u   #    4.54% of all L1-dcache hits    (62.58%)
             1,256      LLC-loads:u               #    0.978 K/sec                    (49.97%)
               473      LLC-load-misses:u         #   37.66% of all LL-cache hits     (49.89%)

       1.284390399 seconds time elapsed

       1.273640000 seconds user
       0.001981000 seconds sys
