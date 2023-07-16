
./build/x86_64-unknown-linux-gnu/stage1/bin/rustc simple.rs -C opt-level=1 -Clto=false -C codegen-units=1 -C llvm-args=-opt-bisect-limit=10
