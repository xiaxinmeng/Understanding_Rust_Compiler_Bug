
$ perf stat -d ./target/release/bench-aa0c4d89a4b32e4a --bench portable
running 4 tests
test bench_1mb_blake2b_portable   ... bench:   2,751,321 ns/iter (+/- 41,372) = 381 MB/s
test bench_1mb_blake2s_portable   ... bench:   4,524,327 ns/iter (+/- 45,675) = 231 MB/s
test bench_block_blake2b_portable ... bench:         373 ns/iter (+/- 4) = 343 MB/s
test bench_block_blake2s_portable ... bench:         306 ns/iter (+/- 6) = 209 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 16 filtered out


 Performance counter stats for './target/release/bench-aa0c4d89a4b32e4a --bench portable':

          2,607.89 msec task-clock:u              #    0.999 CPUs utilized          
                 0      context-switches:u        #    0.000 K/sec                  
                 0      cpu-migrations:u          #    0.000 K/sec                  
               651      page-faults:u             #    0.250 K/sec                  
     4,123,567,018      cycles:u                  #    1.581 GHz                      (49.88%)
    12,672,935,897      instructions:u            #    3.07  insn per cycle           (62.41%)
        46,301,922      branches:u                #   17.755 M/sec                    (62.44%)
            39,487      branch-misses:u           #    0.09% of all branches          (62.55%)
     2,096,214,856      L1-dcache-loads:u         #  803.798 M/sec                    (62.64%)
        10,038,091      L1-dcache-load-misses:u   #    0.48% of all L1-dcache hits    (62.64%)
           175,780      LLC-loads:u               #    0.067 M/sec                    (49.98%)
             1,756      LLC-load-misses:u         #    1.00% of all LL-cache hits     (49.87%)

       2.609665643 seconds time elapsed

       2.589340000 seconds user
       0.003288000 seconds sys
