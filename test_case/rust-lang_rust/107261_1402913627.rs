
$ rustup toolchain install nightly-2023-01-24
$ rustup component add rust-src --toolchain nightly-2023-01-24-x86_64-unknown-linux-gnu
$ rustup override set nightly-2023-01-24
$ cargo build --release
...
error: could not compile `worduino-avr`; 2 warnings emitted

Caused by:
  process didn't exit successfully: `rustc --crate-name worduino_avr src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=150 --crate-type bin --emit=dep-info,link -C opt-level=3 -C lto -C metadata=65b5fd88837c4926 -C extra-filename=-65b5fd88837c4926 --out-dir /home/cactus/prog/rust/avr/bugs/segfault/target/avr-atmega32u4/release/deps --target /home/cactus/prog/rust/avr/bugs/segfault/avr-atmega32u4.json -L dependency=/home/cactus/prog/rust/avr/bugs/segfault/target/avr-atmega32u4/release/deps -L dependency=/home/cactus/prog/rust/avr/bugs/segfault/target/release/deps --extern avr_config=/home/cactus/prog/rust/avr/bugs/segfault/target/avr-atmega32u4/release/deps/libavr_config-9dd7198fde59ab9f.rlib --extern avr_std_stub=/home/cactus/prog/rust/avr/bugs/segfault/target/avr-atmega32u4/release/deps/libavr_std_stub-a6f7257e58f8cef6.rlib --extern 'noprelude:compiler_builtins=/home/cactus/prog/rust/avr/bugs/segfault/target/avr-atmega32u4/release/deps/libcompiler_builtins-04596a6bfc480d2b.rlib' --extern 'noprelude:core=/home/cactus/prog/rust/avr/bugs/segfault/target/avr-atmega32u4/release/deps/libcore-eac7e4ed729ade79.rlib' --extern worduino_engine=/home/cactus/prog/rust/avr/bugs/segfault/target/avr-atmega32u4/release/deps/libworduino_engine-6d2d2f69f1e30efe.rlib -Z unstable-options --emit=llvm-bc` (signal: 11, SIGSEGV: invalid memory reference)
