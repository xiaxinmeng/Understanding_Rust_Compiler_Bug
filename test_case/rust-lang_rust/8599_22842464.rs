
brian@brian-X1:~/dev/rust-sched-bench$ RUST_THREADS=4 perf stat -- ./pingpong-rust

 Performance counter stats for './pingpong-rust':

       6366.487327 task-clock                #    3.798 CPUs utilized          
            19,277 context-switches          #    0.003 M/sec                  
               255 cpu-migrations            #    0.040 K/sec                  
             1,511 page-faults               #    0.237 K/sec                  
    18,957,488,136 cycles                    #    2.978 GHz                    
    12,453,981,192 stalled-cycles-frontend   #   65.69% frontend cycles idle   
   <not supported> stalled-cycles-backend  
    15,206,638,178 instructions              #    0.80  insns per cycle        
                                             #    0.82  stalled cycles per insn
     3,287,804,280 branches                  #  516.424 M/sec                  
        56,231,051 branch-misses             #    1.71% of all branches        

       1.676367495 seconds time elapsed
