plain
Successfully built 4a420d64ab2f
Successfully tagged rust-ci:latest
Built container sha256:4a420d64ab2f4ac3cb53e1969b085295b538deb16ce20e8e564bb2a6f6c42de0
Uploading finished image to https://ci-caches.rust-lang.org/docker/88f64ab586024253b8399b49d7c85418f6e26ee21b2fdbe84432a7eca85ce92374ffe1feb77b23180a9eb043bf32a7bf1539f6edeef05bdaafe699608a5d8546
upload failed: - to s3://rust-lang-ci-sccache2/docker/88f64ab586024253b8399b49d7c85418f6e26ee21b2fdbe84432a7eca85ce92374ffe1feb77b23180a9eb043bf32a7bf1539f6edeef05bdaafe699608a5d8546 Unable to locate credentials
useradd: warning: the home directory already exists.
Not copying any file from skel directory into it.
[CI_JOB_NAME=dist-x86_64-linux]
---
Hugetlb:               0 kB
DirectMap4k:      217024 kB
DirectMap2M:    10268672 kB
DirectMap1G:    50331648 kB
15-01-2023 00:08:18 INFO stage-build: Running multi-stage build using Python 3.6.8 (default, Nov 16 2020, 16:55:22) 
[GCC 4.8.5 20150623 (Red Hat 4.8.5-44)]
15-01-2023 00:08:18 INFO stage-build: Environment values
{ 'AWS_ACCESS_KEY_ID': '',
  'AWS_SECRET_ACCESS_KEY': '',
  'BASE_COMMIT': 'b8f9cb345ab1401f2fbd14cc23f64dda9dd2314e',
  'CARGO_HOME': '/cargo',
  'CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER': 'clang',
  'CC': 'clang',
  'CI': 'true',
  'CI_JOB_NAME': 'dist-x86_64-linux',
  'COMPILETEST_NEEDS_ALL_LLVM_COMPONENTS': '1',
  'CXX': 'clang++',
  'DEPLOY': '1',
  'DIST_REQUIRE_ALL_TOOLS': '1',
  'DIST_SRC': '1',
  'GITHUB_ACTIONS': 'true',
  'GITHUB_REF': 'refs/pull/103019/merge',
  'HOME': '/home/user',
  'HOSTNAME': '891c8c14873c',
  'HOSTS': 'x86_64-unknown-linux-gnu',
  'LD_LIBRARY_PATH': '/rustroot/lib64:/rustroot/lib32:/rustroot/lib',
  'LIBCURL_NO_PKG_CONFIG': '1',
  'MIRRORS_BASE': 'https://ci-mirrors.rust-lang.org/rustc',
  'PATH': '/rustroot/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin',
  'PERF_COMMIT': '3c253134664fdcba862c539d37f0de18557a9a4c',
  'PGO_HOST': 'x86_64-unknown-linux-gnu',
  'PKG_CONFIG_PATH': '/rustroot/lib/pkgconfig',
  'PWD': '/checkout/obj',
  'RUST_CONFIGURE_ARGS': '--enable-full-tools       --enable-sanitizers       '
                         '--enable-profiler       --enable-compiler-docs       '
                         '--set '
                         'target.x86_64-unknown-linux-gnu.linker=clang       '
                         '--set '
                         'target.x86_64-unknown-linux-gnu.ar=/rustroot/bin/llvm-ar       '
                         '--set '
                         'target.x86_64-unknown-linux-gnu.ranlib=/rustroot/bin/llvm-ranlib       '
                         '--set llvm.thin-lto=true       --set '
                         'llvm.ninja=false       --set rust.jemalloc       '
                         '--set rust.use-lld=true       --set rust.lto=thin '
                         '--enable-sccache --disable-manage-submodules '
                         '--enable-locked-deps --enable-cargo-native-static '
                         '--set rust.codegen-units-std=1 '
                         '--dist-compression-formats=xz '
                         '--release-channel=nightly '
                         '--enable-llvm-static-stdcpp --set '
                         'rust.remap-debuginfo --debuginfo-level-std=1 '
                         '--enable-missing-tools',
  'RUST_RELEASE_CHANNEL': 'nightly',
  'SCCACHE_BUCKET': 'rust-lang-ci-sccache2',
  'SCRIPT': 'python3 ../src/ci/stage-build.py python3 ../x.py dist     --host '
            'x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu     '
            '--include-default-paths     build-manifest bootstrap',
  'SHLVL': '3',
  'SRC': '/checkout',
  'TOOLSTATE_REPO': 'https://github.com/rust-lang-nursery/rust-toolstate',
  'TOOLSTATE_REPO_ACCESS_TOKEN': '',
  '_': '/usr/bin/python3'}
