
error: Could not compile `preach`.

Caused by:
  process didn't exit successfully: `rustc --crate-name preach src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=10cd77d00eccd0d1 -C extra-filename=-10cd77d00eccd0d1 --out-dir /home/eric/dev/projects/preach/target/debug/deps -C incremental=/home/eric/dev/projects/preach/target/debug/incremental -L dependency=/home/eric/dev/projects/preach/target/debug/deps --extern env_logger=/home/eric/dev/projects/preach/target/debug/deps/libenv_logger-8b8781143d9dedb1.rlib --extern rand=/home/eric/dev/projects/preach/target/debug/deps/librand-7adbc532e0c445b3.rlib --extern getopts=/home/eric/dev/projects/preach/target/debug/deps/libgetopts-bdad2372c882101d.rlib --extern log=/home/eric/dev/projects/preach/target/debug/deps/liblog-bfec1262ec0fbc2e.rlib` (signal: 11, SIGSEGV: invalid memory reference)

