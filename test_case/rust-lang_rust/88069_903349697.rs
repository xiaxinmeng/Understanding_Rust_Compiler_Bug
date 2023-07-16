plain
 ---> 34b81583a195
Step 25/37 : ENV CC=clang CXX=clang++
 ---> Using cache
 ---> b827571bd577
Step 26/37 : ENV PERF_COMMIT 1e19fc4c6168d2f7596e512f42f358f245d8f09d
 ---> 408fa3a837e9
 ---> 408fa3a837e9
Step 27/37 : RUN curl -LS -o perf.zip https://github.com/rust-lang/rustc-perf/archive/$PERF_COMMIT.zip &&     unzip perf.zip &&     mv rustc-perf-$PERF_COMMIT rustc-perf &&     rm perf.zip
 ---> 08ae741e6f95
Step 28/37 : COPY scripts/sccache.sh /scripts/
 ---> Using cache
 ---> ffad7b02145d
---
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] StdLink { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target_compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.001
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 56.796
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_BUILD_INSTRUMENTED=IR" "-DLLVM_BUILD_RUNTIME=No" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.56.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=clang" "-DCMAKE_CXX_COMPILER=clang++" "-DCMAKE_ASM_COMPILER=clang" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm -static-libstdc++" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 12.0.1
-- The ASM compiler identification is Clang
-- Found assembler: /rustroot/bin/clang
-- Check for working C compiler: /rustroot/bin/clang
---
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
Build completed successfully in 0:20:16
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib ../library/core/src/lib.rs
LLVM Profile Error: Runtime and instrumentation version mismatch : expected 5, but get 7
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib -Copt-level=3 ../library/core/src/lib.rs
LLVM Profile Error: Runtime and instrumentation version mismatch : expected 5, but get 7
+ cp -r /tmp/rustc-perf ./
++ whoami
+ chown -R user: ./rustc-perf
+ RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
+ RUSTC_BOOTSTRAP=1
+ RUSTC_BOOTSTRAP=1
+ /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build -p collector
    Updating git repository `https://github.com/rust-lang/measureme`
    Updating git repository `https://github.com/rust-lang/team`
---
   Compiling linked-hash-map v0.5.3
   Compiling regex-syntax v0.6.18
   Compiling quick-error v1.2.3
   Compiling snap v1.0.1
   Compiling bumpalo v3.4.0
   Compiling fallible-streaming-iterator v0.1.9
   Compiling termcolor v1.1.0
   Compiling arc-swap v0.4.8
   Compiling strsim v0.8.0
   Compiling ansi_term v0.11.0
---
   Compiling lock_api v0.3.4
   Compiling phf_shared v0.8.0
   Compiling unicase v2.6.0
   Compiling humantime v1.3.0
   Compiling lru-cache v0.1.2
   Compiling semver v0.9.0
   Compiling phf v0.8.0
   Compiling openssl-sys v0.9.58
   Compiling libsqlite3-sys v0.18.0
---
   Compiling futures v0.3.7
   Compiling hyper v0.13.10
   Compiling serde_json v1.0.53
   Compiling chrono v0.4.11
   Compiling serde_urlencoded v0.6.1
   Compiling intern v0.1.0 (/checkout/obj/rustc-perf/intern)
   Compiling postgres-types v0.1.1
   Compiling tokio-postgres v0.5.4
   Compiling hyper-tls v0.4.1
   Compiling reqwest v0.10.6
   Compiling postgres-native-tls v0.3.0
   Compiling database v0.1.0 (/checkout/obj/rustc-perf/database)
   Compiling rustc-artifacts v0.2.2
   Compiling collector v0.1.0 (/checkout/obj/rustc-perf/collector)
    Finished dev [unoptimized + debuginfo] target(s) in 34.13s
+ RUST_LOG=collector=debug
+ RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
+ RUSTC_BOOTSTRAP=1
+ /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc Test --builds Check,Debug,Opt --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --runs All --include externs,ctfe-stress-4,inflate,cargo,token-stream-stress,match-stress-enum
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc Test --builds Check,Debug,Opt --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --runs All --include externs,ctfe-stress-4,inflate,cargo,token-stream-stress,match-stress-enum`
[2021-08-22T23:29:41Z DEBUG collector] benchmark rust-mozjs - ignored
[2021-08-22T23:29:41Z DEBUG collector] benchmark native-tls-0.1.5 - ignored
[2021-08-22T23:29:41Z DEBUG collector] benchmark README.md - ignored
[2021-08-22T23:29:41Z DEBUG collector] benchmark LICENSE.md - ignored
[2021-08-22T23:29:41Z DEBUG collector] benchmark deeply-nested - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark stm32f4 - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark hyper-2 - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark webrender-wrench - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark coercions - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark webrender - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark serde - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark `cargo`- registered
[2021-08-22T23:29:41Z DEBUG collector] benchmark deep-vector - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark unused-warnings - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark derive - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark syn - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark `externs`- registered
[2021-08-22T23:29:41Z DEBUG collector] benchmark encoding - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark `match-stress-enum`- registered
[2021-08-22T23:29:41Z DEBUG collector] benchmark regression-31157 - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark tokio-webpush-simple - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark style-servo - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark deeply-nested-async - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark ripgrep - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark `inflate`- registered
[2021-08-22T23:29:41Z DEBUG collector] benchmark await-call-tree - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark keccak - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark many-assoc-items - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark html5ever - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark packed-simd - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark piston-image - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark unify-linearly - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark regex - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark diesel - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark match-stress-exhaustive_patterns - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark wg-grammar - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark wf-projection-stress-65510 - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark issue-46449 - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark deeply-nested-closures - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark helloworld - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark clap-rs - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark cranelift-codegen - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark `token-stream-stress`- registered
[2021-08-22T23:29:41Z DEBUG collector] benchmark ucd - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark `ctfe-stress-4`- registered
[2021-08-22T23:29:41Z DEBUG collector] benchmark tuple-stress - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark issue-58319 - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark unicode_normalization - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark futures - doesn't match --include argument, skipping
[2021-08-22T23:29:41Z DEBUG collector] benchmark rustc - doesn't match --include argument, skipping
Profiling with Eprintln
6 benchmarks remaining
Preparing cargo
[2021-08-22T23:29:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T23:29:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T23:29:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T23:29:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnv2rQU#cargo:0.29.0" "--lib" "--" "--skip-this-rustc"
[2021-08-22T23:29:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnuans7#cargo:0.29.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2021-08-22T23:29:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9iCbA2#cargo:0.29.0" "--release" "--lib" "--" "--skip-this-rustc"
Running cargo: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:31:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:31:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T23:31:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphn0pMs#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:32:12Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:32:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphn0pMs#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphn0pMs/incremental-state"
[2021-08-22T23:32:28Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:32:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphn0pMs#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphn0pMs/incremental-state"
[2021-08-22T23:32:31Z DEBUG collector::execute] applying patch println
[2021-08-22T23:32:31Z DEBUG collector::execute] applying println to "/tmp/.tmphn0pMs"
[2021-08-22T23:32:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T23:32:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphn0pMs#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphn0pMs/incremental-state"
Running cargo: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:32:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:32:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T23:32:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5BFpWZ#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:33:10Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:33:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5BFpWZ#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp5BFpWZ/incremental-state"
[2021-08-22T23:33:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:33:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5BFpWZ#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp5BFpWZ/incremental-state"
[2021-08-22T23:33:56Z DEBUG collector::execute] applying patch println
[2021-08-22T23:33:56Z DEBUG collector::execute] applying println to "/tmp/.tmp5BFpWZ"
[2021-08-22T23:33:56Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T23:33:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5BFpWZ#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp5BFpWZ/incremental-state"
Running cargo: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:34:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:34:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T23:34:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvWloLD#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:35:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:35:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvWloLD#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvWloLD/incremental-state"
[2021-08-22T23:37:13Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:37:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvWloLD#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvWloLD/incremental-state"
[2021-08-22T23:37:19Z DEBUG collector::execute] applying patch println
[2021-08-22T23:37:19Z DEBUG collector::execute] applying println to "/tmp/.tmpvWloLD"
[2021-08-22T23:37:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T23:37:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvWloLD#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvWloLD/incremental-state"
5 benchmarks remaining
Preparing ctfe-stress-4
[2021-08-22T23:38:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T23:38:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T23:38:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T23:38:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaMywwm#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-08-22T23:38:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0sQRYm#ctfe-stress-4:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-08-22T23:38:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplhYsg6#ctfe-stress-4:0.1.0" "--" "--skip-this-rustc"
Running ctfe-stress-4: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:38:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:38:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T23:38:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTes9l8#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:38:38Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:38:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTes9l8#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpTes9l8/incremental-state"
[2021-08-22T23:39:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:39:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTes9l8#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpTes9l8/incremental-state"
Running ctfe-stress-4: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:39:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:39:10Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T23:39:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW4Vser#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:39:34Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:39:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW4Vser#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpW4Vser/incremental-state"
[2021-08-22T23:40:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:40:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW4Vser#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpW4Vser/incremental-state"
Running ctfe-stress-4: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:40:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:40:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T23:40:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpa2SuNZ#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:40:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:40:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpa2SuNZ#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpa2SuNZ/incremental-state"
[2021-08-22T23:41:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:41:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpa2SuNZ#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpa2SuNZ/incremental-state"
4 benchmarks remaining
Preparing externs
[2021-08-22T23:41:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T23:41:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T23:41:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T23:41:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWdUEcD#externs:0.1.0" "--" "--skip-this-rustc"
[2021-08-22T23:41:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjTjGzL#externs:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-08-22T23:41:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQvR45w#externs:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
Running externs: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:41:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:41:03Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T23:41:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplZDBzg#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:41:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:41:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplZDBzg#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmplZDBzg/incremental-state"
[2021-08-22T23:41:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:41:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplZDBzg#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmplZDBzg/incremental-state"
Running externs: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:41:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:41:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T23:41:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0G5AtV#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:41:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:41:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0G5AtV#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp0G5AtV/incremental-state"
[2021-08-22T23:41:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:41:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0G5AtV#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp0G5AtV/incremental-state"
Running externs: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:41:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:41:08Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T23:41:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCk1uQV#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:41:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:41:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCk1uQV#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCk1uQV/incremental-state"
[2021-08-22T23:41:11Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:41:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCk1uQV#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCk1uQV/incremental-state"
3 benchmarks remaining
Preparing inflate
[2021-08-22T23:41:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T23:41:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T23:41:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T23:41:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJ3ncZg#inflate:0.1.0" "--" "--skip-this-rustc"
[2021-08-22T23:41:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpczO8XK#inflate:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-08-22T23:41:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7P8Ux5#inflate:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
Running inflate: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:41:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:41:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T23:41:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIz5u8l#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:41:14Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:41:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIz5u8l#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpIz5u8l/incremental-state"
[2021-08-22T23:41:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:41:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIz5u8l#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpIz5u8l/incremental-state"
[2021-08-22T23:41:17Z DEBUG collector::execute] applying patch println
[2021-08-22T23:41:17Z DEBUG collector::execute] applying println to "/tmp/.tmpIz5u8l"
[2021-08-22T23:41:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T23:41:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIz5u8l#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpIz5u8l/incremental-state"
Running inflate: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:41:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:41:20Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T23:41:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCqoQI2#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:41:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:41:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCqoQI2#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCqoQI2/incremental-state"
[2021-08-22T23:41:27Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:41:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCqoQI2#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCqoQI2/incremental-state"
[2021-08-22T23:41:27Z DEBUG collector::execute] applying patch println
[2021-08-22T23:41:27Z DEBUG collector::execute] applying println to "/tmp/.tmpCqoQI2"
[2021-08-22T23:41:27Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T23:41:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCqoQI2#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCqoQI2/incremental-state"
Running inflate: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:41:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:41:31Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T23:41:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu4N0uV#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:41:40Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:41:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu4N0uV#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpu4N0uV/incremental-state"
[2021-08-22T23:41:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:41:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu4N0uV#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpu4N0uV/incremental-state"
[2021-08-22T23:41:50Z DEBUG collector::execute] applying patch println
[2021-08-22T23:41:50Z DEBUG collector::execute] applying println to "/tmp/.tmpu4N0uV"
[2021-08-22T23:41:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-08-22T23:41:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu4N0uV#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpu4N0uV/incremental-state"
2 benchmarks remaining
Preparing match-stress-enum
[2021-08-22T23:42:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T23:42:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T23:42:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T23:42:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmHMZGU#match-stress-enum:0.1.0" "--" "--skip-this-rustc"
[2021-08-22T23:42:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc6Dwwi#match-stress-enum:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-08-22T23:42:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpq4EKLB#match-stress-enum:0.1.0" "--release" "--" "--skip-this-rustc"
Running match-stress-enum: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:42:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:42:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T23:42:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1HgWRV#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:42:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:42:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1HgWRV#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp1HgWRV/incremental-state"
[2021-08-22T23:42:05Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:42:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1HgWRV#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp1HgWRV/incremental-state"
Running match-stress-enum: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:42:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:42:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T23:42:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxL12dm#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:42:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:42:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxL12dm#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpxL12dm/incremental-state"
[2021-08-22T23:42:10Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:42:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxL12dm#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpxL12dm/incremental-state"
Running match-stress-enum: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:42:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:42:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T23:42:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpt3zbKW#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:42:13Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:42:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpt3zbKW#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpt3zbKW/incremental-state"
[2021-08-22T23:42:15Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:42:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpt3zbKW#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpt3zbKW/incremental-state"
1 benchmark remaining
Preparing token-stream-stress
[2021-08-22T23:42:16Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-08-22T23:42:16Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-08-22T23:42:16Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-08-22T23:42:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfcvfni#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2021-08-22T23:42:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgYcaxp#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2021-08-22T23:42:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprqSJv1#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
Running token-stream-stress: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:42:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:42:17Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-08-22T23:42:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4OW68v#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:42:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:42:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4OW68v#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4OW68v/incremental-state"
[2021-08-22T23:42:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:42:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp4OW68v#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp4OW68v/incremental-state"
Running token-stream-stress: Debug + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:42:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:42:18Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-08-22T23:42:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpONRxSf#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:42:18Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:42:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpONRxSf#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpONRxSf/incremental-state"
[2021-08-22T23:42:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:42:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpONRxSf#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpONRxSf/incremental-state"
Running token-stream-stress: Opt + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-08-22T23:42:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-08-22T23:42:19Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-08-22T23:42:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJuHwDE#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-08-22T23:42:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-08-22T23:42:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJuHwDE#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJuHwDE/incremental-state"
[2021-08-22T23:42:20Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-08-22T23:42:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJuHwDE#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJuHwDE/incremental-state"
+ cd /checkout/obj
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/build/profiles
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
Generating unstable book md files (x86_64-unknown-linux-gnu)
[TIMING] Assemble { target_compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
[TIMING] Sysroot { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 0.000
[TIMING] Libdir { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 0.000
---
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeBeadsGen.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeEmitterGen.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenDAGPatterns.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenHwModes.cpp.o
warning: /checkout/src/llvm-project/llvm/utils/count/count.c: Function control flow change detected (hash mismatch) main Hash = 650973722546133261 [-Wbackend-plugin]
[  1%] Linking C executable ../../bin/count
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  1%] Built target count
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenInstruction.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenInstruction.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenMapTable.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
warning: /checkout/src/llvm-project/llvm/utils/PerfectShuffle/PerfectShuffle.cpp: Function control flow change detected (hash mismatch) main Hash = 899207464865258103 [-Wbackend-plugin]
[  1%] Linking CXX executable ../../bin/llvm-PerfectShuffle
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenRegisters.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
---
[  8%] Linking CXX static library ../libLLVMExtensions.a
[  8%] Built target LLVMBitstreamReader
[  8%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/CodeViewRecordIO.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
warning: /checkout/src/llvm-project/llvm/utils/not/not.cpp: Function control flow change detected (hash mismatch) main Hash = 862289824575759136 [-Wbackend-plugin]
[  8%] Built target LLVMExtensions
[  8%] Linking CXX executable ../../bin/not
[  8%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/ContinuationRecordBuilder.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
---
[  8%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/ELF.cpp.o
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  8%] Linking CXX static library ../libLLVMOption.a
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
warning: /checkout/src/llvm-project/llvm/utils/yaml-bench/YAMLBench.cpp: Function control flow change detected (hash mismatch) main Hash = 909850505536080120 [-Wbackend-plugin]
[  8%] Built target LLVMWindowsManifest
[  8%] Linking CXX executable ../../bin/yaml-bench
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  8%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MachO.cpp.o
---
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  8%] Linking CXX static library ../../libLLVMDebugInfoMSF.a
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
warning: /checkout/src/llvm-project/llvm/utils/FileCheck/FileCheck.cpp: Function control flow change detected (hash mismatch) main Hash = 346383081048862348 [-Wbackend-plugin]
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[  8%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSubsectionRecord.cpp.o
[  8%] Built target LLVMDebugInfoMSF
[  8%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Record.cpp.o
---
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang-12: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 13%] Linking CXX static library ../libLLVMMCA.a
[ 13%] Built target LLVMMCA
LLVM ERROR: Function Import: link error: linking module flags 'ProfileSummary': IDs have conflicting values in 'CMakeFiles/obj.llvm-tblgen.dir/X86ModRMFilters.cpp.o' and 'CMakeFiles/obj.llvm-tblgen.dir/X86DisassemblerTables.cpp.o'
PLEASE submit a bug report to https://bugs.llvm.org/ and include the crash backtrace.
 #0 0x000000000086c60e llvm::sys::PrintStackTrace(llvm::raw_ostream&, int) (/rustroot/bin/ld.lld+0x86c60e)
 #1 0x000000000086a57a llvm::sys::RunSignalHandlers() (/rustroot/bin/ld.lld+0x86a57a)
 #2 0x000000000086a6bc SignalHandler(int) (/rustroot/bin/ld.lld+0x86a6bc)
 #3 0x000014cd30190ff0 __restore_rt (/lib/libpthread.so.0+0xeff0)
 #4 0x000014cd2edcbf15 raise (/lib/libc.so.6+0x31f15)
 #5 0x000014cd2edced20 abort (/lib/libc.so.6+0x34d20)
 #6 0x0000000000824b03 llvm::report_fatal_error(llvm::Twine const&, bool) (/rustroot/bin/ld.lld+0x824b03)
 #7 0x0000000000824c3e (/rustroot/bin/ld.lld+0x824c3e)
 #8 0x00000000018b21d2 llvm::FunctionImporter::importFunctions(llvm::Module&, llvm::StringMap<std::unordered_set<unsigned long, std::hash<unsigned long>, std::equal_to<unsigned long>, std::allocator<unsigned long> >, llvm::MallocAllocator> const&) (/rustroot/bin/ld.lld+0x18b21d2)
 #9 0x0000000001367f1c llvm::lto::thinBackend(llvm::lto::Config const&, unsigned int, std::function<std::unique_ptr<llvm::lto::NativeObjectStream, std::default_delete<llvm::lto::NativeObjectStream> > (unsigned int)>, llvm::Module&, llvm::ModuleSummaryIndex const&, llvm::StringMap<std::unordered_set<unsigned long, std::hash<unsigned long>, std::equal_to<unsigned long>, std::allocator<unsigned long> >, llvm::MallocAllocator> const&, llvm::DenseMap<unsigned long, llvm::GlobalValueSummary*, llvm::DenseMapInfo<unsigned long>, llvm::detail::DenseMapPair<unsigned long, llvm::GlobalValueSummary*> > const&, llvm::MapVector<llvm::StringRef, llvm::BitcodeModule, llvm::DenseMap<llvm::StringRef, unsigned int, llvm::DenseMapInfo<llvm::StringRef>, llvm::detail::DenseMapPair<llvm::StringRef, unsigned int> >, std::vector<std::pair<llvm::StringRef, llvm::BitcodeModule>, std::allocator<std::pair<llvm::StringRef, llvm::BitcodeModule> > > >&, std::vector<unsigned char, std::allocator<unsigned char> > const&) (/rustroot/bin/ld.lld+0x1367f1c)
#10 0x0000000001350d96 (anonymous namespace)::InProcessThinBackend::runThinLTOBackendThread(std::function<std::unique_ptr<llvm::lto::NativeObjectStream, std::default_delete<llvm::lto::NativeObjectStream> > (unsigned int)>, std::function<std::function<std::unique_ptr<llvm::lto::NativeObjectStream, std::default_delete<llvm::lto::NativeObjectStream> > (unsigned int)> (unsigned int, llvm::StringRef)>, unsigned int, llvm::BitcodeModule, llvm::ModuleSummaryIndex&, llvm::StringMap<std::unordered_set<unsigned long, std::hash<unsigned long>, std::equal_to<unsigned long>, std::allocator<unsigned long> >, llvm::MallocAllocator> const&, llvm::DenseSet<llvm::ValueInfo, llvm::DenseMapInfo<llvm::ValueInfo> > const&, std::map<unsigned long, llvm::GlobalValue::LinkageTypes, std::less<unsigned long>, std::allocator<std::pair<unsigned long const, llvm::GlobalValue::LinkageTypes> > > const&, llvm::DenseMap<unsigned long, llvm::GlobalValueSummary*, llvm::DenseMapInfo<unsigned long>, llvm::detail::DenseMapPair<unsigned long, llvm::GlobalValueSummary*> > const&, llvm::MapVector<llvm::StringRef, llvm::BitcodeModule, llvm::DenseMap<llvm::StringRef, unsigned int, llvm::DenseMapInfo<llvm::StringRef>, llvm::detail::DenseMapPair<llvm::StringRef, unsigned int> >, std::vector<std::pair<llvm::StringRef, llvm::BitcodeModule>, std::allocator<std::pair<llvm::StringRef, llvm::BitcodeModule> > > >&)::'lambda'(std::function<std::unique_ptr<llvm::lto::NativeObjectStream, std::default_delete<llvm::lto::NativeObjectStream> > (unsigned int)>)::operator()(std::function<std::unique_ptr<llvm::lto::NativeObjectStream, std::default_delete<llvm::lto::NativeObjectStream> > (unsigned int)>) const (/rustroot/bin/ld.lld+0x1350d96)
#11 0x000000000135cd01 std::_Function_handler<void (), std::_Bind<(anonymous namespace)::InProcessThinBackend::start(unsigned int, llvm::BitcodeModule, llvm::StringMap<std::unordered_set<unsigned long, std::hash<unsigned long>, std::equal_to<unsigned long>, std::allocator<unsigned long> >, llvm::MallocAllocator> const&, llvm::DenseSet<llvm::ValueInfo, llvm::DenseMapInfo<llvm::ValueInfo> > const&, std::map<unsigned long, llvm::GlobalValue::LinkageTypes, std::less<unsigned long>, std::allocator<std::pair<unsigned long const, llvm::GlobalValue::LinkageTypes> > > const&, llvm::MapVector<llvm::StringRef, llvm::BitcodeModule, llvm::DenseMap<llvm::StringRef, unsigned int, llvm::DenseMapInfo<llvm::StringRef>, llvm::detail::DenseMapPair<llvm::StringRef, unsigned int> >, std::vector<std::pair<llvm::StringRef, llvm::BitcodeModule>, std::allocator<std::pair<llvm::StringRef, llvm::BitcodeModule> > > >&)::'lambda'(llvm::BitcodeModule, llvm::ModuleSummaryIndex&, llvm::StringMap<std::unordered_set<unsigned long, std::hash<unsigned long>, std::equal_to<unsigned long>, std::allocator<unsigned long> >, llvm::MallocAllocator> const&, llvm::DenseSet<llvm::ValueInfo, llvm::DenseMapInfo<llvm::ValueInfo> > const&, std::map<unsigned long, llvm::GlobalValue::LinkageTypes, std::less<unsigned long>, std::allocator<std::pair<unsigned long const, llvm::GlobalValue::LinkageTypes> > > const&, llvm::DenseMap<unsigned long, llvm::GlobalValueSummary*, llvm::DenseMapInfo<unsigned long>, llvm::detail::DenseMapPair<unsigned long, llvm::GlobalValueSummary*> > const&, llvm::MapVector<llvm::StringRef, llvm::BitcodeModule, llvm::DenseMap<llvm::StringRef, unsigned int, llvm::DenseMapInfo<llvm::StringRef>, llvm::detail::DenseMapPair<llvm::StringRef, unsigned int> >, std::vector<std::pair<llvm::StringRef, llvm::BitcodeModule>, std::allocator<std::pair<llvm::StringRef, llvm::BitcodeModule> > > >&) (llvm::BitcodeModule, std::reference_wrapper<llvm::ModuleSummaryIndex>, std::reference_wrapper<llvm::StringMap<std::unordered_set<unsigned long, std::hash<unsigned long>, std::equal_to<unsigned long>, std::allocator<unsigned long> >, llvm::MallocAllocator> const>, std::reference_wrapper<llvm::DenseSet<llvm::ValueInfo, llvm::DenseMapInfo<llvm::ValueInfo> > const>, std::reference_wrapper<std::map<unsigned long, llvm::GlobalValue::LinkageTypes, std::less<unsigned long>, std::allocator<std::pair<unsigned long const, llvm::GlobalValue::LinkageTypes> > > const>, std::reference_wrapper<llvm::DenseMap<unsigned long, llvm::GlobalValueSummary*, llvm::DenseMapInfo<unsigned long>, llvm::detail::DenseMapPair<unsigned long, llvm::GlobalValueSummary*> > const>, std::reference_wrapper<llvm::MapVector<llvm::StringRef, llvm::BitcodeModule, llvm::DenseMap<llvm::StringRef, unsigned int, llvm::DenseMapInfo<llvm::StringRef>, llvm::detail::DenseMapPair<llvm::StringRef, unsigned int> >, std::vector<std::pair<llvm::StringRef, llvm::BitcodeModule>, std::allocator<std::pair<llvm::StringRef, llvm::BitcodeModule> > > > >)> >::_M_invoke(std::_Any_data const&) (/rustroot/bin/ld.lld+0x135cd01)
#12 0x00000000026de8b0 std::_Function_handler<std::unique_ptr<std::__future_base::_Result_base, std::__future_base::_Result_base::_Deleter> (), std::__future_base::_Task_setter<std::unique_ptr<std::__future_base::_Result<void>, std::__future_base::_Result_base::_Deleter>, std::_Bind_simple<std::reference_wrapper<std::function<void ()> > ()>, void> >::_M_invoke(std::_Any_data const&) (/rustroot/bin/ld.lld+0x26de8b0)
#13 0x00000000008ac12b std::__future_base::_State_baseV2::_M_do_set(std::function<std::unique_ptr<std::__future_base::_Result_base, std::__future_base::_Result_base::_Deleter> ()>*, bool*) (/rustroot/bin/ld.lld+0x8ac12b)
#14 0x000014cd3018e7a3 pthread_once (/lib/libpthread.so.0+0xc7a3)
#15 0x00000000026df437 std::thread::_Impl<std::_Bind_simple<llvm::ThreadPool::ThreadPool(llvm::ThreadPoolStrategy)::'lambda'() ()> >::_M_run() (/rustroot/bin/ld.lld+0x26df437)
#16 0x000014cd2f653750 std::__shared_count<(__gnu_cxx::_Lock_policy)2>::~__shared_count() /tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/bits/shared_ptr_base.h:658:0
#17 0x000014cd2f653750 std::__shared_ptr<std::thread::_Impl_base, (__gnu_cxx::_Lock_policy)2>::~__shared_ptr() /tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/bits/shared_ptr_base.h:925:0
#18 0x000014cd2f653750 std::shared_ptr<std::thread::_Impl_base>::~shared_ptr() /tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/include/bits/shared_ptr.h:93:0
#19 0x000014cd2f653750 execute_native_thread_routine /tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/src/c++11/../../../../../gcc-5.5.0/libstdc++-v3/src/c++11/thread.cc:79:0
#20 0x000014cd301888ca start_thread (/lib/libpthread.so.0+0x68ca)
#21 0x000014cd2ee72abd clone (/lib/libc.so.6+0xd8abd)
clang-12: error: unable to execute command: Aborted (core dumped)
clang-12: error: linker command failed due to signal (use -v to see invocation)
make[2]: *** [bin/llvm-tblgen] Error 254
make[1]: *** [utils/TableGen/CMakeFiles/llvm-tblgen.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[ 14%] Linking CXX static library ../../libLLVMMCParser.a
[ 14%] Built target LLVMMCParser
[ 14%] Built target LLVMMCParser
make: *** [all] Error 2
 finished in 38.349 seconds
command did not execute successfully, got: exit status: 2


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
Build completed unsuccessfully in 0:01:28
