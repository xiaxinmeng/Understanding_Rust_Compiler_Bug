
~/rust/reverse-complement$ perf stat ./rust < ./input.txt > /dev/null

 Performance counter stats for './rust':

        758.568211      task-clock (msec)         #    1.140 CPUs utilized          
                80      context-switches          #    0.105 K/sec                  
                32      cpu-migrations            #    0.042 K/sec                  
           123,117      page-faults               #    0.162 M/sec                  
     1,580,569,070      cycles                    #    2.084 GHz                    
       849,347,730      stalled-cycles-frontend   #   53.74% frontend cycles idle   
       587,892,723      stalled-cycles-backend    #   37.20% backend  cycles idle   
     2,107,021,931      instructions              #    1.33  insns per cycle        
                                                  #    0.40  stalled cycles per insn
       330,187,928      branches                  #  435.278 M/sec                  
           264,699      branch-misses             #    0.08% of all branches        

       0.665397382 seconds time elapsed

~/rust/reverse-complement$ perf stat ./c < ./input.txt > /dev/null   

 Performance counter stats for './c':

        659.239763      task-clock (msec)         #    1.265 CPUs utilized          
               103      context-switches          #    0.156 K/sec                  
                25      cpu-migrations            #    0.038 K/sec                  
            60,101      page-faults               #    0.091 M/sec                  
     1,367,562,395      cycles                    #    2.074 GHz                    
       491,611,151      stalled-cycles-frontend   #   35.95% frontend cycles idle   
       288,809,513      stalled-cycles-backend    #   21.12% backend  cycles idle   
     2,555,179,319      instructions              #    1.87  insns per cycle        
                                                  #    0.19  stalled cycles per insn
       480,482,167      branches                  #  728.843 M/sec                  
           134,052      branch-misses             #    0.03% of all branches        

       0.521284864 seconds time elapsed
