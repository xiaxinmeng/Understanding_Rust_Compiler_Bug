
test iter::bench_skip_chain_ref_sum                                ... bench:   2,111,466 ns/iter (+/- 62,704)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 411 filtered out; finished in 4.59s

	finished in 4.744 seconds
Build completed successfully in 0:00:05

 Performance counter stats for './x bench --keep-stage 0 --stage 1 library/core/ --test-args iter::bench_skip_chain_ref_sum':

          5,541.40 msec task-clock                       #    1.000 CPUs utilized          
               777      context-switches                 #  140.217 /sec                   
               151      cpu-migrations                   #   27.249 /sec                   
            71,916      page-faults                      #   12.978 K/sec                  
    21,760,400,089      cycles                           #    3.927 GHz                      (83.85%)
       247,295,961      stalled-cycles-frontend          #    1.14% frontend cycles idle     (84.11%)
    11,342,182,645      stalled-cycles-backend           #   52.12% backend cycles idle      (83.88%)
   108,241,690,775      instructions                     #    4.97  insn per cycle         
                                                  #    0.10  stalled cycles per insn  (84.00%)
    13,281,399,988      branches                         #    2.397 G/sec                    (84.11%)
        13,076,998      branch-misses                    #    0.10% of all branches          (83.17%)

       5.538899860 seconds time elapsed

       5.256745000 seconds user
       0.274922000 seconds sys
