
running 1 test
test iter::bench_skip_chain_ref_sum                                ... bench:   1,281,926 ns/iter (+/- 13,244)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 411 filtered out; finished in 5.94s


 Performance counter stats for './7a9805368ebc/bin/corebenches-ac3e1e8c078c7482 --bench iter::bench_skip_chain_ref_sum':

          5,942.66 msec task-clock:u                     #    1.000 CPUs utilized
                 0      context-switches:u               #    0.000 /sec
                 0      cpu-migrations:u                 #    0.000 /sec
               176      page-faults:u                    #   29.616 /sec
    27,870,211,887      cycles:u                         #    4.690 GHz                      (83.32%)
         8,065,226      stalled-cycles-frontend:u        #    0.03% frontend cycles idle     (83.32%)
            13,336      stalled-cycles-backend:u         #    0.00% backend cycles idle      (83.33%)
   166,482,113,215      instructions:u                   #    5.97  insn per cycle
                                                  #    0.00  stalled cycles per insn  (83.34%)
    26,990,127,586      branches:u                       #    4.542 G/sec                    (83.34%)
            47,550      branch-misses:u                  #    0.00% of all branches          (83.34%)

       5.943147770 seconds time elapsed

       5.909855000 seconds user
       0.000984000 seconds sys
