
$ echo "fn main() { (0usize..10_000_000).map(|x| x * x * x * 18913515181 % 1_000_000).map(|x| x as isize).collect::<Vec<_>>().sort(); }" | rustc -O -; and perf stat ./rust_out

 Performance counter stats for './rust_out':

        998.626943      task-clock:u (msec)       #    1.000 CPUs utilized          
                 0      context-switches:u        #    0.000 K/sec                  
                 0      cpu-migrations:u          #    0.000 K/sec                  
            58,695      page-faults:u             #    0.059 M/sec                  
     2,732,796,756      cycles:u                  #    2.737 GHz                    
     4,317,535,452      instructions:u            #    1.58  insn per cycle         
       827,997,970      branches:u                #  829.136 M/sec                  
        10,911,782      branch-misses:u           #    1.32% of all branches        

       0.999124069 seconds time elapsed

$ echo "fn main() { (0usize..10_000_000).map(|x| x * x * x * 18913515181 % 1_000_000).map(|x| x as usize).collect::<Vec<_>>().sort(); }" | rustc -O -; and perf stat ./rust_out

 Performance counter stats for './rust_out':

       1017.731368      task-clock:u (msec)       #    1.000 CPUs utilized          
                 0      context-switches:u        #    0.000 K/sec                  
                 0      cpu-migrations:u          #    0.000 K/sec                  
            58,696      page-faults:u             #    0.058 M/sec                  
     2,818,893,129      cycles:u                  #    2.770 GHz                    
     4,317,536,189      instructions:u            #    1.53  insn per cycle         
       827,998,223      branches:u                #  813.572 M/sec                  
        10,910,360      branch-misses:u           #    1.32% of all branches        

       1.018068951 seconds time elapsed
