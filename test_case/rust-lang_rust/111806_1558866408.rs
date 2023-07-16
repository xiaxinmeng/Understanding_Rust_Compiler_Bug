plain
Executing benchmark clap-3.1.6 (2/8)
Preparing clap-3.1.6
[2023-05-23T08:24:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T08:24:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T08:24:10Z DEBUG collector::execute] cd "/tmp/.tmpvHqlUS" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvHqlUS#clap@3.1.6" "--" "--skip-this-rustc"
[2023-05-23T08:24:10Z DEBUG collector::execute] cd "/tmp/.tmpeZSecn" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeZSecn#clap@3.1.6" "--release" "--" "--skip-this-rustc"
[2023-05-23T08:24:13Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T08:24:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T08:24:13Z DEBUG collector::execute] cd "/tmp/.tmpUEx06I" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUEx06I#clap@3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
Running clap-3.1.6: Opt + [Full]
[2023-05-23T08:24:17Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T08:24:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T08:24:17Z DEBUG collector::execute] cd "/tmp/.tmpuVSgWq" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuVSgWq#clap@3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark hyper-0.14.18 (3/8)
Preparing hyper-0.14.18
[2023-05-23T08:24:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T08:24:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-05-23T08:24:55Z DEBUG collector::execute] cd "/tmp/.tmpR8omDU" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpR8omDU#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
[2023-05-23T08:24:59Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T08:24:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T08:24:59Z DEBUG collector::execute] cd "/tmp/.tmpN4UqcX" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpN4UqcX#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark regex-1.5.5 (4/8)
Preparing regex-1.5.5
[2023-05-23T08:25:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T08:25:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T08:25:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T08:25:08Z DEBUG collector::execute] cd "/tmp/.tmpuQGS3y" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuQGS3y#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-05-23T08:25:08Z DEBUG collector::execute] cd "/tmp/.tmpIo9aeu" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIo9aeu#regex@1.5.5" "--" "--skip-this-rustc"
Running regex-1.5.5: Debug + [Full]
[2023-05-23T08:25:17Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T08:25:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T08:25:17Z DEBUG collector::execute] cd "/tmp/.tmpjDLRYu" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjDLRYu#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-23T08:25:20Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T08:25:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T08:25:20Z DEBUG collector::execute] cd "/tmp/.tmp5WVPwm" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5WVPwm#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark regex-1.5.5 (4/8)
---
[2023-05-23T08:26:04Z DEBUG collector::execute] cd "/tmp/.tmpeoOPK2" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeoOPK2#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
[2023-05-23T08:26:09Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T08:26:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T08:26:09Z DEBUG collector::execute] cd "/tmp/.tmpWxEVlH" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWxEVlH#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0-tiny (6/8)
Preparing ripgrep-13.0.0-tiny
[2023-05-23T08:26:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T08:26:26Z DEBUG collector::execute] cd "/tmp/.tmpWcaY9G" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWcaY9G#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-05-23T08:26:26Z DEBUG collector::execute] cd "/tmp/.tmpWcaY9G" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWcaY9G#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
Running ripgrep-13.0.0-tiny: Opt + [Full]
[2023-05-23T08:26:40Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T08:26:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T08:26:40Z DEBUG collector::execute] cd "/tmp/.tmpgUEYuh" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgUEYuh#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark serde-1.0.136 (7/8)
Preparing serde-1.0.136
[2023-05-23T08:27:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T08:27:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T08:27:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T08:27:25Z DEBUG collector::execute] cd "/tmp/.tmpgoms4B" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgoms4B#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-05-23T08:27:25Z DEBUG collector::execute] cd "/tmp/.tmptTqLum" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptTqLum#serde@1.0.136" "--" "--skip-this-rustc"
[2023-05-23T08:27:26Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T08:27:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T08:27:26Z DEBUG collector::execute] cd "/tmp/.tmppxy0qP" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppxy0qP#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
Running serde-1.0.136: Opt + [Full]
---
[2023-05-23T08:27:37Z DEBUG collector::execute] cd "/tmp/.tmprEOWAv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprEOWAv#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
[2023-05-23T08:27:39Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T08:27:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T08:27:39Z DEBUG collector::execute] cd "/tmp/.tmpwuQyBd" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwuQyBd#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging LLVM PGO profiles to /tmp/tmp-multistage/opt-artifacts/llvm-pgo-intermediate.profdata
stage-build INFO: Executing `/rustroot/bin/llvm-profdata merge -o /tmp/tmp-multistage/opt-artifacts/llvm-pgo-intermediate.profdata /tmp/tmp-multistage/opt-artifacts/llvm-pgo`
stage-build INFO: LLVM PGO statistics
---
stage-build INFO: Section `Stage 1 (LLVM PGO)` ended: OK (1939.36s)
stage-build INFO: Clearing LLVM build files
stage-build INFO: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm`
stage-build INFO: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/lld`
stage-build INFO: Section `Stage 1b (LLVM CS PGO)` starts
stage-build INFO: Section `Stage 1b (LLVM CS PGO) > Build rustc and LLVM` starts
stage-build INFO: Executing `LLVM_USE_CS_PGO=1 LLVM_PROFILE_DIR=/tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw /usr/bin/python3 /checkout/x.py build --target x86_64-unknown-linux-gnu --host x86_64-unknown-linux-gnu --stage 2 library/std --llvm-profile-generate --llvm-profile-use /tmp/tmp-multistage/opt-artifacts/llvm-pgo-intermediate.profdata`
    Finished dev [unoptimized] target(s) in 0.06s
