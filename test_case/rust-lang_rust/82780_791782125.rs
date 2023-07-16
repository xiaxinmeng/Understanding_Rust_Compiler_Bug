plain
  0     0    0     0    0     0      0      0 --:--:--  0:00:03 --:--:--     0
100     9  100     9    0     0      2      0  0:00:04  0:00:04 --:--:--     2
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib /tmp/externs.rs
error: expected one of `!` or `::`, found `/`
  |
1 | I/O error
1 | I/O error
  |  ^ expected one of `!` or `::`
error: aborting due to previous error

