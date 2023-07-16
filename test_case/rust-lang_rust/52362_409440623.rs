
pi@raspberrypi:~/RustTuts/hello-world $ rustup component add llvm-tools-preview --toolchain=nightly
info: downloading component 'llvm-tools-preview'
332.7 KiB / 332.7 KiB (100 %) 135.7 KiB/s ETA:   0 s
info: installing component 'llvm-tools-preview'
pi@raspberrypi:~/RustTuts/hello-world $ cargo +nightly build --target wasm32-unknown-unknown --release --verbose
   Compiling hello-world v0.1.0 (file:///home/pi/RustTuts/hello-world)
     Running `rustc --crate-name hello_world src/lib.rs --crate-type cdylib --emit=dep-info,link -C opt-level=3 -C metadata=c2b67d4c4ad89cb4 --out-dir /home/pi/RustTuts/hello-world/target/wasm32-unknown-unknown/release/deps --target wasm32-unknown-unknown -L dependency=/home/pi/RustTuts/hello-world/target/wasm32-unknown-unknown/release/deps -L dependency=/home/pi/RustTuts/hello-world/target/release/deps`
error: linker `rust-lld` not found                                       ] 0/1: hello-world
  |
  = note: No such file or directory (os error 2)

error: aborting due to previous error

error: Could not compile `hello-world`.

Caused by:
  process didn't exit successfully: `rustc --crate-name hello_world src/lib.rs --crate-type cdylib --emit=dep-info,link -C opt-level=3 -C metadata=c2b67d4c4ad89cb4 --out-dir /home/pi/RustTuts/hello-world/target/wasm32-unknown-unknown/release/deps --target wasm32-unknown-unknown -L dependency=/home/pi/RustTuts/hello-world/target/wasm32-unknown-unknown/release/deps -L dependency=/home/pi/RustTuts/hello-world/target/release/deps` (exit code: 101)

