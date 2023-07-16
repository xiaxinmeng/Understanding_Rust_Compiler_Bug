
thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
error: could not compile `mycrate`.

Caused by:
  process didn't exit successfully: `rustc --crate-name crash2 --edition=2018 src/bin/crash2.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=20e32ce06e01ca6b -C extra-filename=-20e32ce06e01ca6b --out-dir /Users/alex/workspace/const-generics-stackoverflow-test/target/debug/deps -C incremental=/Users/alex/workspace/const-generics-stackoverflow-test/target/debug/incremental -L dependency=/Users/alex/workspace/const-generics-stackoverflow-test/target/debug/deps --extern mycrate=/Users/alex/workspace/const-generics-stackoverflow-test/target/debug/deps/libmycrate-8f49fe4478012ba3.rlib` (signal: 6, SIGABRT: process abort signal)
