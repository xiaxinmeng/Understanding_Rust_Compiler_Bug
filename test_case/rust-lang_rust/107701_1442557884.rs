
running 1 test
test iter::bench_skip_chain_ref_sum                                ... bench:   3,902,217 ns/iter (+/- 151,106)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 411 filtered out; finished in 3.55s

	finished in 3.694 seconds
Build completed successfully in 0:00:04

 Performance counter stats for './x bench --keep-stage 0 --stage 1 library/core/ --test-args iter::bench_skip_chain_ref_sum':

          4,486.33 msec task-clock                       #    1.001 CPUs utilized          
               678      context-switches                 #  151.126 /sec                   
               152      cpu-migrations                   #   33.881 /sec                   
            72,849      page-faults                      #   16.238 K/sec                  
    17,260,463,951      cycles                           #    3.847 GHz                      (84.07%)
       238,723,965      stalled-cycles-frontend          #    1.38% frontend cycles idle     (84.13%)
    11,449,836,029      stalled-cycles-backend           #   66.34% backend cycles idle      (83.68%)
    37,087,407,432      instructions                     #    2.15  insn per cycle         
                                                  #    0.31  stalled cycles per insn  (83.83%)
     6,130,965,765      branches                         #    1.367 G/sec                    (84.06%)
        14,031,568      branch-misses                    #    0.23% of all branches          (84.13%)

       4.482230133 seconds time elapsed

       4.198007000 seconds user
       0.281285000 seconds sys
