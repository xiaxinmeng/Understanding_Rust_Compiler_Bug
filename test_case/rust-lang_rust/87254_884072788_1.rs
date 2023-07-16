
$ hyperfine -w1 -r3 -s basic "~/rusttest/master/bin/rustc -C opt-level=3 -C codegen-units=1 src/main.rs" "~/rusttest/new/bin/rustc -C opt-level=3 -C codegen-units=1 src/main.rs"
Benchmark #1: ~/rusttest/master/bin/rustc -C opt-level=3 -C codegen-units=1 src/main.rs
  Time (mean ± σ):      3.841 s ±  0.007 s    [User: 3.721 s, System: 0.128 s]
  Range (min … max):    3.834 s …  3.847 s    3 runs

Benchmark #2: ~/rusttest/new/bin/rustc -C opt-level=3 -C codegen-units=1 src/main.rs
  Time (mean ± σ):      3.847 s ±  0.019 s    [User: 3.722 s, System: 0.133 s]
  Range (min … max):    3.829 s …  3.867 s    3 runs

Summary
  '~/rusttest/master/bin/rustc -C opt-level=3 -C codegen-units=1 src/main.rs' ran
    1.00 ± 0.01 times faster than '~/rusttest/new/bin/rustc -C opt-level=3 -C codegen-units=1 src/main.rs'
