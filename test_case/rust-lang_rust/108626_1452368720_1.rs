sh
Build completed successfully in 0:00:15

 Performance counter stats for '../../rust/x doc --stage 1 std --json':

         15,548.93 msec task-clock:u              #    1.000 CPUs utilized
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
           423,520      page-faults:u             #    0.027 M/sec
    61,458,365,022      cycles:u                  #    3.953 GHz                      (83.37%)
       505,423,215      stalled-cycles-frontend:u #    0.82% frontend cycles idle     (83.37%)
       710,815,053      stalled-cycles-backend:u  #    1.16% backend cycles idle      (83.40%)
    81,826,900,373      instructions:u            #    1.33  insn per cycle
                                                  #    0.01  stalled cycles per insn  (83.31%)
    15,968,492,682      branches:u                # 1026.984 M/sec                    (83.35%)
       201,022,039      branch-misses:u           #    1.26% of all branches          (83.40%)

      15.542970326 seconds time elapsed

      14.672585000 seconds user
       0.740135000 seconds sys
