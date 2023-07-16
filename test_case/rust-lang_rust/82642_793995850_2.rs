
Benchmark #1: PATH=/Users/eric/.rustup/toolchains/nightly-x86_64-apple-darwin/bin:$PATH cargo build
  Time (mean ± σ):     19.493 s ±  0.057 s    [User: 126.485 s, System: 8.628 s]
  Range (min … max):   19.452 s … 19.533 s    2 runs

Benchmark #2: PATH=/Users/eric/.rustup/toolchains/nightly-2021-03-08-x86_64-apple-darwin/bin:$PATH cargo build
  Time (mean ± σ):     22.563 s ±  0.665 s    [User: 147.224 s, System: 9.124 s]
  Range (min … max):   22.093 s … 23.033 s    2 runs

Summary
  'PATH=/Users/eric/.rustup/toolchains/nightly-x86_64-apple-darwin/bin:$PATH cargo build' ran
    1.16 ± 0.03 times faster than 'PATH=/Users/eric/.rustup/toolchains/nightly-2021-03-08-x86_64-apple-darwin/bin:$PATH cargo build'
