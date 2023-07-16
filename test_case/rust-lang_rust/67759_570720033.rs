plain
2020-01-03T22:40:54.9106810Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-01-03T22:41:01.7062662Z error: failed to run custom build command for `profiler_builtins v0.0.0 (/checkout/src/libprofiler_builtins)`
2020-01-03T22:41:01.7062812Z 
2020-01-03T22:41:01.7062911Z Caused by:
2020-01-03T22:41:01.7063662Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/profiler_builtins-79d44dd7732723e3/build-script-build` (exit code: 1)
2020-01-03T22:41:01.7064278Z --- stdout
2020-01-03T22:41:01.7064543Z TARGET = Some("x86_64-unknown-linux-gnu")
2020-01-03T22:41:01.7064615Z OPT_LEVEL = Some("2")
2020-01-03T22:41:01.7064857Z HOST = Some("x86_64-unknown-linux-gnu")
2020-01-03T22:41:01.7065439Z CC_x86_64-unknown-linux-gnu = Some("sccache clang")
2020-01-03T22:41:01.7066383Z CFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/d5ca0b366ee2e90c658ca0334740b4907ac1ce69")
2020-01-03T22:41:01.7066528Z CRATE_CC_NO_DEFAULTS = None
2020-01-03T22:41:01.7066600Z DEBUG = Some("true")
2020-01-03T22:41:01.7067755Z running: "sccache" "sccache" "clang" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-unknown-linux-gnu" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "-fdebug-prefix-map=/checkout=/rustc/d5ca0b366ee2e90c658ca0334740b4907ac1ce69" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/profiler_builtins-3fff06880a6b0050/out/GCDAProfiling.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c"
2020-01-03T22:41:01.7068350Z cargo:warning=In file included from /checkout/src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:62:
2020-01-03T22:41:01.7068756Z cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/profile/InstrProfiling.h:16:10: fatal error: 'profile/InstrProfData.inc' file not found
2020-01-03T22:41:01.7068907Z cargo:warning=#include "profile/InstrProfData.inc"
2020-01-03T22:41:01.7069008Z cargo:warning=         ^~~~~~~~~~~~~~~~~~~~~~~~~~~
2020-01-03T22:41:01.7069083Z cargo:warning=1 error generated.
2020-01-03T22:41:01.7069361Z 
2020-01-03T22:41:01.7069562Z --- stderr
2020-01-03T22:41:01.7069596Z 
2020-01-03T22:41:01.7069627Z 
2020-01-03T22:41:01.7069627Z 
2020-01-03T22:41:01.7071484Z error occurred: Command "sccache" "sccache" "clang" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-unknown-linux-gnu" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-gnu" "-fdebug-prefix-map=/checkout=/rustc/d5ca0b366ee2e90c658ca0334740b4907ac1ce69" "-fno-builtin" "-fvisibility=hidden" "-fomit-frame-pointer" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_FCNTL_LCK=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/profiler_builtins-3fff06880a6b0050/out/GCDAProfiling.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c" with args "clang" did not execute successfully (status code exit code: 1).
2020-01-03T22:41:01.7072062Z 
2020-01-03T22:41:01.7072113Z 
2020-01-03T22:41:01.7072467Z warning: build failed, waiting for other jobs to finish...
2020-01-03T22:41:03.4272429Z error: build failed
---
2020-01-03T22:41:03.4396848Z   local time: Fri Jan  3 22:41:03 UTC 2020
2020-01-03T22:41:03.9739354Z   network time: Fri, 03 Jan 2020 22:41:03 GMT
2020-01-03T22:41:03.9742186Z == end clock drift check ==
2020-01-03T22:41:07.4513425Z 
2020-01-03T22:41:07.4632411Z ##[error]Bash exited with code '1'.
2020-01-03T22:41:07.4674659Z ##[section]Starting: Checkout
2020-01-03T22:41:07.4677067Z ==============================================================================
2020-01-03T22:41:07.4677161Z Task         : Get sources
2020-01-03T22:41:07.4677264Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