15-01-2023 00:08:19 INFO stage-build: Executing `chown -R user: /tmp/tmp-multistage/rustc-perf`
15-01-2023 00:08:19 DEBUG stage-build: Changing working dir from `/checkout/obj` to `/tmp/tmp-multistage/rustc-perf`
15-01-2023 00:08:19 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build -p collector`
    Updating git repository `https://github.com/rust-lang/team`
    Updating git repository `https://github.com/rust-lang/measureme`
---
   Compiling reqwest v0.11.3
   Compiling database v0.1.0 (/tmp/tmp-multistage/rustc-perf/database)
   Compiling collector v0.1.0 (/tmp/tmp-multistage/rustc-perf/collector)
    Finished dev [unoptimized + debuginfo] target(s) in 32.74s
15-01-2023 00:08:52 DEBUG stage-build: Reverting working dir to `/checkout/obj`
15-01-2023 00:08:52 INFO stage-build: Stage `Build rustc (LLVM PGO)` starts
15-01-2023 00:08:52 INFO stage-build: Executing `/usr/bin/python3 /checkout/x.py build --target x86_64-unknown-linux-gnu --host x86_64-unknown-linux-gnu --stage 2 library/std --llvm-profile-generate`
    Finished dev [unoptimized] target(s) in 0.06s
Building stage0 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
    Finished release [optimized] target(s) in 1.17s
