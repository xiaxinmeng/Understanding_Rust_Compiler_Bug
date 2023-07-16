
error: could not compile `core`.

Caused by:
  process didn't exit successfully: `rustc --crate-name core --edition=2018 /home/seren/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type rlib --emit=dep-info,metadata,link -C opt-level=3 -C panic=abort -C metadata=b3b8d14870eb83b9 -C extra-filename=-b3b8d14870eb83b9 --out-dir /home/seren/rusty-dos/target/dos/release/deps --target /home/seren/rusty-dos/dos.json -Zforce-unstable-if-unmarked -L dependency=/home/seren/rusty-dos/target/dos/release/deps -L dependency=/home/seren/rusty-dos/target/release/deps -C opt-level=z -C relocation-model=static` (signal: 11, SIGSEGV: invalid memory reference)
warning: build failed, waiting for other jobs to finish...
error: build failed
