 shell
[master] > rustc -vV
rustc 1.7.0-nightly (110df043b 2015-12-13)
binary: rustc
commit-hash: 110df043bf585e94764e07700576200290709859
commit-date: 2015-12-13
host: i686-pc-windows-gnu
release: 1.7.0-nightly
[master] > rustc .\src\bin\day9asplode.rs
Assertion failed!

Program: C:\Users\drk\AppData\Local\.multirust\toolchains\nightly\bin\rustc.exe File: C:/bot/slave/nightly-dist-rustc-win-gnu-32/build/src/llvm/include/llvm/Support/Casting.h, Line 237

Expression: isa<X>(Val) && "cast<Ty>() argument of incompatible type!"

This application has requested the Runtime to terminate it in an unusual way.
Please contact the application's support team for more information.