Uplifting stage1 library (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 library from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Build completed successfully in 0:34:50
15-01-2023 00:43:43 INFO stage-build: Stage `Build rustc (LLVM PGO)` ended: OK (2090.91s)
15-01-2023 00:43:43 INFO stage-build: Stage `Gather profiles (LLVM PGO)` starts
15-01-2023 00:43:43 INFO stage-build: Running benchmarks with PGO instrumented LLVM
15-01-2023 00:43:43 DEBUG stage-build: Changing working dir from `/checkout/obj` to `/checkout/obj`
15-01-2023 00:43:43 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
15-01-2023 00:43:56 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib -Copt-level=3 /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
15-01-2023 00:44:12 DEBUG stage-build: Reverting working dir to `/checkout/obj`
15-01-2023 00:44:12 DEBUG stage-build: Changing working dir from `/checkout/obj` to `/tmp/tmp-multistage/rustc-perf`
15-01-2023 00:44:12 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
[2023-01-15T00:44:13Z DEBUG collector] benchmark LICENSE.md - ignored
[2023-01-15T00:44:13Z DEBUG collector] benchmark README.md - ignored
[2023-01-15T00:44:13Z DEBUG collector] benchmark `regex-1.5.5`- registered
---
[2023-01-15T00:50:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T00:50:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T00:50:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8WLoQQ#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (7/7)
15-01-2023 00:51:03 DEBUG stage-build: Reverting working dir to `/checkout/obj`
15-01-2023 00:51:03 INFO stage-build: Merging LLVM PGO profiles to /tmp/tmp-multistage/llvm-pgo.profdata
15-01-2023 00:51:03 INFO stage-build: Executing `/rustroot/bin/llvm-profdata merge -o /tmp/tmp-multistage/llvm-pgo.profdata /tmp/tmp-multistage/llvm-pgo`
15-01-2023 00:51:29 INFO stage-build: LLVM PGO statistics
15-01-2023 00:51:29 INFO stage-build: /tmp/tmp-multistage/llvm-pgo.profdata: 0.0 Byte
15-01-2023 00:51:29 INFO stage-build: /tmp/tmp-multistage/llvm-pgo: 16.58 GiB
15-01-2023 00:51:29 INFO stage-build: Profile file count
15-01-2023 00:51:29 INFO stage-build: 1263
15-01-2023 00:51:29 INFO stage-build: Deleting directory `/tmp/tmp-multistage/llvm-pgo`
15-01-2023 00:51:31 INFO stage-build: Stage `Gather profiles (LLVM PGO)` ended: OK (468.37s)
15-01-2023 00:51:31 INFO stage-build: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm`
15-01-2023 00:51:32 INFO stage-build: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/lld`
15-01-2023 00:51:32 INFO stage-build: Stage `Build rustc (rustc PGO)` starts
15-01-2023 00:51:32 INFO stage-build: Executing `/usr/bin/python3 /checkout/x.py build --target x86_64-unknown-linux-gnu --host x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate /tmp/tmp-multistage/rustc-pgo`
    Finished dev [unoptimized] target(s) in 0.06s
Building stage0 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.23s
Copying stage0 library from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
    Finished release [optimized] target(s) in 0.60s
Uplifting stage1 library (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 library from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Build completed successfully in 0:11:05
15-01-2023 01:02:38 INFO stage-build: Stage `Build rustc (rustc PGO)` ended: OK (665.59s)
15-01-2023 01:02:38 INFO stage-build: Stage `Gather profiles (rustc PGO)` starts
15-01-2023 01:02:38 INFO stage-build: Running benchmarks with PGO instrumented rustc
15-01-2023 01:02:38 DEBUG stage-build: Changing working dir from `/checkout/obj` to `/checkout/obj`
15-01-2023 01:02:38 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
15-01-2023 01:03:12 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib -Copt-level=3 /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
15-01-2023 01:03:51 DEBUG stage-build: Reverting working dir to `/checkout/obj`
15-01-2023 01:03:51 DEBUG stage-build: Changing working dir from `/checkout/obj` to `/tmp/tmp-multistage/rustc-perf`
15-01-2023 01:03:51 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Check,Debug,Opt --scenarios All --include externs,ctfe-stress-5,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0`
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Check,Debug,Opt --scenarios All --include externs,ctfe-stress-5,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0`
[2023-01-15T01:03:51Z DEBUG collector] benchmark LICENSE.md - ignored
[2023-01-15T01:03:51Z DEBUG collector] benchmark README.md - ignored
[2023-01-15T01:03:51Z DEBUG collector] benchmark `match-stress`- registered
---
[2023-01-15T01:03:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:03:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-15T01:03:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMQIufJ#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:04:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T01:04:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMQIufJ#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMQIufJ/incremental-state"
[2023-01-15T01:04:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:04:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMQIufJ#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMQIufJ/incremental-state"
[2023-01-15T01:04:04Z DEBUG collector::execute] applying println to "/tmp/.tmpMQIufJ"
[2023-01-15T01:04:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T01:04:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T01:04:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMQIufJ#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMQIufJ/incremental-state"
[2023-01-15T01:04:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:04:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T01:04:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu0etJv#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:04:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T01:04:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T01:04:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu0etJv#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpu0etJv/incremental-state"
[2023-01-15T01:04:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:04:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu0etJv#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpu0etJv/incremental-state"
[2023-01-15T01:04:11Z DEBUG collector::execute] applying println to "/tmp/.tmpu0etJv"
[2023-01-15T01:04:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T01:04:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T01:04:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpu0etJv#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpu0etJv/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-01-15T01:04:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-15T01:04:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-15T01:06:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:06:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-15T01:06:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKEXpDo#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:07:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T01:07:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKEXpDo#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKEXpDo/incremental-state"
[2023-01-15T01:08:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:08:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKEXpDo#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKEXpDo/incremental-state"
[2023-01-15T01:08:15Z DEBUG collector::execute] applying println to "/tmp/.tmpKEXpDo"
[2023-01-15T01:08:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T01:08:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T01:08:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKEXpDo#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpKEXpDo/incremental-state"
[2023-01-15T01:08:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:08:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T01:08:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2U6ieI#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:09:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-01-15T01:12:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:12:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-15T01:12:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYGTlvf#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:12:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-15T01:12:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYGTlvf#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYGTlvf/incremental-state"
[2023-01-15T01:12:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:12:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYGTlvf#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYGTlvf/incremental-state"
[2023-01-15T01:12:52Z DEBUG collector::execute] applying println to "/tmp/.tmpYGTlvf"
[2023-01-15T01:12:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T01:12:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T01:12:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYGTlvf#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpYGTlvf/incremental-state"
[2023-01-15T01:12:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:12:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-15T01:12:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiajqix#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:13:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T01:13:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T01:13:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiajqix#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpiajqix/incremental-state"
[2023-01-15T01:13:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:13:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiajqix#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpiajqix/incremental-state"
[2023-01-15T01:13:40Z DEBUG collector::execute] applying println to "/tmp/.tmpiajqix"
[2023-01-15T01:13:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T01:13:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T01:13:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiajqix#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpiajqix/incremental-state"
[2023-01-15T01:13:44Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:13:44Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T01:13:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZmrATB#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:14:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-01-15T01:14:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:14:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-15T01:14:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQUUFhl#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:14:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-15T01:14:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQUUFhl#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQUUFhl/incremental-state"
[2023-01-15T01:14:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:14:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQUUFhl#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQUUFhl/incremental-state"
[2023-01-15T01:14:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:14:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-15T01:14:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6h7Oa3#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:14:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-15T01:14:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:14:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-15T01:14:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3WIkK#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:14:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-15T01:14:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3WIkK#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH3WIkK/incremental-state"
[2023-01-15T01:14:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:14:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH3WIkK#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH3WIkK/incremental-state"
[2023-01-15T01:14:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:14:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-15T01:14:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpslqyso#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:14:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T01:14:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T01:14:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpslqyso#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpslqyso/incremental-state"
[2023-01-15T01:14:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:14:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpslqyso#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpslqyso/incremental-state"
[2023-01-15T01:14:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:14:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T01:14:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzkEwPE#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:14:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T01:14:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T01:14:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzkEwPE#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzkEwPE/incremental-state"
[2023-01-15T01:14:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:14:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzkEwPE#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzkEwPE/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-01-15T01:15:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-15T01:15:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-15T01:15:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:15:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-15T01:15:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPuzotI#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:15:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T01:15:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPuzotI#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPuzotI/incremental-state"
[2023-01-15T01:15:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:15:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPuzotI#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpPuzotI/incremental-state"
[2023-01-15T01:15:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:15:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T01:15:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRL9Olx#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T01:15:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T01:15:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T01:15:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRL9Olx#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRL9Olx/incremental-state"
[2023-01-15T01:15:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T01:15:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRL9Olx#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpRL9Olx/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-01-15T01:15:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-01-15T01:15:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2023-01-15T01:15:54Z DEBUG collector::execute] applying new row to "/tmp/.tmpg2gX0a"
[2023-01-15T01:15:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-15T01:15:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpg2gX0a#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpg2gX0a/incremental-state"
Finished benchmark tuple-stress (8/8)
15-01-2023 01:16:00 DEBUG stage-build: Reverting working dir to `/checkout/obj`
15-01-2023 01:16:00 INFO stage-build: Merging Rustc PGO profiles to /tmp/tmp-multistage/rustc-pgo.profdata
15-01-2023 01:16:00 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-multistage/rustc-pgo.profdata /tmp/tmp-multistage/rustc-pgo`
15-01-2023 01:16:02 INFO stage-build: Rustc PGO statistics
15-01-2023 01:16:02 INFO stage-build: /tmp/tmp-multistage/rustc-pgo.profdata: 0.0 Byte
15-01-2023 01:16:02 INFO stage-build: /tmp/tmp-multistage/rustc-pgo: 31.56 MiB
15-01-2023 01:16:02 INFO stage-build: Profile file count
15-01-2023 01:16:02 INFO stage-build: 2
15-01-2023 01:16:02 INFO stage-build: Deleting directory `/tmp/tmp-multistage/rustc-pgo`
15-01-2023 01:16:02 INFO stage-build: Stage `Gather profiles (rustc PGO)` ended: OK (803.85s)
15-01-2023 01:16:02 INFO stage-build: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm`
15-01-2023 01:16:02 INFO stage-build: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/lld`
15-01-2023 01:16:02 INFO stage-build: Stage `Build rustc (LLVM BOLT)` starts
15-01-2023 01:16:02 INFO stage-build: Executing `/usr/bin/python3 /checkout/x.py build --target x86_64-unknown-linux-gnu --host x86_64-unknown-linux-gnu --stage 2 library/std --llvm-profile-use /tmp/tmp-multistage/llvm-pgo.profdata --llvm-bolt-profile-generate`
    Finished dev [unoptimized] target(s) in 0.06s
Building stage0 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.23s
Copying stage0 library from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
    Finished release [optimized] target(s) in 7.31s
Uplifting stage1 library (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 library from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Build completed successfully in 0:31:03
15-01-2023 01:47:06 INFO stage-build: Stage `Build rustc (LLVM BOLT)` ended: OK (1863.31s)
15-01-2023 01:47:06 INFO stage-build: Stage `Gather profiles (LLVM BOLT)` starts
15-01-2023 01:47:06 INFO stage-build: Running benchmarks with BOLT instrumented LLVM
15-01-2023 01:47:06 DEBUG stage-build: Changing working dir from `/checkout/obj` to `/checkout/obj`
15-01-2023 01:47:06 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
15-01-2023 01:47:22 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib -Copt-level=3 /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
15-01-2023 01:47:44 DEBUG stage-build: Reverting working dir to `/checkout/obj`
15-01-2023 01:47:44 DEBUG stage-build: Changing working dir from `/checkout/obj` to `/tmp/tmp-multistage/rustc-perf`
15-01-2023 01:47:44 INFO stage-build: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Check,Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Check,Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
[2023-01-15T01:47:44Z DEBUG collector] benchmark LICENSE.md - ignored
[2023-01-15T01:47:44Z DEBUG collector] benchmark README.md - ignored
[2023-01-15T01:47:44Z DEBUG collector] benchmark `regex-1.5.5`- registered
---
[2023-01-15T01:57:35Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T01:57:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T01:57:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp6IDuAN#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (7/7)
15-01-2023 01:57:49 DEBUG stage-build: Reverting working dir to `/checkout/obj`
15-01-2023 01:57:49 INFO stage-build: Merging LLVM BOLT profiles to /tmp/tmp-multistage/bolt.profdata
15-01-2023 01:57:49 INFO stage-build: Executing `/rustroot/bin/llvm-profdata merge -o /tmp/tmp-multistage/bolt.profdata /tmp/tmp-multistage/llvm-pgo`
error: /tmp/tmp-multistage/llvm-pgo: No such file or directory
15-01-2023 01:57:49 INFO stage-build: Stage `Gather profiles (LLVM BOLT)` ended: FAIL (642.86s)
15-01-2023 01:57:49 ERROR stage-build: The multi-stage build has failed
Timer results
---------------------------------------------------------
---------------------------------------------------------
Build rustc (LLVM PGO):                 2090.91s (32.00%)
Gather profiles (LLVM PGO):              468.37s ( 7.17%)
Build rustc (rustc PGO):                 665.59s (10.19%)
Gather profiles (rustc PGO):             803.85s (12.30%)
Build rustc (LLVM BOLT):                1863.31s (28.51%)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 596, in <module>
    raise e
  File "../src/ci/stage-build.py", line 593, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 565, in execute_build_pipeline
    gather_llvm_bolt_profiles(pipeline)
  File "../src/ci/stage-build.py", line 494, in gather_llvm_bolt_profiles
    pipeline.llvm_profile_dir_root()
Gather profiles (LLVM BOLT):             642.86s ( 9.84%)
Total duration:                         6534.90s
---------------------------------------------------------
  File "../src/ci/stage-build.py", line 301, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/rustroot/bin/llvm-profdata', 'merge', '-o', '/tmp/tmp-multistage/bolt.profdata', '/tmp/tmp-multistage/llvm-pgo']' returned non-zero exit status 1.
