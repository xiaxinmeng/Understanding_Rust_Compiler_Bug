console
$ /.../rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc kernel.rs --target wasm32-wasi --crate-type=cdylib -o kernel.wasm -C link-args="--shared-memory" -C target-feature='+atomics,+atomics' --sysroot /.../rust/build/x86_64-unknown-linux-gnu/stage1

...
error: linker `rust-lld` not found
  |
  = note: No such file or directory (os error 2)

error: aborting due to previous error; 1 warning emitted
