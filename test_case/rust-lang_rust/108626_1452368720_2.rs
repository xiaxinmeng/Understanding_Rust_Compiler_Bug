sh
Build completed successfully in 0:00:15

 Performance counter stats for '../../rust/x doc --stage 1 std --json':

         15,617.23 msec task-clock:u              #    1.000 CPUs utilized
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
           425,083      page-faults:u             #    0.027 M/sec
    61,592,835,803      cycles:u                  #    3.944 GHz                      (83.39%)
       490,297,782      stalled-cycles-frontend:u #    0.80% frontend cycles idle     (83.37%)
       727,341,887      stalled-cycles-backend:u  #    1.18% backend cycles idle      (83.38%)
    81,851,581,044      instructions:u            #    1.33  insn per cycle
                                                  #    0.01  stalled cycles per insn  (83.37%)
    15,975,646,762      branches:u                # 1022.950 M/sec                    (83.38%)
       202,179,644      branch-misses:u           #    1.27% of all branches          (83.31%)

      15.609654475 seconds time elapsed

      14.611848000 seconds user
       0.869682000 seconds sys
