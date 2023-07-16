
 Performance counter stats for 'taskset -c 10 setarch -R cargo +1.53.0 run --release':

         38,401.81 msec task-clock                #    0.993 CPUs utilized          
             4,146      context-switches          #  107.964 /sec                   
                 1      cpu-migrations            #    0.026 /sec                   
         1,836,379      page-faults               #   47.820 K/sec                  
   145,282,395,661      cycles                    #    3.783 GHz                      (83.31%)
     8,635,115,711      stalled-cycles-frontend   #    5.94% frontend cycles idle     (83.33%)
    14,091,004,039      stalled-cycles-backend    #    9.70% backend cycles idle      (83.35%)
   276,568,526,127      instructions              #    1.90  insn per cycle         
                                                  #    0.05  stalled cycles per insn  (83.35%)
    58,882,876,033      branches                  #    1.533 G/sec                    (83.34%)
     1,668,071,098      branch-misses             #    2.83% of all branches          (83.33%)

      38.683641018 seconds time elapsed

      33.289333000 seconds user
       4.982310000 seconds sys



 Performance counter stats for 'taskset -c 10 setarch -R cargo +1.55.0 run --release':

         38,329.38 msec task-clock                #    0.993 CPUs utilized          
             4,100      context-switches          #  106.968 /sec                   
                 1      cpu-migrations            #    0.026 /sec                   
         1,836,327      page-faults               #   47.909 K/sec                  
   145,022,048,731      cycles                    #    3.784 GHz                      (83.35%)
     8,081,468,638      stalled-cycles-frontend   #    5.57% frontend cycles idle     (83.33%)
     9,507,856,383      stalled-cycles-backend    #    6.56% backend cycles idle      (83.33%)
   276,509,854,969      instructions              #    1.91  insn per cycle         
                                                  #    0.03  stalled cycles per insn  (83.33%)
    58,998,314,219      branches                  #    1.539 G/sec                    (83.34%)
     1,660,786,933      branch-misses             #    2.81% of all branches          (83.32%)

      38.611771146 seconds time elapsed

      32.925889000 seconds user
       5.274858000 seconds sys


 Performance counter stats for 'taskset -c 10 setarch -R cargo +nightly run --release':

         41,429.33 msec task-clock                #    0.993 CPUs utilized          
             4,388      context-switches          #  105.915 /sec                   
                 1      cpu-migrations            #    0.024 /sec                   
         2,245,367      page-faults               #   54.198 K/sec                  
   156,700,534,422      cycles                    #    3.782 GHz                      (83.32%)
     7,538,796,456      stalled-cycles-frontend   #    4.81% frontend cycles idle     (83.33%)
    18,760,349,317      stalled-cycles-backend    #   11.97% backend cycles idle      (83.35%)
   325,918,991,866      instructions              #    2.08  insn per cycle         
                                                  #    0.06  stalled cycles per insn  (83.34%)
    68,187,995,269      branches                  #    1.646 G/sec                    (83.33%)
     1,707,743,015      branch-misses             #    2.50% of all branches          (83.33%)

      41.716983461 seconds time elapsed

      35.519123000 seconds user
       5.763294000 seconds sys
 