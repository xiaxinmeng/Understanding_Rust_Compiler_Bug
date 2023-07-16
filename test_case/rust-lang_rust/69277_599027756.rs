plain
2020-03-14T08:24:34.1631363Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2020-03-14T08:24:34.1682653Z error: failed to run custom build command for `unwind v0.0.0 (/checkout/src/libunwind)`
2020-03-14T08:24:34.1686078Z 
2020-03-14T08:24:34.1686342Z Caused by:
2020-03-14T08:24:34.1692648Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/unwind-50f41811f813e479/build-script-build` (exit code: 1)
2020-03-14T08:24:34.1693330Z --- stdout
2020-03-14T08:24:34.1693956Z cargo:rerun-if-changed=build.rs
2020-03-14T08:24:34.1695373Z cargo:rustc-link-search=native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-c6f588d77e17ec07/out
2020-03-14T08:24:34.1697858Z running: "musl-g++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m32" "-static" "-march=i686" "-Wl,-melf_i386" "-I" "../llvm-project/libunwind/include" "-std=c99" "-std=c++11" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-D__LITTLE_ENDIAN__=1" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-c6f588d77e17ec07/out/../llvm-project/libunwind/src/Unwind-EHABI.o" "-c" "../llvm-project/libunwind/src/Unwind-EHABI.cpp"
2020-03-14T08:24:34.1699861Z --- stderr
2020-03-14T08:24:34.1700223Z 
2020-03-14T08:24:34.1700539Z 
2020-03-14T08:24:34.1700539Z 
2020-03-14T08:24:34.1701221Z error occurred: Failed to find tool. Is `musl-g++` installed?
2020-03-14T08:24:34.1702005Z 
2020-03-14T08:24:34.1710824Z 
2020-03-14T08:24:34.1712220Z warning: build failed, waiting for other jobs to finish...
2020-03-14T08:24:54.2262376Z [RUSTC-TIMING] core test:false 25.348
---
2020-03-14T08:24:54.2458364Z   local time: Sat Mar 14 08:24:54 UTC 2020
2020-03-14T08:24:54.5320093Z   network time: Sat, 14 Mar 2020 08:24:54 GMT
2020-03-14T08:24:54.5324254Z == end clock drift check ==
2020-03-14T08:24:55.6709357Z 
2020-03-14T08:24:55.6791579Z ##[error]Bash exited with code '1'.
2020-03-14T08:24:55.6869974Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-14T08:24:55.6878355Z ==============================================================================
2020-03-14T08:24:55.6878913Z Task         : Get sources
2020-03-14T08:24:55.6879391Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
