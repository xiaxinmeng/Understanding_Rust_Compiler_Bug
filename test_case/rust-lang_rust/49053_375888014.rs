
Building stage1 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
error: process didn't exit successfully: `C:\projects\rust\build\bootstrap/debug/rustc -vV` (exit code: 101)
--- stdout
rustc 1.26.0-nightly (23fd027a2 2018-03-24)
binary: rustc
commit-hash: 23fd027a23563fd676433a42955ea7239a8bc29c
commit-date: 2018-03-24
host: x86_64-pc-windows-msvc
release: 1.26.0-nightly
--- stderr
error: couldn't load codegen backend "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage1\\lib\\rustlib\\x86_64-pc-windows-msvc\\codegen-backends\\rustc_trans-llvm.dll": "The specified module could not be found. (os error 126)"
