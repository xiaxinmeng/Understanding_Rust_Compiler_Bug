
$ hyperfine -w 1 -m 3 -p 'rm target -rf' 'cargo +stage1 build --release' 'cargo +stage1.2 build --release'
Benchmark #1: cargo +stage1 build --release

  Time (mean ± σ):      3.532 s ±  0.014 s    [User: 6.576 s, System: 0.115 s]
 
  Range (min … max):    3.520 s …  3.548 s
 
Benchmark #2: cargo +stage1.2 build --release

  Time (mean ± σ):      3.166 s ±  0.059 s    [User: 6.180 s, System: 0.137 s]
 
  Range (min … max):    3.122 s …  3.233 s
 
Summary

  'cargo +stage1.2 build --release' ran
    1.12x faster than 'cargo +stage1 build --release'
