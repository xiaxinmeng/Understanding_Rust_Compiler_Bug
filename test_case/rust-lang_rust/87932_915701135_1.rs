bash
❯ cargo build
   Compiling rustc_repro_87932 v0.1.0 (/home/adrien/Projects/RustAudio/rustc_repro_87932)

thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
error: could not compile `rustc_repro_87932`

Caused by:
  process didn't exit successfully: `rustc --crate-name rustc_repro_87932 --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=0af247fb9f5226ff -C extra-filename=-0af247fb9f5226ff --out-dir /home/adrien/Projects/RustAudio/rustc_repro_87932/target/debug/deps -C incremental=/home/adrien/Projects/RustAudio/rustc_repro_87932/target/debug/incremental -L dependency=/home/adrien/Projects/RustAudio/rustc_repro_87932/target/debug/deps --extern mylib=/home/adrien/Projects/RustAudio/rustc_repro_87932/target/debug/deps/libmylib-8a9782bb2f662999.rmeta --extern mylib_sys=/home/adrien/Projects/RustAudio/rustc_repro_87932/target/debug/deps/libmylib_sys-423bf155b0053619.rmeta` (signal: 6, SIGABRT: process abort signal)
