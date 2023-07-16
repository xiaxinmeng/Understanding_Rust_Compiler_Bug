
Stage 1 (LLVM PGO):            2326.24s (23.29%)
  Build rustc and LLVM:        1729.07s (17.31%)
  Gather profiles:              597.17s ( 5.98%)
Stage 2 (rustc PGO):           1772.25s (17.74%)
  Build rustc and LLVM:         697.05s ( 6.98%)
  Gather profiles:             1075.20s (10.76%)
Stage 3 (LLVM BOLT):           2924.91s (29.28%)
  Build rustc and LLVM:        2102.94s (21.05%)
  Gather profiles:              821.96s ( 8.23%)
Stage 4 (final build):         2966.38s (29.69%)

Total duration:                      2h 46m 30s
