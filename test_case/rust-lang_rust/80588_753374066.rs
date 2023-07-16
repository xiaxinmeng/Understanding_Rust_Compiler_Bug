
listless% hyperfine ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc rustc 
Benchmark #1: /home/dsilvers/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
  Time (mean ± σ):      11.3 ms ±   0.9 ms    [User: 6.9 ms, System: 4.3 ms]
  Range (min … max):    10.1 ms …  15.4 ms    252 runs
 
Benchmark #2: rustc
  Time (mean ± σ):      71.2 ms ±   2.3 ms    [User: 60.0 ms, System: 11.2 ms]
  Range (min … max):    67.3 ms …  76.2 ms    41 runs
 
Summary
  '/home/dsilvers/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc' ran
    6.31 ± 0.54 times faster than 'rustc'
listless%                                                                                

