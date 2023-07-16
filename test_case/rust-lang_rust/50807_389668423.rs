
Building stage1 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
error: process didn't exit successfully: `C:\projects\rust\build\bootstrap/debug/rustc -vV` (exit code: 101)
--- stdout
rustc 1.27.0-dev
binary: rustc
commit-hash: unknown
commit-date: unknown
host: x86_64-pc-windows-msvc
release: 1.27.0-dev
--- stderr
error: couldn't load codegen backend "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage1\\lib\\rustlib\\x86_64-pc-windows-msvc\\codegen-backends\\rustc_trans-llvm.dll": "The specified module could not be found. (os error 126)"
