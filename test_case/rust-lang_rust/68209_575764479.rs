
   Compiling compile-test v0.1.0 (/home/jfrimmel/compile-test)
error: could not compile `compile-test`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name compile_test src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=a40bb9266b098d04 -C extra-filename=-a40bb9266b098d04 --out-dir /home/jfrimmel/compile-test/target/debug/deps -C incremental=/home/jfrimmel/compile-test/target/debug/incremental -L dependency=/home/jfrimmel/compile-test/target/debug/deps` (signal: 9, SIGKILL: kill)
