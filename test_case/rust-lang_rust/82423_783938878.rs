
Benchmark #1: a.sh — system malloc
  Time (mean ± σ):     19.119 s ±  0.058 s    [User: 89.221 s, System: 6.249 s]
  Range (min … max):   19.070 s … 19.183 s    3 runs

Benchmark #2: b.sh — jemalloc + patch
  Time (mean ± σ):     17.462 s ±  0.030 s    [User: 80.078 s, System: 5.993 s]
  Range (min … max):   17.441 s … 17.496 s    3 runs

Benchmark #3: c.sh — nightly-2021-02-22
  Time (mean ± σ):     22.273 s ±  0.053 s    [User: 105.825 s, System: 5.951 s]
  Range (min … max):   22.211 s … 22.305 s    3 runs

Summary
  'b.sh' ran
    1.09 ± 0.00 times faster than 'a.sh'
    1.28 ± 0.00 times faster than 'c.sh'