##[endgroup]
[TIMING] compile::Assemble { target_compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu } } -- 0.000
[TIMING] compile::StartupObjects { compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
---
CMAKE_x86_64-unknown-linux-gnu = None
CMAKE_x86_64_unknown_linux_gnu = None
HOST_CMAKE = None
CMAKE = None
running: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" && CMAKE_PREFIX_PATH="" DESTDIR="" "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_UNREACHABLE_OPTIMIZE=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;LoongArch;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_WARNINGS=OFF" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_BUILD_INSTRUMENTED=CSIR" "-DLLVM_CSPROFILE_DATA_DIR=/tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw" "-DLLVM_BUILD_RUNTIME=No" "-DLLVM_ENABLE_ZSTD=OFF" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.71.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=clang" "-DCMAKE_CXX_COMPILER=clang++" "-DCMAKE_ASM_COMPILER=clang" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_RANLIB=/rustroot/bin/llvm-ranlib" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_BUILD_TYPE=Release"
  Your CMake version is 3.17.5.  Starting with LLVM 17.0.0, the minimum
  version of CMake required to build LLVM will become 3.20.0, and using an
  older CMake will become an error.  Please upgrade your CMake to at least
  3.20.0 now to avoid issues in the future!
---
[ 10%] Building Options.inc...
[ 10%] Building Options.inc...
[ 10%] Building Opts.inc...
[ 10%] Building OMP.h.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building Opts.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building Attributes.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building ACC.inc...
[ 10%] Built target JITLinkTableGen
[ 10%] Built target LipoOptsTableGen
[ 10%] Built target CxxfiltOptsTableGen
[ 10%] Built target CxxfiltOptsTableGen
[ 10%] Built target DllOptionsTableGen
[ 10%] Built target LibOptionsTableGen
[ 10%] Built target MLTableGen
[ 10%] Built target MtTableGen
[ 10%] Built target NmOptsTableGen
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Built target DwarfutilTableGen
[ 10%] Built target DsymutilTableGen
[ 10%] Building IntrinsicImpl.inc...
[ 10%] Building OMP.inc...
[ 10%] Building OMP.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building IntrinsicEnums.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building IntrinsicsAMDGPU.h...
[ 10%] Building IntrinsicsAArch64.h...
[ 10%] Building IntrinsicsAArch64.h...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target ObjdumpOptsTableGen
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target RcOptsTableGen
Scanning dependencies of target StringsOptsTableGen
Scanning dependencies of target ReadobjOptsTableGen
[ 10%] Building IntrinsicsARM.h...
[ 10%] Building IntrinsicsARM.h...
Scanning dependencies of target SymbolizerOptsTableGen
[ 10%] Built target acc_gen
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building Opts.inc...
[ 10%] Building ObjdumpOpts.inc...
[ 10%] Building ObjcopyOpts.inc...
[ 10%] Building Opts.inc...
[ 10%] Building Opts.inc...
[ 10%] Building Opts.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building Opts.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building Opts.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building Opts.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target LLVMFrontendOpenACC
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Built target SizeOptsTableGen
[ 10%] Built target ObjdumpOptsTableGen
[ 10%] Built target ReadobjOptsTableGen
[ 10%] Built target RcOptsTableGen
[ 10%] Built target RcOptsTableGen
[ 10%] Building IntrinsicsBPF.h...
[ 10%] Building CXX object lib/Frontend/OpenACC/CMakeFiles/LLVMFrontendOpenACC.dir/ACC.cpp.o
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Built target SymbolizerOptsTableGen
[ 10%] Building IntrinsicsDirectX.h...
[ 10%] Built target TLICheckerOptsTableGen
[ 10%] Built target StringsOptsTableGen
[ 10%] Built target StringsOptsTableGen
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target OtoolOptsTableGen
Scanning dependencies of target OtoolOptsTableGen
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building IntrinsicsLoongArch.h...
[ 10%] Building IntrinsicsMips.h...
Scanning dependencies of target InstallNameToolOptsTableGen
Scanning dependencies of target InstallNameToolOptsTableGen
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building IntrinsicsNVPTX.h...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building OtoolOpts.inc...
[ 10%] Building OtoolOpts.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Built target WindresOptsTableGen
[ 10%] Building InstallNameToolOpts.inc...
[ 10%] Building InstallNameToolOpts.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building IntrinsicsR600.h...
[ 10%] Building IntrinsicsR600.h...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target BitcodeStripOptsTableGen
[ 10%] Building BitcodeStripOpts.inc...
[ 10%] Building BitcodeStripOpts.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target StripOptsTableGen
[ 10%] Building StripOpts.inc...
[ 10%] Building StripOpts.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building IntrinsicsRISCV.h...
[ 10%] Building IntrinsicsRISCV.h...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building IntrinsicsS390.h...
[ 10%] Building IntrinsicsS390.h...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building IntrinsicsX86.h...
[ 10%] Building IntrinsicsX86.h...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building IntrinsicsVE.h...
[ 10%] Building IntrinsicsVE.h...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target BPFCommonTableGen
Scanning dependencies of target NVPTXCommonTableGen
Scanning dependencies of target LoongArchCommonTableGen
Scanning dependencies of target HexagonCommonTableGen
---
[ 10%] Building NVPTXGenAsmWriter.inc...
[ 10%] Building LoongArchGenAsmMatcher.inc...
Scanning dependencies of target LLVMOrcShared
[ 10%] Building SparcGenAsmMatcher.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building MSP430GenAsmMatcher.inc...
[ 10%] Building MSP430GenAsmMatcher.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building ARMGenAsmMatcher.inc...
[ 10%] Building HexagonGenAsmMatcher.inc...
[ 10%] Building HexagonGenAsmMatcher.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building RISCVGenAsmMatcher.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building MipsGenAsmMatcher.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building PPCGenAsmMatcher.inc...
[ 10%] Building PPCGenAsmMatcher.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building CXX object lib/ExecutionEngine/Orc/Shared/CMakeFiles/LLVMOrcShared.dir/AllocationActions.cpp.o
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target LLVMRemarks
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Linking CXX static library ../../libLLVMFrontendOpenACC.a
[ 10%] Built target LLVMFrontendOpenACC
Scanning dependencies of target WebAssemblyCommonTableGen
[ 10%] Building WebAssemblyGenAsmMatcher.inc...
[ 10%] Building WebAssemblyGenAsmMatcher.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building BPFGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building MSP430GenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building LoongArchGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building SparcGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building WebAssemblyGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building BPFGenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building MSP430GenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building PPCGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building MipsGenAsmWriter.inc...
[ 10%] Building MipsGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building SystemZGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 10%] Building SparcGenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building HexagonGenAsmWriter.inc...
[ 11%] Building HexagonGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building BPFGenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building MSP430GenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building ARMGenAsmWriter.inc...
[ 11%] Building ARMGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building SparcGenDAGISel.inc...
[ 11%] Building SparcGenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building LoongArchGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building NVPTXGenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building PPCGenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building BPFGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building MSP430GenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building MipsGenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 11%] Building SystemZGenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 12%] Building WebAssemblyGenDisassemblerTables.inc...
[ 12%] Building WebAssemblyGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building SparcGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building LoongArchGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building BPFGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building MSP430GenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building HexagonGenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building PPCGenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building SparcGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building WebAssemblyGenFastISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building SystemZGenDAGISel.inc...
[ 13%] Building SystemZGenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building AArch64GenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building ARMGenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building BPFGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building LoongArchGenMCPseudoLowering.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building MSP430GenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building HexagonGenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building BPFGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building LoongArchGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building SparcGenMCCodeEmitter.inc...
[ 13%] Building SparcGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building WebAssemblyGenInstrInfo.inc...
[ 13%] Building WebAssemblyGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building X86GenAsmMatcher.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building MSP430GenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building BPFGenSubtargetInfo.inc...
[ 13%] Building BPFGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building ARMGenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building LoongArchGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 13%] Building SparcGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building MSP430GenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building SystemZGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building MipsGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target AVRCommonTableGen
[ 14%] Building AVRGenAsmMatcher.inc...
[ 14%] Building AVRGenAsmMatcher.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building LoongArchGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building SparcGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target M68kCommonTableGen
[ 14%] Building M68kGenGlobalISel.inc...
[ 14%] Building M68kGenGlobalISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building WebAssemblyGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building NVPTXGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building AVRGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target LLVMTargetParser
[ 14%] Built target SparcCommonTableGen
[ 14%] Building CXX object lib/TargetParser/CMakeFiles/LLVMTargetParser.dir/AArch64TargetParser.cpp.o
[ 14%] Building HexagonGenDFAPacketizer.inc...
[ 14%] Building HexagonGenDFAPacketizer.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building RISCVGenCompressInstEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building AArch64GenAsmWriter1.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 14%] Building SystemZGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 15%] Building PPCGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 15%] Building WebAssemblyGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 15%] Building MipsGenFastISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 15%] Building AVRGenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 15%] Building M68kGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building WebAssemblyGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AVRGenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building M68kGenRegisterBank.inc...
[ 16%] Building M68kGenRegisterBank.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building HexagonGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building PPCGenFastISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building X86GenAsmWriter.inc...
[ 16%] Building X86GenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AVRGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building M68kGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building HexagonGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/Remark.cpp.o
[ 16%] Building ARMGenDisassemblerTables.inc...
[ 16%] Building ARMGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AVRGenInstrInfo.inc...
[ 16%] Building AVRGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building HexagonGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building M68kGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AArch64GenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AVRGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building RISCVGenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building M68kGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AVRGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/RemarkFormat.cpp.o
[ 16%] Building ARMGenFastISel.inc...
[ 16%] Building ARMGenFastISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building PPCGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building HexagonGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building X86GenAsmWriter1.inc...
[ 16%] Building X86GenAsmWriter1.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building MipsGenPostLegalizeGICombiner.inc...
[ 16%] Building MipsGenPostLegalizeGICombiner.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AVRGenSubtargetInfo.inc...
[ 16%] Building AVRGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building SystemZGenMCCodeEmitter.inc...
[ 16%] Building SystemZGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building NVPTXGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building M68kGenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AArch64GenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building RISCVGenDisassemblerTables.inc...
[ 16%] Building RISCVGenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building HexagonGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building MipsGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building SystemZGenRegisterInfo.inc...
[ 16%] Building SystemZGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building M68kGenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building ARMGenGlobalISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building CXX object lib/TargetParser/CMakeFiles/LLVMTargetParser.dir/CSKYTargetParser.cpp.o
[ 16%] Building M68kGenAsmWriter.inc...
[ 16%] Building M68kGenAsmWriter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building SystemZGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building X86GenCallingConv.inc...
[ 16%] Building X86GenCallingConv.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AArch64GenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building RISCVGenGlobalISel.inc...
[ 16%] Building RISCVGenGlobalISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building M68kGenAsmMatcher.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building X86GenDAGISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building PPCGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building M68kGenDisassemblerTable.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building MipsGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Built target M68kCommonTableGen
[ 16%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/RemarkSerializer.cpp.o
[ 16%] Building PPCGenRegisterInfo.inc...
[ 16%] Building PPCGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building ARMGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AArch64GenFastISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building ARMGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building MipsGenMCPseudoLowering.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building X86GenDisassemblerTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building PPCGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/RemarkStringTable.cpp.o
[ 16%] Building X86GenEVEX2VEXTables.inc...
[ 16%] Building X86GenEVEX2VEXTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building MipsGenRegisterBank.inc...
[ 16%] Building MipsGenRegisterBank.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building MipsGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building ARMGenMCPseudoLowering.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building PPCGenExegesis.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building MipsGenSubtargetInfo.inc...
[ 16%] Building MipsGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building MipsGenExegesis.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building PPCGenRegisterBank.inc...
[ 16%] Building PPCGenRegisterBank.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building ARMGenRegisterBank.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building AArch64GenGlobalISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building RISCVGenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building X86GenExegesis.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Building PPCGenGlobalISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 16%] Built target MipsCommonTableGen
[ 17%] Building ARMGenRegisterInfo.inc...
[ 17%] Building ARMGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenO0PreLegalizeGICombiner.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building ARMGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building ARMGenSystemRegister.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenPreLegalizeGICombiner.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building RISCVGenMCCodeEmitter.inc...
[ 17%] Building RISCVGenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenPostLegalizeGICombiner.inc...
[ 17%] Building AArch64GenPostLegalizeGICombiner.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building X86GenFastISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building RISCVGenMCPseudoLowering.inc...
[ 17%] Building RISCVGenMCPseudoLowering.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building X86GenGlobalISel.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenPostLegalizeGILowering.inc...
[ 17%] Building AArch64GenPostLegalizeGILowering.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building CXX object lib/TargetParser/CMakeFiles/LLVMTargetParser.dir/X86TargetParser.cpp.o
Scanning dependencies of target LLVMARMUtils
[ 17%] Building CXX object lib/Target/ARM/Utils/CMakeFiles/LLVMARMUtils.dir/ARMBaseInfo.cpp.o
[ 17%] Building AArch64GenMCCodeEmitter.inc...
[ 17%] Building AArch64GenMCCodeEmitter.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenMCPseudoLowering.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenRegisterBank.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Built target LLVMRemarks
[ 17%] Building X86GenInstrInfo.inc...
[ 17%] Building X86GenInstrInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building RISCVGenRegisterBank.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building RISCVGenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Linking CXX static library ../libLLVMTargetParser.a
[ 17%] Built target LLVMARMUtils
[ 17%] Building X86GenMnemonicTables.inc...
[ 17%] Building X86GenMnemonicTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
Scanning dependencies of target LLVMBinaryFormat
[ 17%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/AMDGPUMetadataVerifier.cpp.o
[ 17%] Building RISCVGenSearchableTables.inc...
[ 17%] Building RISCVGenSearchableTables.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenSubtargetInfo.inc...
[ 17%] Building AArch64GenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building RISCVGenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building X86GenRegisterBank.inc...
[ 17%] Building X86GenRegisterBank.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenSystemOperands.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building CXX object lib/FuzzMutate/CMakeFiles/LLVMFuzzerCLI.dir/FuzzerCLI.cpp.o
[ 17%] Building X86GenRegisterInfo.inc...
[ 17%] Building X86GenRegisterInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building X86GenSubtargetInfo.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Building AArch64GenExegesis.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[ 17%] Built target LLVMFuzzerCLI
Scanning dependencies of target LLVMOrcTargetProcess
[ 17%] Building CXX object lib/ExecutionEngine/Orc/TargetProcess/CMakeFiles/LLVMOrcTargetProcess.dir/ExecutorSharedMemoryMapperService.cpp.o
[ 17%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/DXContainer.cpp.o
---
[  2%] Building Options.inc...
[  3%] Building Options.inc...
[  3%] Building Options.inc...
[  4%] Building Options.inc...
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[  5%] Built target WasmOptionsTableGen
[  5%] Built target MinGWOptionsTableGen
[  5%] Built target COFFOptionsTableGen
[  5%] Built target ELFOptionsTableGen
---
    Finished release [optimized] target(s) in 0.16s
##[endgroup]
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, tool: "lld-wrapper", path: "src/tools/lld-wrapper", mode: ToolStd, is_optional_tool: false, source_type: InTree, extra_features: [], allow_features: "" } -- 0.179
[TIMING] tool::LldWrapper { compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[TIMING] compile::StartupObjects { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
[TIMING] llvm::Sanitizers { target: x86_64-unknown-linux-gnu } -- 0.000
##[group]Building stage1 library artifacts (x86_64-unknown-linux-gnu)
Building stage1 library artifacts (x86_64-unknown-linux-gnu)
Building stage1 library artifacts (x86_64-unknown-linux-gnu)
   Compiling cc v1.0.77
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.143
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] build_script_build test:false 0.169
[RUSTC-TIMING] build_script_build test:false 0.216
[RUSTC-TIMING] cc test:false 0.673
   Compiling compiler_builtins v0.1.91
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
[RUSTC-TIMING] build_script_build test:false 0.133
[RUSTC-TIMING] build_script_build test:false 0.158
[RUSTC-TIMING] build_script_build test:false 0.337
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.059
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
   Compiling rustc-demangle v0.1.21
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] cfg_if test:false 0.090
[RUSTC-TIMING] adler test:false 0.298
[RUSTC-TIMING] adler test:false 0.298
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] libc test:false 1.100
[RUSTC-TIMING] compiler_builtins test:false 1.411
[RUSTC-TIMING] memchr test:false 1.697
[RUSTC-TIMING] rustc_demangle test:false 1.924
[RUSTC-TIMING] rustc_demangle test:false 1.924
   Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/library/rustc-std-workspace-alloc)
   Compiling panic_abort v0.0.0 (/checkout/library/panic_abort)
   Compiling panic_unwind v0.0.0 (/checkout/library/panic_unwind)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling miniz_oxide v0.6.2
   Compiling hashbrown v0.13.1
   Compiling object v0.30.1
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_std_workspace_alloc test:false 0.056
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] panic_unwind test:false 0.121
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] std_detect test:false 0.261
[RUSTC-TIMING] std_detect test:false 0.261
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] alloc test:false 3.741
[RUSTC-TIMING] hashbrown test:false 0.866
[RUSTC-TIMING] miniz_oxide test:false 0.977
   Compiling addr2line v0.19.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] core test:false 22.967
