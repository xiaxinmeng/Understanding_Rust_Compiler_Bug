
error: could not compile `ra_ide_db`.

Caused by:
  process didn't exit successfully: `rustc --crate-name ra_ide_db --edition=2018 crates/ra_ide_db/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -Cembed-bitcode=no -C debuginfo=0 -C metadata=1b26a821a979b7f0 -C extra-filename=-1b26a821a979b7f0 --out-dir /home/helin/repos/rust-analyzer/target/release/deps -C incremental=/home/helin/repos/rust-analyzer/target/release/incremental -L dependency=/home/helin/repos/rust-analyzer/target/release/deps --extern either=/home/helin/repos/rust-analyzer/target/release/deps/libeither-aa67f500fa1b3836.rmeta --extern fst=/home/helin/repos/rust-analyzer/target/release/deps/libfst-a436241aaecda5dd.rmeta --extern log=/home/helin/repos/rust-analyzer/target/release/deps/liblog-d65b441cd99ca94f.rmeta --extern once_cell=/home/helin/repos/rust-analyzer/target/release/deps/libonce_cell-1f1d61fb9d888693.rmeta --extern ra_db=/home/helin/repos/rust-analyzer/target/release/deps/libra_db-a1580ba10ebdaadf.rmeta --extern hir=/home/helin/repos/rust-analyzer/target/release/deps/libra_hir-65194feebe637091.rmeta --extern ra_prof=/home/helin/repos/rust-analyzer/target/release/deps/libra_prof-121a6d18eb447714.rmeta --extern ra_syntax=/home/helin/repos/rust-analyzer/target/release/deps/libra_syntax-40f70cf46f5a35df.rmeta --extern ra_text_edit=/home/helin/repos/rust-analyzer/target/release/deps/libra_text_edit-72023d99c13fe359.rmeta --extern rayon=/home/helin/repos/rust-analyzer/target/release/deps/librayon-14c5f6912a720164.rmeta --extern rustc_hash=/home/helin/repos/rust-analyzer/target/release/deps/librustc_hash-0d698360b305d6d8.rmeta --extern superslice=/home/helin/repos/rust-analyzer/target/release/deps/libsuperslice-4f6adebb3fcb1bf0.rmeta --extern test_utils=/home/helin/repos/rust-analyzer/target/release/deps/libtest_utils-e498bb2e216f0313.rmeta` (signal: 11, SIGSEGV: invalid memory reference)
warning: build failed, waiting for other jobs to finish...
remark: <unknown>:0:0: loop not vectorized: loop control flow is not understood by vectorizer
remark: <unknown>:0:0: loop not vectorized: loop control flow is not understood by vectorizer
remark: <unknown>:0:0: loop not vectorized: loop control flow is not understood by vectorizer
error: build failed
➜  rust-analyzer git:(master) cargo build --release
   Compiling ra_ide_db v0.1.0 (/home/helin/repos/rust-analyzer/crates/ra_ide_db)
   Compiling ra_ssr v0.1.0 (/home/helin/repos/rust-analyzer/crates/ra_ssr)
   Compiling ra_assists v0.1.0 (/home/helin/repos/rust-analyzer/crates/ra_assists)
   Compiling ra_ide v0.1.0 (/home/helin/repos/rust-analyzer/crates/ra_ide)
   Compiling rust-analyzer v0.1.0 (/home/helin/repos/rust-analyzer/crates/rust-analyzer)
