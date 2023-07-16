rust
➜  hello git:(master) ✗ RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release --target=x86_64-unknown-linux-gnu
   Compiling pin-project-lite v0.2.7
   Compiling libc v0.2.106
   Compiling num_cpus v1.13.0
   Compiling tokio v1.13.0
   Compiling hello v0.1.0 (/home/user/src/hello)
rustc: /home/user/src/rust/src/llvm-project/llvm/include/llvm/Support/Casting.h:269: typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::Function; Y = llvm::Value; typename llvm::cast_retty<X, Y*>::ret_type = llvm::Function*]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
error: could not compile `hello`

Caused by:
  process didn't exit successfully: `rustc --crate-name hello --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C lto -C metadata=f7513a844da49c9b -C extra-filename=-f7513a844da49c9b --out-dir /home/user/src/hello/target/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/user/src/hello/target/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/user/src/hello/target/release/deps --extern tokio=/home/user/src/hello/target/x86_64-unknown-linux-gnu/release/deps/libtokio-7721661bcaabc1c1.rlib -Cprofile-use=/tmp/pgo-data/merged.profdata` (signal: 6, SIGABRT: process abort signal)