[RUSTC-TIMING] gimli test:false 4.492
[RUSTC-TIMING] object test:false 4.917
[RUSTC-TIMING] object test:false 4.917
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc-std-workspace-std v1.99.0 (/checkout/library/rustc-std-workspace-std)
   Compiling proc_macro v0.0.0 (/checkout/library/proc_macro)
   Compiling proc_macro v0.0.0 (/checkout/library/proc_macro)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_std_workspace_std test:false 0.060
[RUSTC-TIMING] rustc_std_workspace_std test:false 0.060
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling getopts v0.2.21
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] getopts test:false 1.610
   Compiling test v0.0.0 (/checkout/library/test)
   Compiling test v0.0.0 (/checkout/library/test)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] test test:false 11.297
   Compiling sysroot v0.0.0 (/checkout/library/sysroot)
   Compiling sysroot v0.0.0 (/checkout/library/sysroot)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
    Finished release [optimized + debuginfo] target(s) in 58.79s
##[endgroup]
[TIMING] compile::StdLink { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target_compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, crates: [] } -- 0.000
[TIMING] compile::Std { target: x86_64-unknown-linux-gnu, compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, crates: [] } -- 58.863
---
   Compiling cc v1.0.77
   Compiling log v0.4.14
   Compiling parking_lot_core v0.8.5
   Compiling memchr v2.5.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] stable_deref_trait test:false 0.127
   Compiling instant v0.1.12
   Compiling instant v0.1.12
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling scopeguard v1.1.0
[RUSTC-TIMING] unicode_ident test:false 0.200
   Compiling proc-macro-hack v0.5.19
   Compiling proc-macro-hack v0.5.19
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rustc-hash v1.1.0
[RUSTC-TIMING] instant test:false 0.146
   Compiling tracing-core v0.1.28
   Compiling tracing-core v0.1.28
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] build_script_build test:false 0.327
   Compiling pin-project-lite v0.2.8
   Compiling pin-project-lite v0.2.8
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling typenum v1.16.0
   Compiling typenum v1.16.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] rustc_hash test:false 0.160
   Compiling bitflags v1.3.2
