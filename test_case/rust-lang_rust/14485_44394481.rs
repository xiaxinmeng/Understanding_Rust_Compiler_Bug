
~/rust/coreutils/cat$ perf stat ./cat -v /tmp/blabla.txt > /dev/null

 Performance counter stats for './cat -v /tmp/blabla.txt':

       1598.236550      task-clock (msec)         #    0.999 CPUs utilized          
                12      context-switches          #    0.008 K/sec                  
                26      cpu-migrations            #    0.016 K/sec                  
               313      page-faults               #    0.196 K/sec                  
     3,356,501,774      cycles                    #    2.100 GHz                    
     1,378,688,552      stalled-cycles-frontend   #   41.08% frontend cycles idle   
     1,012,720,553      stalled-cycles-backend    #   30.17% backend  cycles idle   
     4,485,975,285      instructions              #    1.34  insns per cycle        
                                                  #    0.31  stalled cycles per insn
     1,435,550,790      branches                  #  898.209 M/sec                  
        76,588,047      branch-misses             #    5.34% of all branches        

       1.599360146 seconds time elapsed
