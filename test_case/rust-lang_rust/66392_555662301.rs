plain
2019-11-19T19:06:30.8550193Z [RUSTC-TIMING] getrandom test:false 0.444
2019-11-19T19:06:31.2240470Z error: failed to run custom build command for `libnghttp2-sys v0.1.1`
2019-11-19T19:06:31.2244061Z 
2019-11-19T19:06:31.2244697Z Caused by:
2019-11-19T19:06:31.2245293Z   process didn't exit successfully: `D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\release\build\libnghttp2-sys-e4c66f64246c6564\build-script-build` (exit code: 1)
2019-11-19T19:06:31.2245868Z --- stdout
2019-11-19T19:06:31.2246397Z TARGET = Some("x86_64-pc-windows-msvc")
2019-11-19T19:06:31.2246879Z OPT_LEVEL = Some("2")
2019-11-19T19:06:31.2247374Z HOST = Some("x86_64-pc-windows-msvc")
2019-11-19T19:06:31.2248152Z CC_x86_64-pc-windows-msvc = None
2019-11-19T19:06:31.2248670Z CC_x86_64_pc_windows_msvc = None
2019-11-19T19:06:31.2249157Z HOST_CC = None
2019-11-19T19:06:31.2249576Z CC = Some("D:/a/1/s/citools/clang-rust/bin/clang-cl.exe")
2019-11-19T19:06:31.2250069Z CFLAGS_x86_64-pc-windows-msvc = None
2019-11-19T19:06:31.2250480Z CFLAGS_x86_64_pc_windows_msvc = None
2019-11-19T19:06:31.2250914Z HOST_CFLAGS = None
2019-11-19T19:06:31.2251310Z CFLAGS = None
2019-11-19T19:06:31.2251714Z CRATE_CC_NO_DEFAULTS = None
2019-11-19T19:06:31.2252134Z CARGO_CFG_TARGET_FEATURE = Some("crt-static,fxsr,mmx,sse,sse2")
2019-11-19T19:06:31.2252695Z DEBUG = Some("false")
2019-11-19T19:06:31.2253363Z running: "D:/a/1/s/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Brepro" "-m64" "-I" "nghttp2/lib/includes" "-I" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0-bootstrap-tools\\x86_64-pc-windows-msvc\\release\\build\\libnghttp2-sys-cc04ace718ad2a7f\\out\\i\\include" "-DNGHTTP2_STATICLIB" "-DHAVE_NETINET_IN" "-Dssize_t=int64_t" "-FoD:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0-bootstrap-tools\\x86_64-pc-windows-msvc\\release\\build\\libnghttp2-sys-cc04ace718ad2a7f\\out\\i\\lib\\nghttp2/lib/nghttp2_buf.o" "-c" "nghttp2/lib/nghttp2_buf.c"
2019-11-19T19:06:31.2253931Z cargo:warning=Stack dump:
2019-11-19T19:06:31.2255343Z cargo:warning=0. Program arguments: D:\a\1\s\citools\clang-rust\bin\clang-cl.exe -cc1 -triple x86_64-pc-windows-msvc19.16.27034 -emit-obj -disable-free -disable-llvm-verifier -discard-value-names -main-file-name nghttp2_buf.c -mrelocation-model pic -pic-level 2 -mthread-model posix -relaxed-aliasing -fmath-errno -masm-verbose -mconstructor-aliases -munwind-tables -target-cpu x86-64 -mllvm -x86-asm-syntax=intel -D_MT -flto-visibility-public-std --dependent-lib=libcmt --dependent-lib=oldnames -stack-protector 2 -fms-volatile -fdiagnostics-format msvc -dwarf-column-info -momit-leaf-frame-pointer -ffunction-sections -coverage-notes-file C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\libnghttp2-sys-0.1.1\nghttp2_buf.gcno -resource-dir D:\a\1\s\citools\clang-rust\lib\clang\9.0.0 -I nghttp2/lib/includes -I D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\build\libnghttp2-sys-cc04ace718ad2a7f\out\i\include -D NGHTTP2_STATICLIB -D HAVE_NETINET_IN -D ssize_t=int64_t -internal-isystem D:\a\1\s\citools\clang-rust\lib\clang\9.0.0\include -internal-isystem C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\include -internal-isystem C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\ucrt -internal-isystem C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\um -internal-isystem C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\cppwinrt -internal-isystem C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\winrt -internal-isystem C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\shared -O2 -fdebug-compilation-dir C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\libnghttp2-sys-0.1.1 -ferror-limit 19 -fmessage-length 0 -fno-use-cxa-atexit -fms-extensions -fms-compatibility -fms-compatibility-version=19.16.27034 -fdelayed-template-parsing -fobjc-runtime=gcc -fdiagnostics-show-option -vectorize-loops -vectorize-slp -faddrsig -o D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\build\libnghttp2-sys-cc04ace718ad2a7f\out\i\lib\nghttp2/lib/nghttp2_buf.o -x c nghttp2/lib/nghttp2_buf.c 
2019-11-19T19:06:31.2256850Z cargo:warning=1. <eof> parser at end of file
2019-11-19T19:06:31.2257412Z cargo:warning=#0 0x00007ff639ba2d84 (D:\a\1\s\citools\clang-rust\bin\clang-cl.exe+0x2f22d84)
2019-11-19T19:06:31.2258208Z cargo:warning=#1 0x0000021924000024 
2019-11-19T19:06:31.2258631Z cargo:warning=#2 0x00000219234b0000 
2019-11-19T19:06:31.2259166Z cargo:warning=clang-cl: error: clang frontend command failed due to signal (use -v to see invocation)
2019-11-19T19:06:31.2259693Z cargo:warning=clang version 9.0.0 (tags/RELEASE_900/final)
2019-11-19T19:06:31.2260164Z cargo:warning=Target: x86_64-pc-windows-msvc
2019-11-19T19:06:31.2260615Z cargo:warning=Thread model: posix
2019-11-19T19:06:31.2261036Z cargo:warning=InstalledDir: D:/a/1/s/citools/clang-rust/bin
2019-11-19T19:06:31.2261636Z cargo:warning=clang-cl: note: diagnostic msg: PLEASE submit a bug report to https://bugs.llvm.org/ and include the crash backtrace, preprocessed source, and associated run script.
2019-11-19T19:06:31.2262186Z cargo:warning=clang-cl: note: diagnostic msg: Error generating preprocessed source(s).
2019-11-19T19:06:31.2263122Z 
2019-11-19T19:06:31.2263635Z --- stderr
2019-11-19T19:06:31.2263985Z 
2019-11-19T19:06:31.2264329Z 
2019-11-19T19:06:31.2264329Z 
2019-11-19T19:06:31.2264961Z error occurred: Command "D:/a/1/s/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Brepro" "-m64" "-I" "nghttp2/lib/includes" "-I" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0-bootstrap-tools\\x86_64-pc-windows-msvc\\release\\build\\libnghttp2-sys-cc04ace718ad2a7f\\out\\i\\include" "-DNGHTTP2_STATICLIB" "-DHAVE_NETINET_IN" "-Dssize_t=int64_t" "-FoD:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0-bootstrap-tools\\x86_64-pc-windows-msvc\\release\\build\\libnghttp2-sys-cc04ace718ad2a7f\\out\\i\\lib\\nghttp2/lib/nghttp2_buf.o" "-c" "nghttp2/lib/nghttp2_buf.c" with args "clang-cl.exe" did not execute successfully (status code exit code: 0xc000001d).
2019-11-19T19:06:31.2266310Z 
2019-11-19T19:06:31.2266661Z 
2019-11-19T19:06:31.2267116Z warning: build failed, waiting for other jobs to finish...
2019-11-19T19:06:31.3759143Z [RUSTC-TIMING] winapi test:false 18.052
---
2019-11-19T19:06:31.4334374Z   local time: Tue Nov 19 19:06:31 CUT 2019
2019-11-19T19:06:31.6920209Z   network time: Tue, 19 Nov 2019 19:06:31 GMT
2019-11-19T19:06:31.6963605Z == end clock drift check ==
2019-11-19T19:06:31.6993384Z 
2019-11-19T19:06:31.7693944Z ##[error]Bash exited with code '1'.
2019-11-19T19:06:31.7791155Z ##[section]Starting: Checkout
2019-11-19T19:06:31.7896100Z ==============================================================================
2019-11-19T19:06:31.7896221Z Task         : Get sources
2019-11-19T19:06:31.7896314Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
