
cargo build -Z build-std=core --target avr-unknown-gnu-atmega328 --release
    Updating crates.io index
   Compiling core v0.0.0 (C:\Users\Thomas\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core)
   Compiling compiler_builtins v0.1.36 (C:\Users\Thomas\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\vendor\compiler_builtins)
   Compiling ruduino v0.2.0
   Compiling rustc-std-workspace-core v1.99.0 (C:\Users\Thomas\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\rustc-std-workspace-core)
error: could not compile `core`

Caused by:
  process didn't exit successfully: `rustc --crate-name core --edition=2018 C:\Users\Thomas\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=9bf8a2365c66f54c -C extra-filename=-9bf8a2365c66f54c --out-dir C:\Users\Thomas\CLionProjects\blink\target\avr-unknown-gnu-atmega328\release\deps --target avr-unknown-gnu-atmega328 -Z force-unstable-if-unmarked -L dependency=C:\Users\Thomas\CLionProjects\blink\target\avr-unknown-gnu-atmega328\release\deps -L dependency=C:\Users\Thomas\CLionProjects\blink\target\release\deps --cap-lints allow` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
