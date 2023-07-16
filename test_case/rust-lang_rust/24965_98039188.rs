
Puts implementations in bins hashed by the fast-reject key, and
only looks up the relevant impls, reducing O(n^2)-ishness

Before: 688.92user 5.08system 8:56.70elapsed 129%CPU (0avgtext+0avgdata 1208164maxresident)k, LLVM 379.142s
After: 637.78user 5.11system 8:17.48elapsed 129%CPU (0avgtext+0avgdata 1201448maxresident)k LLVM 375.552s

Performance increase is +7%-ish
