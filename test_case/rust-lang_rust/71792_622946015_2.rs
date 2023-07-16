
➜  wrapping-perf git:(master) ✗ RUSTC=~/opt/bin/rustc cargo -q build && hyperfine './target/debug/wrapping-perf 1'
Benchmark #1: ./target/debug/wrapping-perf 1
  Time (mean ± σ):     368.1 ms ±   0.8 ms    [User: 366.2 ms, System: 1.7 ms]
  Range (min … max):   366.9 ms … 369.0 ms    10 runs
