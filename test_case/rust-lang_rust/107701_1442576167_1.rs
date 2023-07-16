
running 1 test
test iter::bench_skip_chain_ref_sum                                ... bench:   1,673,832 ns/iter (+/- 5,689)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 411 filtered out; finished in 0.50s


 Performance counter stats for './beb9614c03b7/bin/corebenches-ac3e1e8c078c7482 --bench iter::bench_skip_chain_ref_sum':

            504.67 msec task-clock:u                     #    1.000 CPUs utilized
                 0      context-switches:u               #    0.000 /sec
                 0      cpu-migrations:u                 #    0.000 /sec
               173      page-faults:u                    #  342.798 /sec
     2,412,468,602      cycles:u                         #    4.780 GHz                      (83.24%)
           469,826      stalled-cycles-frontend:u        #    0.02% frontend cycles idle     (83.36%)
               153      stalled-cycles-backend:u         #    0.00% backend cycles idle      (83.36%)
    15,038,910,520      instructions:u                   #    6.23  insn per cycle
                                                  #    0.00  stalled cycles per insn  (83.36%)
     1,804,833,553      branches:u                       #    3.576 G/sec                    (83.36%)
            19,104      branch-misses:u                  #    0.00% of all branches          (83.34%)

       0.504889089 seconds time elapsed

       0.502324000 seconds user
       0.000000000 seconds sys
