
$ PATH=/c/Program\ Files/LLVM/bin/:$PATH cargo +nightly run --release
   Compiling memchr v2.3.3
   Compiling log v0.4.8
   Compiling regex-syntax v0.6.17
   Compiling aho-corasick v0.7.10
   Compiling bstr v0.2.12
   Compiling regex v1.3.7
   Compiling globset v0.4.5
   Compiling test_ice v0.1.0 (C:\projects\temp\test_ice)
    Finished release [optimized] target(s) in 16.54s
     Running `target\release\test_ice.exe`
error: process didn't exit successfully: `target\release\test_ice.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
Segmentation fault
