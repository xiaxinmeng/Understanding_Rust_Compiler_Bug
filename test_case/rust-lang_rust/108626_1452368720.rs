sh
Build completed successfully in 0:00:15

 Performance counter stats for '../../rust/x doc --stage 1 std --json':

         15,873.45 msec task-clock:u              #    1.000 CPUs utilized
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
           419,374      page-faults:u             #    0.026 M/sec
    62,660,633,799      cycles:u                  #    3.948 GHz                      (83.38%)
       512,794,289      stalled-cycles-frontend:u #    0.82% frontend cycles idle     (83.39%)
        80,806,547      stalled-cycles-backend:u  #    0.13% backend cycles idle      (83.38%)
    82,024,064,071      instructions:u            #    1.31  insn per cycle
                                                  #    0.01  stalled cycles per insn  (83.36%)
    16,035,947,363      branches:u                # 1010.237 M/sec                    (83.38%)
       201,578,639      branch-misses:u           #    1.26% of all branches          (83.32%)

      15.867989639 seconds time elapsed

      14.909949000 seconds user
       0.826553000 seconds sys
