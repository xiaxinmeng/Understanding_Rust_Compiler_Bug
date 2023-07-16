
$ hyperfine -w1 -r3 -s basic "~/rusttest/master/bin/rustc -C opt-level=3 src/main.rs" "~/rusttest/new/bin/rustc -C opt-level=3 src/main.rs"
Benchmark #1: ~/rusttest/master/bin/rustc -C opt-level=3 src/main.rs
  Time (mean ± σ):      5.123 s ±  0.009 s    [User: 5.015 s, System: 0.196 s]
  Range (min … max):    5.113 s …  5.131 s    3 runs

Benchmark #2: ~/rusttest/new/bin/rustc -C opt-level=3 src/main.rs
  Time (mean ± σ):      5.319 s ±  0.007 s    [User: 5.198 s, System: 0.212 s]
  Range (min … max):    5.313 s …  5.328 s    3 runs

Summary
  '~/rusttest/master/bin/rustc -C opt-level=3 src/main.rs' ran
    1.04 ± 0.00 times faster than '~/rusttest/new/bin/rustc -C opt-level=3 src/main.rs'
