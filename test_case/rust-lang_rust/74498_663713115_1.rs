
>set RUSTFLAGS=-Cllvm-args=-cvp-dont-add-nowrap-flags
>echo %RUSTFLAGS%
-Cllvm-args=-cvp-dont-add-nowrap-flags
>cargo run --release
   Compiling bmp-minimal v0.1.0 (C:\X\bmp-minimal)
    Finished release [optimized] target(s) in 4.46s
     Running `target\release\bmp-minimal.exe`
memory allocation of 13421772800 bytes failederror: process didn't exit successfully: `target\release\bmp-minimal.exe` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
