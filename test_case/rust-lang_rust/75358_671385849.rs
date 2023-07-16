console
rustc -C debuginfo=1 -o ./foo-stage1.s --emit asm -Cllvm-args=--x86-asm-syntax=intel --crate-type rlib --color=always -C opt-level=3 -Z symbol-mangling-version=v0 ./foo.rs
