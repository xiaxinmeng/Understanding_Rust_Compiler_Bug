
------------------------------------------------
Stage 1 (LLVM PGO):            1936.16s (21.44%)
  Build rustc and LLVM:        1447.07s (16.02%)
    LLVM:                       322.39s ( 3.57%)
    Rustc:                     1105.31s (12.24%)
  Gather profiles:              489.10s ( 5.41%)
Stage 2 (rustc PGO):           1579.59s (17.49%)
  Build rustc and LLVM:         650.87s ( 7.21%)
    LLVM:                       290.31s ( 3.21%)
    Rustc:                      342.16s ( 3.79%)
  Gather profiles:              928.72s (10.28%)
Stage 3 (LLVM BOLT):           2722.61s (30.14%)
  Build rustc and LLVM:        1892.67s (20.95%)
    LLVM:                      1141.65s (12.64%)
    Rustc:                      725.08s ( 8.03%)
  Gather profiles:              829.94s ( 9.19%)
Stage 4 (final build):         2794.01s (30.93%)
  LLVM:                         984.42s (10.90%)
  Rustc:                        414.21s ( 4.59%)
                                                
Total duration:                       2h 30m 32s
------------------------------------------------
