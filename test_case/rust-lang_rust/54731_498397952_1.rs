
$ perf stat -d ./target/release/bench-5919a355cd983822 --bench portable
running 4 tests
test bench_1mb_blake2b_portable   ... bench:   2,779,936 ns/iter (+/- 16,871) = 377 MB/s
test bench_1mb_blake2s_portable   ... bench:   4,779,937 ns/iter (+/- 51,220) = 219 MB/s
test bench_block_blake2b_portable ... bench:         374 ns/iter (+/- 2) = 342 MB/s
test bench_block_blake2s_portable ... bench:         319 ns/iter (+/- 1) = 200 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 16 filtered out


 Performance counter stats for './target/release/bench-5919a355cd983822 --bench portable':

          2,802.36 msec task-clock:u              #    1.000 CPUs utilized          
                 0      context-switches:u        #    0.000 K/sec                  
                 0      cpu-migrations:u          #    0.000 K/sec                  
               652      page-faults:u             #    0.233 K/sec                  
     4,442,001,169      cycles:u                  #    1.585 GHz                      (49.90%)
    13,487,054,371      instructions:u            #    3.04  insn per cycle           (62.43%)
        50,421,665      branches:u                #   17.993 M/sec                    (62.43%)
            45,927      branch-misses:u           #    0.09% of all branches          (62.44%)
     2,378,582,828      L1-dcache-loads:u         #  848.778 M/sec                    (62.55%)
        10,076,671      L1-dcache-load-misses:u   #    0.42% of all L1-dcache hits    (62.62%)
           271,738      LLC-loads:u               #    0.097 M/sec                    (50.08%)
             1,836      LLC-load-misses:u         #    0.68% of all LL-cache hits     (49.98%)

       2.802961980 seconds time elapsed

       2.781578000 seconds user
       0.006615000 seconds sys
