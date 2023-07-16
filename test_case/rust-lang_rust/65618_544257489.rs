
...
   Compiling bench-test v0.1.0 (/tmp/crittest)
     Running `rustc --edition=2018 --crate-name bench_test bench_name.rs --error-format=json --json=diagnostic-rendered-ansi --emit=dep-info,link -C opt-level=3 --cfg test -C metadata=8c71f5ab08445b57 -C extra-filename=-8c71f5ab08445b57 --out-dir /tmp/crittest/target/release/deps -L dependency=/tmp/crittest/target/release/deps --extern bench_test=/tmp/crittest/target/release/deps/libbench_test-0f2595cb3983be9b.rlib --extern criterion=/tmp/crittest/target/release/deps/libcriterion-0d07609906f00047.rlib -Ctarget-cpu=znver1`
    Finished bench [optimized] target(s) in 2.26s
     Running `/tmp/crittest/target/release/deps/bench_test-8c71f5ab08445b57 --bench`
bench/old               time:   [0.0000 ps 0.0000 ps 0.0000 ps]                               
                        change: [-53.418% -6.0379% +85.594%] (p = 0.87 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
bench/new               time:   [0.0000 ps 0.0000 ps 0.0000 ps]                               
                        change: [-52.325% -7.9366% +80.704%] (p = 0.82 > 0.05)
                        No change in performance detected.
Found 20 outliers among 100 measurements (20.00%)
  6 (6.00%) high mild
  14 (14.00%) high severe
