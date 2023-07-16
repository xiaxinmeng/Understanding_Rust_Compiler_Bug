
$ rustup update
...
nightly-x86_64-apple-darwin unchanged - rustc 1.47.0-nightly (db0492ace 2020-07-29)

$ cargo +nightly check
   Compiling version_check v0.9.2
    Checking siphasher v0.3.3
   Compiling log v0.4.8
    Checking cfg-if v0.1.10
   Compiling unicase v2.6.0
    Checking phf_shared v0.8.0
    Checking phf v0.8.0
    Checking typos-dict v0.2.1 (/Users/edpage/src/personal/typos/crates/typos-dict)

thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
error: could not compile `typos-dict`.

Caused by:
  process didn't exit successfully: `rustc --crate-name typos_dict --edition=2018 crates/typos-dict/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -Cembed-bitcode=no -C debuginfo=2 -C metadata=d5a6aa6c6eb18cbb -C extra-filename=-d5a6aa6c6eb18cbb --out-dir /Users/edpage/src/personal/typos/target/debug/deps -C incremental=/Users/edpage/src/personal/typos/target/debug/incremental -L dependency=/Users/edpage/src/personal/typos/target/debug/deps --extern log=/Users/edpage/src/personal/typos/target/debug/deps/liblog-194f264183e56651.rmeta --extern phf=/Users/edpage/src/personal/typos/target/debug/deps/libphf-49669432db2fabaf.rmeta --extern unicase=/Users/edpage/src/personal/typos/target/debug/deps/libunicase-0d7595d4b238a095.rmeta` (signal: 6, SIGABRT: process abort signal)
