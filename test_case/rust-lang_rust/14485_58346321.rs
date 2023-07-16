
$ perf stat -r10 ./cat-noline -v ~/garbage  > /dev/null

 Performance counter stats for './cat-noline -v /home/bs/garbage' (10 runs):

        941.592477      task-clock (msec)         #    1.000 CPUs utilized            ( +-  0.78% )
                 1      context-switches          #    0.001 K/sec                    ( +- 11.11% )
                 1      cpu-migrations            #    0.001 K/sec                    ( +- 44.44% )
               146      page-faults               #    0.155 K/sec                    ( +-  0.31% )
     3,623,287,334      cycles                    #    3.848 GHz                      ( +-  0.18% )
     1,554,100,197      stalled-cycles-frontend   #   42.89% frontend cycles idle     ( +-  0.38% )
   <not supported>      stalled-cycles-backend   
     4,733,907,018      instructions              #    1.31  insns per cycle        
                                                  #    0.33  stalled cycles per insn  ( +-  0.00% )
     1,366,195,900      branches                  # 1450.942 M/sec                    ( +-  0.00% )
        80,324,010      branch-misses             #    5.88% of all branches          ( +-  0.01% )

       0.941340757 seconds time elapsed                                          ( +-  0.78% )
