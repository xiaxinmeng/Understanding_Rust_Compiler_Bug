
thread '<main>' panicked at 'called `Result::unwrap()` on an `Err` value: couldn't create file (OS Error 3: 지정된 경로를 찾을 수 없습니다.
; path=Files\OpenVPN\bin;C:\Rust\bin;C:\msys64\usr\bin\site_perl;C:\msys64\usr\bin\vendor_perl;C:\msys64\usr\bin\core_perl \C\msys64\home\barosl\dev\rust\x86_64-pc-windows-gnu\stage2\bin\rustc.exe --out-dir \C\msys64\home\barosl\dev\rust\x86_64-pc-windows-gnu\test\run-make\cannot-read-embedded-idents -L \C\msys64\home\barosl\dev\rust\x86_64-pc-windows-gnu\test\run-make\cannot-read-embedded-idents\broken.rs; mode=truncate; access=write)', C:\msys64\home\barosl\dev\rust\src\libcore\result.rs:742
make[1]: *** [dotest] Error 101

------        ---------------------------------------------

/home/barosl/dev/rust/mk/tests.mk:1026: 'x86_64-pc-windows-gnu/test/run-make/cannot-read-embedded-idents-2-T-x86_64-pc-windows-gnu-H-x86_64-pc-windows-gnu.ok' 타겟에 대한 명령이 실패했습니다
make: *** [x86_64-pc-windows-gnu/test/run-make/cannot-read-embedded-idents-2-T-x86_64-pc-windows-gnu-H-x86_64-pc-windows-gnu.ok] 오류 2
