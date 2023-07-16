
Benchmark #1: PATH=/Users/eric/.rustup/toolchains/nightly-x86_64-apple-darwin/bin:$PATH cargo build
  Time (mean ± σ):     66.658 s ±  0.394 s    [User: 299.565 s, System: 27.624 s]
  Range (min … max):   66.380 s … 66.937 s    2 runs

Benchmark #2: PATH=/Users/eric/.rustup/toolchains/nightly-2021-03-08-x86_64-apple-darwin/bin:$PATH cargo build
  Time (mean ± σ):     75.251 s ±  0.283 s    [User: 337.470 s, System: 28.521 s]
  Range (min … max):   75.051 s … 75.451 s    2 runs

Summary
  'PATH=/Users/eric/.rustup/toolchains/nightly-x86_64-apple-darwin/bin:$PATH cargo build' ran
    1.13 ± 0.01 times faster than 'PATH=/Users/eric/.rustup/toolchains/nightly-2021-03-08-x86_64-apple-darwin/bin:$PATH cargo build'
