
(gdb) run
Starting program: /Users/edpage/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name typos_dict --edition=2018 crates/typos-dict/src/lib.rs --error-fo
rmat=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -Cembed-bitcode=no -C debuginfo=2 -C metadata=d5a6aa6c6eb18cbb -C extra-filename=-d
5a6aa6c6eb18cbb --out-dir /Users/edpage/src/personal/typos/target/debug/deps -C incremental=/Users/edpage/src/personal/typos/target/debug/incremental -L dependency=/Us
ers/edpage/src/personal/typos/target/debug/deps --extern log=/Users/edpage/src/personal/typos/target/debug/deps/liblog-194f264183e56651.rmeta --extern phf=/Users/edpag
e/src/personal/typos/target/debug/deps/libphf-49669432db2fabaf.rmeta --extern unicase=/Users/edpage/src/personal/typos/target/debug/deps/libunicase-0d7595d4b238a095.rm
eta
Unable to find Mach task port for process-id 76733: (os/kern) failure (0x5).
 (please check gdb is codesigned - see taskgated(8)