[RUSTC-TIMING] build_script_build test:false 0.452
   Compiling thiserror v1.0.38
   Compiling thiserror v1.0.38
[RUSTC-TIMING] build_script_build test:false 0.448
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling either v1.6.0
   Compiling either v1.6.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling thin-vec v0.2.12
   Compiling remove_dir_all v0.5.3
[RUSTC-TIMING] build_script_build test:false 0.551
[RUSTC-TIMING] build_script_build test:false 0.551
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling arrayvec v0.7.0
[RUSTC-TIMING] build_script_build test:false 0.598
[RUSTC-TIMING] build_script_build test:false 0.617
[RUSTC-TIMING] build_script_build test:false 0.617
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling fastrand v1.8.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] remove_dir_all test:false 0.143
   Compiling ena v0.14.2
   Compiling ena v0.14.2
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling itertools v0.10.5
[RUSTC-TIMING] version_check test:false 0.792
[RUSTC-TIMING] version_check test:false 0.792
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling ahash v0.7.4
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling indexmap v1.9.3
   Compiling indexmap v1.9.3
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling lock_api v0.4.7
   Compiling lock_api v0.4.7
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling generic-array v0.14.4
   Compiling generic-array v0.14.4
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
[RUSTC-TIMING] build_script_build test:false 0.230
   Compiling elsa v1.7.1
   Compiling elsa v1.7.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] thin_vec test:false 0.640
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
[RUSTC-TIMING] build_script_build test:false 0.247
[RUSTC-TIMING] ena test:false 0.516
[RUSTC-TIMING] ena test:false 0.516
   Compiling ppv-lite86 v0.2.8
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling cpufeatures v0.2.5
   Compiling unicode-width v0.1.10
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_arena test:false 0.329
[RUSTC-TIMING] rustc_arena test:false 0.329
   Compiling unic-common v0.9.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] elsa test:false 0.364
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] cpufeatures test:false 0.125
   Compiling serde_derive v1.0.160
