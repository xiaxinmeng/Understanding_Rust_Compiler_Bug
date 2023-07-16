console
  $ rustc +stage1 kernel.rs -o kernel.wasm --target wasm32-wasi --crate-type=cdylib  -C link-args="--shared-memory" -C target-feature='+atomics,+atomics'
  $ wasm2wat --enable-threads kernel.wasm | grep '(memory'
    (import "env" "memory" (memory (;0;) 17 16384 shared))
  