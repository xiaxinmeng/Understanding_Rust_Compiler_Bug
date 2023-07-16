text
$ hyperfine 'rustc -C opt-level=2 main_500.rs'
Benchmark #1: rustc -C opt-level=2 main_500.rs
  Time (mean ± σ):      3.277 s ±  0.044 s    [User: 3.272 s, System: 0.067 s]
  Range (min … max):    3.212 s …  3.370 s    10 runs