[RUSTC-TIMING] fastrand test:false 0.914
   Compiling litemap v0.7.0
   Compiling litemap v0.7.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling unic-char-range v0.9.0
[RUSTC-TIMING] unicode_width test:false 0.196
   Compiling writeable v0.5.1
   Compiling writeable v0.5.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] unic_char_range test:false 0.288
   Compiling syn v2.0.8
   Compiling syn v2.0.8
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling scoped-tls v1.0.0
[RUSTC-TIMING] rustc_graphviz test:false 0.758
   Compiling unic-char-property v0.9.0
   Compiling unic-char-property v0.9.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling getrandom v0.2.8
[RUSTC-TIMING] build_script_build test:false 0.466
[RUSTC-TIMING] build_script_build test:false 0.466
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling memmap2 v0.2.1
   Compiling memmap2 v0.2.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling perf-event-open-sys v1.0.1
[RUSTC-TIMING] unic_char_property test:false 0.205
   Compiling jobserver v0.1.26
   Compiling jobserver v0.1.26
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] proc_macro2 test:false 1.604
   Compiling rand_core v0.6.4
   Compiling rand_core v0.6.4
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling tempfile v3.3.0
   Compiling tempfile v3.3.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling parking_lot v0.11.2
   Compiling parking_lot v0.11.2
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rand_chacha v0.3.0
   Compiling rand_chacha v0.3.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling psm v0.1.21
[RUSTC-TIMING] memmap2 test:false 0.541
   Compiling stacker v0.1.15
   Compiling stacker v0.1.15
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling hashbrown v0.12.3
[RUSTC-TIMING] rand_core test:false 0.528
[RUSTC-TIMING] rand_core test:false 0.528
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] tracing_core test:false 2.572
   Compiling unic-ucd-version v0.9.0
   Compiling unic-ucd-version v0.9.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling measureme v10.1.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rand v0.8.5
[RUSTC-TIMING] build_script_build test:false 0.339
[RUSTC-TIMING] parking_lot_core test:false 1.007
[RUSTC-TIMING] build_script_build test:false 0.371
[RUSTC-TIMING] build_script_build test:false 0.371
   Compiling serde v1.0.160
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling unic-emoji-char v0.9.0
   Compiling unic-emoji-char v0.9.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] typenum test:false 1.498
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] rustc_hash test:false 0.143
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling type-map v0.4.0
   Compiling type-map v0.4.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling ryu v1.0.5
   Compiling ryu v1.0.5
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] type_map test:false 0.315
   Compiling rustc_lexer v0.1.0 (/checkout/compiler/rustc_lexer)
   Compiling rustc_lexer v0.1.0 (/checkout/compiler/rustc_lexer)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling regex-automata v0.2.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] unic_emoji_char test:false 0.629
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] stacker test:false 0.573
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling self_cell v0.10.2
   Compiling self_cell v0.10.2
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling serde_json v1.0.85
   Compiling serde_json v1.0.85
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] self_cell test:false 0.175
[RUSTC-TIMING] self_cell test:false 0.175
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] smallvec test:false 0.381
   Compiling rand_xoshiro v0.6.0
   Compiling rand_xoshiro v0.6.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling itoa v1.0.6
   Compiling itoa v1.0.6
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling annotate-snippets v0.9.1
[RUSTC-TIMING] unicode_width test:false 0.159
[RUSTC-TIMING] unicode_width test:false 0.159
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling crypto-common v0.1.6
   Compiling crypto-common v0.1.6
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] itoa test:false 0.233
   Compiling block-buffer v0.10.2
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
[RUSTC-TIMING] build_script_build test:false 0.441
[RUSTC-TIMING] build_script_build test:false 0.441
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling odht v0.3.1
   Compiling termize v0.1.1
   Compiling termize v0.1.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_fs_util test:false 0.162
   Compiling digest v0.10.6
   Compiling digest v0.10.6
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] ryu test:false 0.520
   Compiling termcolor v1.1.3
   Compiling lazy_static v1.4.0
   Compiling lazy_static v1.4.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling getopts v0.2.21
   Compiling getopts v0.2.21
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling semver v1.0.12
[RUSTC-TIMING] parking_lot test:false 2.317
   Compiling memoffset v0.8.0
   Compiling memoffset v0.8.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rustc_serialize v0.0.0 (/checkout/compiler/rustc_serialize)
   Compiling rustc_serialize v0.0.0 (/checkout/compiler/rustc_serialize)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling sha1 v0.10.5
