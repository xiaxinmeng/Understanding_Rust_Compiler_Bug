
Benchmark #1: PATH=/Users/eric/.rustup/toolchains/nightly-x86_64-apple-darwin/bin:$PATH cargo build
  Time (mean ± σ):      5.968 s ±  0.010 s    [User: 7.561 s, System: 0.493 s]
  Range (min … max):    5.961 s …  5.975 s    2 runs

Benchmark #2: PATH=/Users/eric/.rustup/toolchains/nightly-2021-03-08-x86_64-apple-darwin/bin:$PATH cargo build
  Time (mean ± σ):      6.777 s ±  0.049 s    [User: 8.598 s, System: 0.509 s]
  Range (min … max):    6.743 s …  6.812 s    2 runs

Summary
  'PATH=/Users/eric/.rustup/toolchains/nightly-x86_64-apple-darwin/bin:$PATH cargo build' ran
    1.14 ± 0.01 times faster than 'PATH=/Users/eric/.rustup/toolchains/nightly-2021-03-08-x86_64-apple-darwin/bin:$PATH cargo build'
