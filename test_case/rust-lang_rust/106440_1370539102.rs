
# master
$ ./x test tidy && hyperfine -w 5 '"build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "." "build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "48"'
Benchmark #1: "build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "." "build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "48"
  Time (mean ± σ):     612.7 ms ±  20.5 ms    [User: 2.189 s, System: 1.378 s]
  Range (min … max):   572.2 ms … 643.1 ms    10 runs


# this PR
Benchmark #1: "build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "." "build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "48"
  Time (mean ± σ):     607.8 ms ±  32.2 ms    [User: 2.635 s, System: 1.411 s]
  Range (min … max):   559.6 ms … 658.3 ms    10 runs


# file-parallel branch
Benchmark #1: "build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "." "build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "48"
  Time (mean ± σ):     635.7 ms ±  24.6 ms    [User: 3.046 s, System: 1.639 s]
  Range (min … max):   597.7 ms … 687.2 ms    10 runs
