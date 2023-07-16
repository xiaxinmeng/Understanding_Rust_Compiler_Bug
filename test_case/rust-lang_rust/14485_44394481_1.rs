
~/rust/coreutils/cat$ perf stat ./cat -v /tmp/blabla.txt > /dev/null

 Performance counter stats for './cat -v /tmp/blabla.txt':

       3283.608533      task-clock (msec)         #    1.001 CPUs utilized          
                 7      context-switches          #    0.002 K/sec                  
                21      cpu-migrations            #    0.006 K/sec                  
               336      page-faults               #    0.102 K/sec                  
     7,105,046,317      cycles                    #    2.164 GHz                    
     1,729,009,226      stalled-cycles-frontend   #   24.33% frontend cycles idle   
     1,119,307,210      stalled-cycles-backend    #   15.75% backend  cycles idle   
    12,545,235,483      instructions              #    1.77  insns per cycle        
                                                  #    0.14  stalled cycles per insn
     3,403,847,620      branches                  # 1036.618 M/sec                  
        78,237,041      branch-misses             #    2.30% of all branches        

       3.281030203 seconds time elapsed
