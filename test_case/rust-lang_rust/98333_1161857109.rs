rust
error: linking with `rust-lld` failed: exit status: 1
  |
  = note: "rust-lld" "-flavor" "gnu" "/tmp/rustcZzSyCr/symbols.o" […]
  = note: rust-lld: error: undefined symbol: __atomic_load_4
          >>> referenced by compiler_builtins.5931568b-cgu.83
          >>>               compiler_builtins-35b1260b00e1afde.compiler_builtins.5931568b-cgu.83.rcgu.o:(compiler_builtins::mem::memcpy::h4f55b8ec9004b8fa) in archive /home/simon/projects/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/riscv32i-unknown-none-elf/lib/libcompiler_builtins-35b1260b00e1afde.rlib
