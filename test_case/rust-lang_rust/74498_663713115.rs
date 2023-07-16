
> $Env:RUSTFLAGS="-C llvm-args=-cvp-dont-add-nowrap-flags-GARBAGE-GARBAGE"
> cargo run --release
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names -C llvm-args=-cvp-dont-add-nowrap-flags-GARBAGE-GARBAGE --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit code: 1)
--- stderr
rustc: Unknown command line argument '-cvp-dont-add-nowrap-flags-GARBAGE-GARBAGE'.  Try: 'rustc --help'
rustc: Did you mean '--cvp-dont-add-nowrap-flags'?

> $Env:RUSTFLAGS="-C llvm-args=-cvp-dont-add-nowrap-flags"
> cargo run --release
    Finished release [optimized] target(s) in 0.07s
     Running `target\release\bmp-minimal.exe`
memory allocation of 13421772800 bytes failederror: process didn't exit successfully: `target\release\bmp-minimal.exe` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
