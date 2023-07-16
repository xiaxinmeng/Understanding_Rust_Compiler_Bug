
$ perf stat -r10 ./cat-inline -v ~/garbage  > /dev/null

 Performance counter stats for './cat-inline -v /home/bs/garbage' (10 runs):

        931.492671      task-clock (msec)         #    1.000 CPUs utilized            ( +-  0.59% )
                 1      context-switches          #    0.001 K/sec                    ( +- 11.11% )
                 1      cpu-migrations            #    0.001 K/sec                    ( +- 30.77% )
               146      page-faults               #    0.157 K/sec                    ( +-  0.23% )
     3,609,375,727      cycles                    #    3.875 GHz                      ( +-  0.25% )
     1,548,644,880      stalled-cycles-frontend   #   42.91% frontend cycles idle     ( +-  0.70% )
   <not supported>      stalled-cycles-backend   
     4,733,869,621      instructions              #    1.31  insns per cycle        
                                                  #    0.33  stalled cycles per insn  ( +-  0.00% )
     1,366,189,092      branches                  # 1466.666 M/sec                    ( +-  0.00% )
        80,322,444      branch-misses             #    5.88% of all branches          ( +-  0.01% )

       0.931251603 seconds time elapsed                                          ( +-  0.59% )