[RUSTC-TIMING] build_script_build test:false 0.245
   Compiling sha2 v0.10.6
   Compiling sha2 v0.10.6
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] indexmap test:false 1.040
   Compiling md-5 v0.10.0
   Compiling md-5 v0.10.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling datafrog v2.0.1
   Compiling datafrog v2.0.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] measureme test:false 2.438
   Compiling convert_case v0.4.0
   Compiling convert_case v0.4.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling regex-syntax v0.6.26
   Compiling regex-syntax v0.6.26
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling tinyvec_macros v0.1.0
   Compiling tinyvec_macros v0.1.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling tinyvec v1.6.0
   Compiling tinyvec v1.6.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling aho-corasick v0.7.18
[RUSTC-TIMING] termcolor test:false 1.116
   Compiling crossbeam-utils v0.8.14
   Compiling crossbeam-utils v0.8.14
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_version v0.4.0
[RUSTC-TIMING] rand_xoshiro test:false 1.734
   Compiling polonius-engine v0.13.0
   Compiling polonius-engine v0.13.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling ahash v0.8.2
   Compiling ahash v0.8.2
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] semver test:false 0.773
   Compiling static_assertions v1.1.0
   Compiling crc32fast v1.3.2
   Compiling crc32fast v1.3.2
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] build_script_build test:false 0.475
[RUSTC-TIMING] build_script_build test:false 0.475
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling twox-hash v1.6.3
   Compiling twox-hash v1.6.3
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling tracing-log v0.1.2
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling field-offset v0.3.5
[RUSTC-TIMING] build_script_build test:false 0.448
[RUSTC-TIMING] build_script_build test:false 0.448
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling sharded-slab v0.1.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling thread_local v1.1.4
[RUSTC-TIMING] regex_automata test:false 3.201
   Compiling adler v1.0.2
   Compiling adler v1.0.2
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling crossbeam-channel v0.5.6
   Compiling crossbeam-channel v0.5.6
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] build_script_build test:false 0.329
   Compiling ansi_term v0.12.1
   Compiling ansi_term v0.12.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling snap v1.1.0
   Compiling snap v1.1.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling unicode-normalization v0.1.22
   Compiling unicode-normalization v0.1.22
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling miniz_oxide v0.6.2
   Compiling miniz_oxide v0.6.2
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] build_script_build test:false 0.530
[RUSTC-TIMING] build_script_build test:false 0.530
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling atty v0.2.14
[RUSTC-TIMING] annotate_snippets test:false 3.130
   Compiling byteorder v1.4.3
   Compiling byteorder v1.4.3
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] atty test:false 0.154
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling libloading v0.7.1
   Compiling libloading v0.7.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling fixedbitset v0.2.0
   Compiling fixedbitset v0.2.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling hashbrown v0.13.1
   Compiling hashbrown v0.13.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling unicase v2.6.0
   Compiling unicase v2.6.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling flate2 v1.0.25
[RUSTC-TIMING] byteorder test:false 0.550
   Compiling petgraph v0.5.1
   Compiling petgraph v0.5.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling synstructure v0.13.0
   Compiling synstructure v0.13.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling unicode-script v0.5.5
   Compiling unicode-script v0.5.5
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling fallible-iterator v0.2.0
   Compiling fallible-iterator v0.2.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] crossbeam_channel test:false 1.881
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling memoffset v0.6.5
   Compiling memoffset v0.6.5
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rustc-demangle v0.1.21
   Compiling rustc-demangle v0.1.21
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling regex v1.5.6
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling regex-automata v0.1.10
   Compiling regex-automata v0.1.10
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling gimli v0.26.2
[RUSTC-TIMING] snap test:false 1.503
   Compiling unicode-security v0.1.0
   Compiling unicode-security v0.1.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling punycode v0.4.1
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling pulldown-cmark v0.9.2
   Compiling pulldown-cmark v0.9.2
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] flate2 test:false 1.437
[RUSTC-TIMING] build_script_build test:false 0.236
[RUSTC-TIMING] build_script_build test:false 0.236
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling synstructure v0.12.6
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] synstructure test:false 1.665
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling pathdiff v0.2.1
[RUSTC-TIMING] unicase test:false 0.519
[RUSTC-TIMING] unicase test:false 0.519
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] pathdiff test:false 0.124
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling cstr v0.2.8
   Compiling cstr v0.2.8
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling matchers v0.1.0
   Compiling matchers v0.1.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling jemalloc-sys v0.5.3+5.3.0-patched
   Compiling jemalloc-sys v0.5.3+5.3.0-patched
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rustc_error_codes v0.0.0 (/checkout/compiler/rustc_error_codes)
   Compiling rustc_error_codes v0.0.0 (/checkout/compiler/rustc_error_codes)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_error_codes test:false 0.227
