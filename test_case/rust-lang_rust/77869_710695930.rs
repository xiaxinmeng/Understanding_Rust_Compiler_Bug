zsh
$ cargo build
   Compiling legion_testing v0.1.0 (/home/overminddl1/rust/legion_testing)
error: could not compile `legion_testing`.

Caused by:
  process didn't exit successfully: `rustc --crate-name legion_testing --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=668da26770ceeea9 -C extra-filename=-668da26770ceeea9 --out-dir /home/overminddl1/rust/legion_testing/target/debug/deps -C incremental=/home/overminddl1/rust/legion_testing/target/debug/incremental -L dependency=/home/overminddl1/rust/legion_testing/target/debug/deps` (signal: 11, SIGSEGV: invalid memory reference)
