
c:\Users\pfxbo\issue_74498>set RUSTFLAGS=
set RUSTFLAGS=

c:\Users\pfxbo\issue_74498>echo %RUSTFLAGS%
echo %RUSTFLAGS%
%RUSTFLAGS%

c:\Users\pfxbo\issue_74498>cargo run --release
cargo run --release
   Compiling issue_74498 v0.1.0 (C:\Users\pfxbo\issue_74498)
    Finished release [optimized] target(s) in 2.25s
     Running `target\release\issue_74498.exe`
memory allocation of 53687091200 bytes failederror: process didn't exit successfully: `target\release\issue_74498.exe` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)

c:\Users\pfxbo\issue_74498>set RUSTFLAGS=-Cllvm-args=-cvp-dont-add-nowrap-flags
set RUSTFLAGS=-Cllvm-args=-cvp-dont-add-nowrap-flags

c:\Users\pfxbo\issue_74498>echo %RUSTFLAGS%
echo %RUSTFLAGS%
-Cllvm-args=-cvp-dont-add-nowrap-flags

c:\Users\pfxbo\issue_74498>cargo run --release
cargo run --release
   Compiling issue_74498 v0.1.0 (C:\Users\pfxbo\issue_74498)
    Finished release [optimized] target(s) in 2.18s
     Running `target\release\issue_74498.exe`
1025 bytes written

c:\Users\pfxbo\issue_74498>cargo --version
cargo --version
cargo 1.44.1 (88ba85757 2020-06-11)

c:\Users\pfxbo\issue_74498>rustc --version
rustc --version
rustc 1.44.1 (c7087fe00 2020-06-17)

c:\Users\pfxbo\issue_74498>