[RUSTC-TIMING] unicode_security test:false 2.040
[RUSTC-TIMING] rustc_demangle test:false 2.484
[RUSTC-TIMING] rustc_demangle test:false 2.484
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] build_script_build test:false 0.377
[RUSTC-TIMING] synstructure test:false 1.720
[RUSTC-TIMING] build_script_build test:false 0.725
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] syn test:false 11.354
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[RUSTC-TIMING] syn test:false 12.798
   Compiling zerofrom-derive v0.1.1
   Compiling zerofrom-derive v0.1.1
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling yoke-derive v0.7.0
   Compiling yoke-derive v0.7.0
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling zerovec-derive v0.9.3
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling displaydoc v0.2.3
   Compiling displaydoc v0.2.3
---
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_incremental test:false 7.521
   Compiling rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
   Compiling rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_transmute test:false 8.863
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_metadata test:false 30.431
[RUSTC-TIMING] rustc_metadata test:false 30.431
   Compiling rustc_hir_analysis v0.0.0 (/checkout/compiler/rustc_hir_analysis)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
   Compiling rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_traits test:false 49.992
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
   Compiling rustc_driver_impl v0.0.0 (/checkout/compiler/rustc_driver_impl)
   Compiling rustc_driver_impl v0.0.0 (/checkout/compiler/rustc_driver_impl)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_ty_utils test:false 30.673
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_passes test:false 34.639
---
[RUSTC-TIMING] rustc_mir_transform test:false 74.400
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_hir_typeck test:false 62.061
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
[RUSTC-TIMING] rustc_driver test:false 250.993
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
    Finished release [optimized] target(s) in 7m 46s
