
2019-08-07T09:31:26.2317594Z    Compiling libc v0.2.60
2019-08-07T09:31:26.2509078Z    Compiling compiler_builtins v0.1.18
2019-08-07T09:31:31.4941891Z    Compiling backtrace-sys v0.1.30
2019-08-07T09:31:33.3554962Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-08-07T09:31:33.3555404Z error: failed to run custom build command for `unwind v0.0.0 (/checkout/src/libunwind)`
2019-08-07T09:31:33.3555915Z Caused by:
2019-08-07T09:31:33.3555915Z Caused by:
2019-08-07T09:31:33.3556485Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/unwind-a04f9435c6eb75e3/build-script-build` (exit code: 101)
2019-08-07T09:31:33.3556711Z --- stdout
2019-08-07T09:31:33.3556938Z cargo:rerun-if-changed=build.rs
2019-08-07T09:31:33.3557262Z cargo:rustc-link-search=native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-3b3aa3b1105b96b5/out
2019-08-07T09:31:33.3558000Z running: "musl-g++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-static" "-march=i686" "-Wl,-melf_i386" "-I" "../llvm-project/libunwind/include" "-std=c99" "-std=c++11" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-3b3aa3b1105b96b5/out/../llvm-project/libunwind/src/Unwind-EHABI.o" "-c" "../llvm-project/libunwind/src/Unwind-EHABI.cpp"
2019-08-07T09:31:33.3558842Z --- stderr
2019-08-07T09:31:33.3559052Z thread 'main' panicked at '
2019-08-07T09:31:33.3559217Z 
2019-08-07T09:31:33.3559217Z 
2019-08-07T09:31:33.3559475Z Internal error occurred: Failed to find tool. Is `musl-g++` installed?
2019-08-07T09:31:33.3559806Z ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.35/src/lib.rs:2398:5
