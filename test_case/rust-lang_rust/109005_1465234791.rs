
Benchmark 1: rm -r target && cargo +1cc5bddfa7ccb13bce2c17c099baba2111a2dc22 check
  Time (mean ± σ):     302.3 ms ±   5.7 ms    [User: 241.6 ms, System: 56.7 ms]
  Range (min … max):   291.7 ms … 317.8 ms    50 runs

Benchmark 2: rm -r target && cargo +150cb381471533050751111e5faf1d9f05c02f77 check
  Time (mean ± σ):     292.0 ms ±   5.9 ms    [User: 234.4 ms, System: 53.4 ms]
  Range (min … max):   282.1 ms … 305.8 ms    50 runs

Summary
  'rm -r target && cargo +150cb381471533050751111e5faf1d9f05c02f77 check' ran
    1.04 ± 0.03 times faster than 'rm -r target && cargo +1cc5bddfa7ccb13bce2c17c099baba2111a2dc22 check'