##[endgroup]
[TIMING] compile::RustcLink { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target_compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, crates: [] } -- 0.004
[TIMING] compile::Rustc { target: x86_64-unknown-linux-gnu, compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, crates: [] } -- 466.329
[TIMING] compile::Rustc { target: x86_64-unknown-linux-gnu, compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, crates: [] } -- 466.329
Assembling stage2 compiler
[TIMING] compile::Sysroot { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu } } -- 0.000
[TIMING] builder::Builder::sysroot_libdir::Libdir { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
##[group]Building stage1 tool lld-wrapper (x86_64-unknown-linux-gnu)
Building stage1 tool lld-wrapper (x86_64-unknown-linux-gnu)
   Compiling lld-wrapper v0.1.0 (/checkout/src/tools/lld-wrapper)
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Error: Failed to write file "default.profraw": Read-only file system
    Finished release [optimized] target(s) in 0.55s
##[endgroup]
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, tool: "lld-wrapper", path: "src/tools/lld-wrapper", mode: ToolStd, is_optional_tool: false, source_type: InTree, extra_features: [], allow_features: "" } -- 0.571
[TIMING] tool::LldWrapper { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
[TIMING] tool::LldWrapper { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
[TIMING] compile::StartupObjects { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
Uplifting library (stage1 -> stage2)
[TIMING] compile::StdLink { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target_compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, crates: ["std"] } -- 0.000
[TIMING] compile::Std { target: x86_64-unknown-linux-gnu, compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, crates: ["std"] } -- 0.000
---
.bootstrap::compile::Std                                             0.00s
..bootstrap::compile::StartupObjects                                 0.00s
..bootstrap::compile::StdLink                                        0.00s

stage-build INFO: Section `Stage 1b (LLVM CS PGO) > Build rustc and LLVM` ended: OK (1980.51s)
stage-build INFO: Section `Stage 1b (LLVM CS PGO) > Gather profiles` starts
stage-build INFO: Running benchmarks with CS PGO instrumented LLVM
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage/opt-artifacts`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage/opt-artifacts`
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib -Copt-level=3 /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage/opt-artifacts`
LLVM Profile Warning: %m specifier can only be specified once in /tmp/tmp-multistage/opt-artifacts/llvm-pgo/%4m.profraw/default_%m.profraw.
stage-build DEBUG: Changing working dir from `/checkout/obj` to `/tmp/tmp-multistage/opt-artifacts/rustc-perf`
stage-build INFO: Executing `RUST_LOG=collector=debug RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
---
Executing benchmark cargo-0.60.0 (1/8)
Preparing cargo-0.60.0
[2023-05-23T09:01:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T09:01:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T09:01:57Z DEBUG collector::execute] cd "/tmp/.tmps6Gnt1" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmps6Gnt1#cargo@0.60.0" "--lib" "--" "--skip-this-rustc"
[2023-05-23T09:01:57Z DEBUG collector::execute] cd "/tmp/.tmpLwelHG" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLwelHG#cargo@0.60.0" "--release" "--lib" "--" "--skip-this-rustc"
[2023-05-23T09:02:50Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:02:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T09:02:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T09:02:50Z DEBUG collector::execute] cd "/tmp/.tmpjDSTkv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjDSTkv#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-23T09:03:20Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:03:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T09:03:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T09:03:21Z DEBUG collector::execute] cd "/tmp/.tmpIdecAK" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIdecAK#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark clap-3.1.6 (2/8)
Preparing clap-3.1.6
[2023-05-23T09:04:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T09:04:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
Executing benchmark hyper-0.14.18 (3/8)
Preparing hyper-0.14.18
[2023-05-23T09:04:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T09:04:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T09:04:42Z DEBUG collector::execute] cd "/tmp/.tmpBpNYQY" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBpNYQY#hyper@0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-23T09:04:42Z DEBUG collector::execute] cd "/tmp/.tmpauWTaE" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpauWTaE#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2023-05-23T09:05:06Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:05:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T09:05:07Z DEBUG collector::execute] cd "/tmp/.tmpsRtT8O" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpsRtT8O#hyper@0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Running hyper-0.14.18: Opt + [Full]
---
Executing benchmark regex-1.5.5 (4/8)
Preparing regex-1.5.5
[2023-05-23T09:05:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T09:05:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T09:05:19Z DEBUG collector::execute] cd "/tmp/.tmpVNhBVP" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVNhBVP#regex@1.5.5" "--" "--skip-this-rustc"
[2023-05-23T09:05:19Z DEBUG collector::execute] cd "/tmp/.tmpSadDv4" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSadDv4#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2023-05-23T09:05:26Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:05:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T09:05:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T09:05:26Z DEBUG collector::execute] cd "/tmp/.tmpU3YujK" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpU3YujK#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-23T09:05:29Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:05:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T09:05:29Z DEBUG collector::execute] cd "/tmp/.tmpaDpL2V" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaDpL2V#regex@1.5.5" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark regex-1.5.5 (4/8)
Finished benchmark regex-1.5.5 (4/8)
Executing benchmark ripgrep-13.0.0 (5/8)
Preparing ripgrep-13.0.0
[2023-05-23T09:05:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T09:05:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T09:05:37Z DEBUG collector::execute] cd "/tmp/.tmpXKnOEH" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXKnOEH#ripgrep@13.0.0" "--" "--skip-this-rustc"
[2023-05-23T09:05:37Z DEBUG collector::execute] cd "/tmp/.tmpYKhI0K" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYKhI0K#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
[2023-05-23T09:06:04Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:06:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T09:06:04Z DEBUG collector::execute] cd "/tmp/.tmpJD54FQ" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJD54FQ#ripgrep@13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
Running ripgrep-13.0.0: Opt + [Full]
[2023-05-23T09:06:08Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:06:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T09:06:08Z DEBUG collector::execute] cd "/tmp/.tmpLwSYJL" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLwSYJL#ripgrep@13.0.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark ripgrep-13.0.0-tiny (6/8)
Preparing ripgrep-13.0.0-tiny
[2023-05-23T09:06:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T09:06:19Z DEBUG collector::execute] cd "/tmp/.tmpSLnWFh" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSLnWFh#ripgrep@13.0.0" "--release" "--" "--skip-this-rustc"
---
Executing benchmark serde-1.0.136 (7/8)
Preparing serde-1.0.136
[2023-05-23T09:07:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T09:07:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T09:07:11Z DEBUG collector::execute] cd "/tmp/.tmpLXqTYA" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLXqTYA#serde@1.0.136" "--release" "--" "--skip-this-rustc"
[2023-05-23T09:07:11Z DEBUG collector::execute] cd "/tmp/.tmpB4QNfE" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpB4QNfE#serde@1.0.136" "--" "--skip-this-rustc"
[2023-05-23T09:07:11Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:07:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T09:07:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T09:07:11Z DEBUG collector::execute] cd "/tmp/.tmpWUvD3x" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWUvD3x#serde@1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-23T09:07:15Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:07:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T09:07:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T09:07:15Z DEBUG collector::execute] cd "/tmp/.tmpVMRXAv" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVMRXAv#serde@1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Executing benchmark syn-1.0.89 (8/8)
Preparing syn-1.0.89
[2023-05-23T09:07:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-05-23T09:07:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T09:07:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-05-23T09:07:18Z DEBUG collector::execute] cd "/tmp/.tmpTxcKt7" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTxcKt7#syn@1.0.89" "--" "--skip-this-rustc"
[2023-05-23T09:07:18Z DEBUG collector::execute] cd "/tmp/.tmpNvdtc3" && CARGO_INCREMENTAL="0" CARGO_MAKEFLAGS="-j --jobserver-fds=6,7 --jobserver-auth=6,7" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNvdtc3#syn@1.0.89" "--release" "--" "--skip-this-rustc"
[2023-05-23T09:07:21Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:07:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T09:07:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-05-23T09:07:21Z DEBUG collector::execute] cd "/tmp/.tmpMUxW0P" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMUxW0P#syn@1.0.89" "--" "--wrap-rustc-with" "Eprintln"
[2023-05-23T09:07:24Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-05-23T09:07:24Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-05-23T09:07:24Z DEBUG collector::execute] cd "/tmp/.tmpTqqE42" && CARGO_INCREMENTAL="0" EXPECT_ONLY_WRAPPED_RUSTC="1" RUSTC="/tmp/tmp-multistage/opt-artifacts/rustc-perf/target/debug/rustc-fake" RUSTC_BOOTSTRAP="1" RUSTC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTqqE42#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (8/8)
Finished benchmark syn-1.0.89 (8/8)
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging LLVM CS PGO profiles to /tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata
stage-build INFO: Executing `/rustroot/bin/llvm-profdata merge -o /tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata /tmp/tmp-multistage/opt-artifacts/llvm-pgo /tmp/tmp-multistage/opt-artifacts/llvm-pgo-intermediate.profdata`
error: /tmp/tmp-multistage/opt-artifacts/llvm-pgo: No such file or directory
stage-build INFO: Section `Stage 1b (LLVM CS PGO) > Gather profiles` ended: FAIL (362.96s)
stage-build INFO: Section `Stage 1b (LLVM CS PGO)` ended: FAIL (2343.47s)
stage-build ERROR: The multi-stage build has failed
------------------------------------------------
Stage 1 (LLVM PGO):            1939.36s (45.28%)
  Build rustc and LLVM:        1460.78s (34.11%)
    LLVM:                       486.57s (11.36%)
    LLVM:                       486.57s (11.36%)
    Rustc:                      955.40s (22.31%)
  Gather profiles:              478.59s (11.17%)
Stage 1b (LLVM CS PGO):        2343.47s (54.72%)
    LLVM:                      1417.88s (33.11%)
    Rustc:                      543.98s (12.70%)
  Gather profiles:              362.96s ( 8.47%)
                                                
                                                
Total duration:                       1h 11m 23s
------------------------------------------------
root INFO: Free disk space: 503.27 GiB out of total 581.32 GiB (13.42% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 924, in <module>
    run(runner)
  File "../src/ci/stage-build.py", line 914, in run
    raise e
  File "../src/ci/stage-build.py", line 911, in run
    execute_build_pipeline(timer, pipeline, runner, build_args)
  File "../src/ci/stage-build.py", line 838, in execute_build_pipeline
    gather_llvm_cs_profiles(pipeline, runner)
  File "../src/ci/stage-build.py", line 665, in gather_llvm_cs_profiles
    pipeline.llvm_profile_merged_file_intermediate()
  File "../src/ci/stage-build.py", line 455, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/rustroot/bin/llvm-profdata', 'merge', '-o', '/tmp/tmp-multistage/opt-artifacts/llvm-pgo.profdata', '/tmp/tmp-multistage/opt-artifacts/llvm-pgo', '/tmp/tmp-multistage/opt-artifacts/llvm-pgo-intermediate.profdata']' returned non-zero exit status 1.
