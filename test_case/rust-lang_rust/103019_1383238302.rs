plain
Step 3/27 : RUN yum upgrade -y &&     yum install -y epel-release &&     yum install -y       automake       bzip2       file       cmake3       gcc       gcc-c++       git       glibc-devel.i686       glibc-devel.x86_64       libedit-devel       libstdc++-devel.i686       libstdc++-devel.x86_64       make       ncurses-devel       openssl-devel       patch       perl       pkgconfig       python3       unzip       wget       xz       zlib-devel.i686       zlib-devel.x86_64       && yum clean all
 ---> Running in 99f797484dea
Loaded plugins: fastestmirror, ovl
Determining fastest mirrors
 * base: mirrors.advancedhosters.com
 * extras: mirror.ette.biz
Resolving Dependencies
--> Running transaction check
---> Package bash.x86_64 0:4.2.46-34.el7 will be updated
---> Package bash.x86_64 0:4.2.46-35.el7_9 will be an update
---

Complete!
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: mirrors.advancedhosters.com
 * extras: mirror.ette.biz
Resolving Dependencies
--> Running transaction check
---> Package epel-release.noarch 0:7-11 will be installed
--> Finished Dependency Resolution
---
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: mirrors.advancedhosters.com
 * epel: forksystems.mm.fcix.net
 * extras: mirror.ette.biz
http://mirror.es.its.nyu.edu/epel/7/x86_64/repodata/repomd.xml: [Errno 12] Timeout on http://mirror.es.its.nyu.edu/epel/7/x86_64/repodata/repomd.xml: (28, 'Connection timed out after 30001 milliseconds')
Trying other mirror.
Package 1:pkgconfig-0.27.1-4.el7.x86_64 already installed and latest version
Package xz-5.2.2-2.el7_9.x86_64 already installed and latest version
---
Successfully built c4e1319e85e8
Successfully tagged rust-ci:latest
Built container sha256:c4e1319e85e87ec6eff1358378b7a7b01111345581a0ce47b53788b0bc5f4617
Uploading finished image to https://ci-caches.rust-lang.org/docker/88f64ab586024253b8399b49d7c85418f6e26ee21b2fdbe84432a7eca85ce92374ffe1feb77b23180a9eb043bf32a7bf1539f6edeef05bdaafe699608a5d8546
upload failed: - to s3://rust-lang-ci-sccache2/docker/88f64ab586024253b8399b49d7c85418f6e26ee21b2fdbe84432a7eca85ce92374ffe1feb77b23180a9eb043bf32a7bf1539f6edeef05bdaafe699608a5d8546 Unable to locate credentials
useradd: warning: the home directory already exists.
Not copying any file from skel directory into it.
[CI_JOB_NAME=dist-x86_64-linux]
---
Hugetlb:               0 kB
DirectMap4k:      210880 kB
DirectMap2M:     9226240 kB
DirectMap1G:    51380224 kB
stage-build INFO: Running multi-stage build using Python 3.6.8 (default, Nov 16 2020, 16:55:22) 
[GCC 4.8.5 20150623 (Red Hat 4.8.5-44)]
stage-build INFO: Environment values
{ 'AWS_ACCESS_KEY_ID': '',
  'AWS_SECRET_ACCESS_KEY': '',
  'BASE_COMMIT': 'fc11ee02ee91b32e23684cd478bca80fe5323b47',
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
  'HOSTNAME': '30c51e8cb1d9',
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
stage-build INFO: Executing `chown -R user: /tmp/tmp-multistage/rustc-perf`
stage-build DEBUG: Changing working dir from `/checkout/obj` to `/tmp/tmp-multistage/rustc-perf`
stage-build INFO: Executing `RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build -p collector`
    Updating git repository `https://github.com/rust-lang/team`
    Updating git repository `https://github.com/rust-lang/measureme`
---
   Compiling reqwest v0.11.3
   Compiling database v0.1.0 (/tmp/tmp-multistage/rustc-perf/database)
   Compiling collector v0.1.0 (/tmp/tmp-multistage/rustc-perf/collector)
    Finished dev [unoptimized + debuginfo] target(s) in 28.45s
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Stage `Build rustc (LLVM PGO)` starts
stage-build INFO: Executing `LLVM_PROFILE_DIR=/tmp/tmp-multistage/llvm-pgo/prof-%p /usr/bin/python3 /checkout/x.py build --target x86_64-unknown-linux-gnu --host x86_64-unknown-linux-gnu --stage 2 library/std --llvm-profile-generate`
    Finished dev [unoptimized] target(s) in 0.06s
Building stage0 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
    Finished release [optimized] target(s) in 1.00s
Uplifting stage1 library (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 library from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Build completed successfully in 0:30:15
stage-build INFO: Stage `Build rustc (LLVM PGO)` ended: OK (1815.67s)
stage-build INFO: Stage `Gather profiles (LLVM PGO)` starts
stage-build INFO: Running benchmarks with PGO instrumented LLVM
stage-build DEBUG: Changing working dir from `/checkout/obj` to `/checkout/obj`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib -Copt-level=3 /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build DEBUG: Changing working dir from `/checkout/obj` to `/tmp/tmp-multistage/rustc-perf`
stage-build INFO: Executing `RUST_LOG=collector=debug RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
[2023-01-15T16:23:34Z DEBUG collector] benchmark LICENSE.md - ignored
[2023-01-15T16:23:34Z DEBUG collector] benchmark README.md - ignored
[2023-01-15T16:23:34Z DEBUG collector] benchmark `regex-1.5.5`- registered
---
[2023-01-15T16:29:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:29:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T16:29:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnt42Nf#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (7/7)
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging LLVM PGO profiles to /tmp/tmp-multistage/llvm-pgo.profdata
stage-build INFO: Executing `/rustroot/bin/llvm-profdata merge -o /tmp/tmp-multistage/llvm-pgo.profdata /tmp/tmp-multistage/llvm-pgo`
stage-build INFO: LLVM PGO statistics
stage-build INFO: /tmp/tmp-multistage/llvm-pgo.profdata: 28.08 MiB
stage-build INFO: /tmp/tmp-multistage/llvm-pgo: 16.58 GiB
stage-build INFO: Profile file count: 1263
stage-build INFO: Deleting directory `/tmp/tmp-multistage/llvm-pgo`
stage-build INFO: Stage `Gather profiles (LLVM PGO)` ended: OK (418.73s)
stage-build INFO: Clearing LLVM build files
stage-build INFO: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm`
stage-build INFO: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/lld`
stage-build INFO: Stage `Build rustc (rustc PGO)` starts
stage-build INFO: Executing `/usr/bin/python3 /checkout/x.py build --target x86_64-unknown-linux-gnu --host x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate /tmp/tmp-multistage/rustc-pgo`
    Finished dev [unoptimized] target(s) in 0.06s
Building stage0 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.23s
Copying stage0 library from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
    Finished release [optimized] target(s) in 0.53s
Uplifting stage1 library (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 library from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Build completed successfully in 0:09:44
stage-build INFO: Stage `Build rustc (rustc PGO)` ended: OK (584.46s)
stage-build INFO: Stage `Gather profiles (rustc PGO)` starts
stage-build INFO: Running benchmarks with PGO instrumented rustc
stage-build DEBUG: Changing working dir from `/checkout/obj` to `/checkout/obj`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 LLVM_PROFILE_FILE=/tmp/tmp-multistage/rustc-pgo/default_%m_%p.profraw /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 LLVM_PROFILE_FILE=/tmp/tmp-multistage/rustc-pgo/default_%m_%p.profraw /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib -Copt-level=3 /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build DEBUG: Changing working dir from `/checkout/obj` to `/tmp/tmp-multistage/rustc-perf`
stage-build INFO: Executing `RUST_LOG=collector=debug RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc RUSTC_BOOTSTRAP=1 LLVM_PROFILE_FILE=/tmp/tmp-multistage/rustc-pgo/default_%m_%p.profraw /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Check,Debug,Opt --scenarios All --include externs,ctfe-stress-5,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0`
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Check,Debug,Opt --scenarios All --include externs,ctfe-stress-5,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0`
[2023-01-15T16:40:59Z DEBUG collector] benchmark LICENSE.md - ignored
[2023-01-15T16:40:59Z DEBUG collector] benchmark README.md - ignored
[2023-01-15T16:40:59Z DEBUG collector] benchmark `match-stress`- registered
---
[2023-01-15T16:41:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:41:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-15T16:41:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxhDMWs#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:41:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-15T16:41:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxhDMWs#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxhDMWs/incremental-state"
[2023-01-15T16:41:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:41:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxhDMWs#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxhDMWs/incremental-state"
[2023-01-15T16:41:05Z DEBUG collector::execute] applying println to "/tmp/.tmpxhDMWs"
[2023-01-15T16:41:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:41:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:41:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxhDMWs#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxhDMWs/incremental-state"
[2023-01-15T16:41:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:41:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-15T16:41:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBEWekP#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:41:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T16:41:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T16:41:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBEWekP#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBEWekP/incremental-state"
[2023-01-15T16:41:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:41:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBEWekP#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBEWekP/incremental-state"
[2023-01-15T16:41:11Z DEBUG collector::execute] applying println to "/tmp/.tmpBEWekP"
[2023-01-15T16:41:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:41:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:41:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBEWekP#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBEWekP/incremental-state"
[2023-01-15T16:41:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:41:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T16:41:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpstA2uj#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:41:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T16:41:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T16:41:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpstA2uj#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpstA2uj/incremental-state"
[2023-01-15T16:41:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:41:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpstA2uj#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpstA2uj/incremental-state"
[2023-01-15T16:41:18Z DEBUG collector::execute] applying println to "/tmp/.tmpstA2uj"
[2023-01-15T16:41:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:41:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:41:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpstA2uj#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpstA2uj/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-01-15T16:41:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-01-15T16:41:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2023-01-15T16:42:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:42:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-15T16:42:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjY9Hkv#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:42:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-15T16:42:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjY9Hkv#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjY9Hkv/incremental-state"
[2023-01-15T16:42:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:42:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjY9Hkv#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjY9Hkv/incremental-state"
[2023-01-15T16:43:00Z DEBUG collector::execute] applying println to "/tmp/.tmpjY9Hkv"
[2023-01-15T16:43:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:43:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:43:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjY9Hkv#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjY9Hkv/incremental-state"
[2023-01-15T16:43:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:43:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-15T16:43:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzJtqHv#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:43:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T16:43:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T16:43:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzJtqHv#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzJtqHv/incremental-state"
[2023-01-15T16:44:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:44:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzJtqHv#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzJtqHv/incremental-state"
[2023-01-15T16:45:03Z DEBUG collector::execute] applying println to "/tmp/.tmpzJtqHv"
[2023-01-15T16:45:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:45:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:45:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzJtqHv#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzJtqHv/incremental-state"
[2023-01-15T16:45:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:45:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T16:45:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEkz2GN#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:46:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T16:46:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T16:46:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEkz2GN#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEkz2GN/incremental-state"
[2023-01-15T16:47:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:47:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEkz2GN#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEkz2GN/incremental-state"
[2023-01-15T16:47:20Z DEBUG collector::execute] applying println to "/tmp/.tmpEkz2GN"
[2023-01-15T16:47:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:47:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:47:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEkz2GN#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpEkz2GN/incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2023-01-15T16:47:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-15T16:47:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-15T16:48:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:48:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T16:48:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQxJWjz#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:48:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T16:48:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQxJWjz#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQxJWjz/incremental-state"
[2023-01-15T16:48:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:48:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQxJWjz#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpQxJWjz/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2023-01-15T16:48:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-15T16:48:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-15T16:48:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:48:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-15T16:48:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjnxrKL#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:49:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-15T16:49:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjnxrKL#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjnxrKL/incremental-state"
[2023-01-15T16:49:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:49:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjnxrKL#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjnxrKL/incremental-state"
[2023-01-15T16:49:22Z DEBUG collector::execute] applying println to "/tmp/.tmpjnxrKL"
[2023-01-15T16:49:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:49:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:49:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjnxrKL#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpjnxrKL/incremental-state"
[2023-01-15T16:49:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:49:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-15T16:49:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdy2ulC#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:49:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T16:49:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T16:49:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdy2ulC#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdy2ulC/incremental-state"
[2023-01-15T16:50:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:50:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdy2ulC#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdy2ulC/incremental-state"
[2023-01-15T16:50:07Z DEBUG collector::execute] applying println to "/tmp/.tmpdy2ulC"
[2023-01-15T16:50:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:50:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:50:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdy2ulC#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdy2ulC/incremental-state"
[2023-01-15T16:50:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:50:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T16:50:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzHLjuf#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:50:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T16:50:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T16:50:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzHLjuf#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzHLjuf/incremental-state"
[2023-01-15T16:50:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:50:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzHLjuf#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzHLjuf/incremental-state"
[2023-01-15T16:50:54Z DEBUG collector::execute] applying println to "/tmp/.tmpzHLjuf"
[2023-01-15T16:50:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:50:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-15T16:50:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzHLjuf#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzHLjuf/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2023-01-15T16:50:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-15T16:50:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-01-15T16:51:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:51:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T16:51:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFWsCBF#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:51:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T16:51:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFWsCBF#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFWsCBF/incremental-state"
[2023-01-15T16:51:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:51:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFWsCBF#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFWsCBF/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-01-15T16:51:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-15T16:51:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-15T16:51:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:51:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-15T16:51:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDwWofM#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:51:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-15T16:51:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDwWofM#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDwWofM/incremental-state"
[2023-01-15T16:51:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:51:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDwWofM#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDwWofM/incremental-state"
[2023-01-15T16:51:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:51:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-15T16:51:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSYceeP#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:51:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T16:51:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-15T16:51:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSYceeP#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSYceeP/incremental-state"
[2023-01-15T16:51:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:51:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSYceeP#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSYceeP/incremental-state"
[2023-01-15T16:51:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:51:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T16:51:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj9aFxh#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:51:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-01-15T16:51:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:51:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T16:51:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfamLjA#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:51:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T16:51:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfamLjA#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfamLjA/incremental-state"
[2023-01-15T16:51:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:51:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfamLjA#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpfamLjA/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-01-15T16:51:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-15T16:51:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-01-15T16:52:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T16:52:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T16:52:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplanWPa#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-15T16:52:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-15T16:52:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplanWPa#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplanWPa/incremental-state"
[2023-01-15T16:52:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-15T16:52:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplanWPa#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplanWPa/incremental-state"
[2023-01-15T16:52:15Z DEBUG collector::execute] applying new row to "/tmp/.tmplanWPa"
[2023-01-15T16:52:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-15T16:52:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-15T16:52:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplanWPa#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmplanWPa/incremental-state"
Finished benchmark tuple-stress (8/8)
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging Rustc PGO profiles to /tmp/tmp-multistage/rustc-pgo.profdata
stage-build INFO: Executing `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-multistage/rustc-pgo.profdata /tmp/tmp-multistage/rustc-pgo`
stage-build INFO: Rustc PGO statistics
stage-build INFO: /tmp/tmp-multistage/rustc-pgo.profdata: 90.54 MiB
stage-build INFO: /tmp/tmp-multistage/rustc-pgo: 19.58 GiB
stage-build INFO: Profile file count: 1332
stage-build INFO: Deleting directory `/tmp/tmp-multistage/rustc-pgo`
stage-build INFO: Stage `Gather profiles (rustc PGO)` ended: OK (806.32s)
stage-build INFO: Clearing LLVM build files
stage-build INFO: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm`
stage-build INFO: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/lld`
stage-build INFO: Stage `Build rustc (LLVM BOLT)` starts
stage-build INFO: Executing `/usr/bin/python3 /checkout/x.py build --target x86_64-unknown-linux-gnu --host x86_64-unknown-linux-gnu --stage 2 library/std --llvm-profile-use /tmp/tmp-multistage/llvm-pgo.profdata --llvm-bolt-profile-generate`
    Finished dev [unoptimized] target(s) in 0.06s
Building stage0 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.23s
Copying stage0 library from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
    Finished release [optimized] target(s) in 7.32s
Uplifting stage1 library (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 library from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Build completed successfully in 0:27:42
stage-build INFO: Stage `Build rustc (LLVM BOLT)` ended: OK (1662.92s)
stage-build INFO: Stage `Gather profiles (LLVM BOLT)` starts
stage-build INFO: Running benchmarks with BOLT instrumented LLVM
stage-build DEBUG: Changing working dir from `/checkout/obj` to `/checkout/obj`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib -Copt-level=3 /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage`
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build DEBUG: Changing working dir from `/checkout/obj` to `/tmp/tmp-multistage/rustc-perf`
stage-build INFO: Executing `RUST_LOG=collector=debug RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Check,Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --profiles Check,Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
[2023-01-15T17:21:38Z DEBUG collector] benchmark LICENSE.md - ignored
[2023-01-15T17:21:38Z DEBUG collector] benchmark README.md - ignored
[2023-01-15T17:21:38Z DEBUG collector] benchmark `regex-1.5.5`- registered
---
[2023-01-15T17:30:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-15T17:30:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-15T17:30:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpM8zWXy#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (7/7)
stage-build DEBUG: Reverting working dir to `/checkout/obj`
stage-build INFO: Merging LLVM BOLT profiles to /tmp/tmp-multistage/bolt.profdata
stage-build INFO: Executing `merge-fdata /tmp/prof.fdata.151917.fdata /tmp/prof.fdata.151925.fdata /tmp/prof.fdata.151928.fdata /tmp/prof.fdata.151931.fdata /tmp/prof.fdata.151943.fdata /tmp/prof.fdata.152611.fdata /tmp/prof.fdata.152620.fdata /tmp/prof.fdata.152621.fdata /tmp/prof.fdata.152636.fdata /tmp/prof.fdata.152638.fdata /tmp/prof.fdata.152640.fdata /tmp/prof.fdata.152642.fdata /tmp/prof.fdata.152643.fdata /tmp/prof.fdata.152644.fdata /tmp/prof.fdata.152730.fdata /tmp/prof.fdata.152938.fdata /tmp/prof.fdata.152939.fdata /tmp/prof.fdata.152940.fdata /tmp/prof.fdata.152956.fdata /tmp/prof.fdata.152957.fdata /tmp/prof.fdata.152958.fdata /tmp/prof.fdata.152961.fdata /tmp/prof.fdata.152963.fdata /tmp/prof.fdata.152987.fdata /tmp/prof.fdata.153000.fdata /tmp/prof.fdata.153028.fdata /tmp/prof.fdata.153029.fdata /tmp/prof.fdata.153037.fdata /tmp/prof.fdata.153043.fdata /tmp/prof.fdata.153054.fdata /tmp/prof.fdata.153092.fdata /tmp/prof.fdata.153094.fdata /tmp/prof.fdata.153097.fdata /tmp/prof.fdata.153099.fdata /tmp/prof.fdata.153103.fdata /tmp/prof.fdata.153105.fdata /tmp/prof.fdata.153107.fdata /tmp/prof.fdata.153110.fdata /tmp/prof.fdata.153113.fdata /tmp/prof.fdata.153116.fdata /tmp/prof.fdata.153118.fdata /tmp/prof.fdata.153120.fdata /tmp/prof.fdata.153123.fdata /tmp/prof.fdata.153124.fdata /tmp/prof.fdata.153125.fdata /tmp/prof.fdata.153145.fdata /tmp/prof.fdata.153485.fdata /tmp/prof.fdata.153489.fdata /tmp/prof.fdata.153493.fdata /tmp/prof.fdata.153580.fdata /tmp/prof.fdata.153626.fdata /tmp/prof.fdata.153636.fdata /tmp/prof.fdata.153638.fdata /tmp/prof.fdata.153645.fdata /tmp/prof.fdata.153646.fdata /tmp/prof.fdata.153651.fdata /tmp/prof.fdata.153658.fdata /tmp/prof.fdata.153661.fdata /tmp/prof.fdata.153666.fdata /tmp/prof.fdata.153670.fdata /tmp/prof.fdata.153672.fdata /tmp/prof.fdata.153685.fdata /tmp/prof.fdata.153689.fdata /tmp/prof.fdata.153695.fdata /tmp/prof.fdata.153697.fdata /tmp/prof.fdata.153728.fdata /tmp/prof.fdata.153752.fdata /tmp/prof.fdata.153780.fdata /tmp/prof.fdata.153815.fdata /tmp/prof.fdata.153831.fdata /tmp/prof.fdata.153906.fdata /tmp/prof.fdata.153910.fdata /tmp/prof.fdata.153932.fdata /tmp/prof.fdata.153950.fdata /tmp/prof.fdata.153966.fdata /tmp/prof.fdata.153973.fdata /tmp/prof.fdata.153983.fdata /tmp/prof.fdata.153997.fdata /tmp/prof.fdata.154015.fdata /tmp/prof.fdata.154037.fdata /tmp/prof.fdata.154046.fdata /tmp/prof.fdata.154053.fdata /tmp/prof.fdata.154063.fdata /tmp/prof.fdata.154117.fdata /tmp/prof.fdata.154121.fdata /tmp/prof.fdata.154125.fdata /tmp/prof.fdata.154165.fdata /tmp/prof.fdata.154171.fdata /tmp/prof.fdata.154172.fdata /tmp/prof.fdata.154190.fdata /tmp/prof.fdata.154233.fdata /tmp/prof.fdata.154237.fdata /tmp/prof.fdata.154246.fdata /tmp/prof.fdata.154265.fdata /tmp/prof.fdata.154271.fdata /tmp/prof.fdata.154283.fdata /tmp/prof.fdata.154296.fdata /tmp/prof.fdata.154312.fdata /tmp/prof.fdata.154324.fdata /tmp/prof.fdata.154334.fdata /tmp/prof.fdata.154404.fdata /tmp/prof.fdata.154412.fdata /tmp/prof.fdata.154428.fdata /tmp/prof.fdata.154442.fdata /tmp/prof.fdata.154449.fdata /tmp/prof.fdata.154456.fdata /tmp/prof.fdata.154498.fdata /tmp/prof.fdata.154508.fdata /tmp/prof.fdata.154518.fdata /tmp/prof.fdata.154534.fdata /tmp/prof.fdata.154543.fdata /tmp/prof.fdata.154570.fdata /tmp/prof.fdata.154574.fdata /tmp/prof.fdata.154578.fdata /tmp/prof.fdata.154595.fdata /tmp/prof.fdata.154603.fdata /tmp/prof.fdata.154629.fdata /tmp/prof.fdata.154645.fdata /tmp/prof.fdata.154681.fdata /tmp/prof.fdata.154688.fdata /tmp/prof.fdata.154695.fdata /tmp/prof.fdata.154710.fdata /tmp/prof.fdata.154712.fdata /tmp/prof.fdata.154717.fdata /tmp/prof.fdata.154724.fdata /tmp/prof.fdata.154733.fdata /tmp/prof.fdata.154767.fdata /tmp/prof.fdata.154775.fdata /tmp/prof.fdata.154781.fdata /tmp/prof.fdata.154787.fdata /tmp/prof.fdata.154804.fdata /tmp/prof.fdata.154816.fdata /tmp/prof.fdata.154883.fdata /tmp/prof.fdata.154907.fdata /tmp/prof.fdata.154912.fdata /tmp/prof.fdata.154935.fdata /tmp/prof.fdata.154954.fdata /tmp/prof.fdata.154956.fdata /tmp/prof.fdata.154985.fdata /tmp/prof.fdata.154988.fdata /tmp/prof.fdata.155003.fdata /tmp/prof.fdata.155017.fdata /tmp/prof.fdata.155049.fdata /tmp/prof.fdata.155119.fdata /tmp/prof.fdata.155146.fdata /tmp/prof.fdata.155183.fdata /tmp/prof.fdata.155185.fdata /tmp/prof.fdata.155237.fdata /tmp/prof.fdata.155299.fdata /tmp/prof.fdata.155318.fdata /tmp/prof.fdata.155323.fdata /tmp/prof.fdata.155326.fdata /tmp/prof.fdata.155344.fdata /tmp/prof.fdata.155386.fdata /tmp/prof.fdata.155395.fdata /tmp/prof.fdata.155413.fdata /tmp/prof.fdata.155419.fdata /tmp/prof.fdata.155423.fdata /tmp/prof.fdata.155455.fdata /tmp/prof.fdata.155464.fdata /tmp/prof.fdata.155474.fdata /tmp/prof.fdata.155478.fdata /tmp/prof.fdata.155486.fdata /tmp/prof.fdata.155496.fdata /tmp/prof.fdata.155518.fdata /tmp/prof.fdata.155521.fdata /tmp/prof.fdata.155542.fdata /tmp/prof.fdata.155553.fdata /tmp/prof.fdata.155563.fdata /tmp/prof.fdata.155572.fdata /tmp/prof.fdata.155587.fdata /tmp/prof.fdata.155598.fdata /tmp/prof.fdata.155616.fdata /tmp/prof.fdata.155632.fdata /tmp/prof.fdata.155663.fdata /tmp/prof.fdata.155669.fdata /tmp/prof.fdata.155676.fdata /tmp/prof.fdata.155711.fdata /tmp/prof.fdata.155734.fdata /tmp/prof.fdata.155741.fdata /tmp/prof.fdata.155754.fdata /tmp/prof.fdata.155790.fdata /tmp/prof.fdata.155800.fdata /tmp/prof.fdata.155806.fdata /tmp/prof.fdata.155812.fdata /tmp/prof.fdata.155826.fdata /tmp/prof.fdata.155835.fdata /tmp/prof.fdata.155910.fdata /tmp/prof.fdata.155917.fdata /tmp/prof.fdata.155958.fdata /tmp/prof.fdata.155989.fdata /tmp/prof.fdata.156020.fdata /tmp/prof.fdata.156027.fdata /tmp/prof.fdata.156032.fdata /tmp/prof.fdata.156044.fdata /tmp/prof.fdata.156062.fdata /tmp/prof.fdata.156078.fdata /tmp/prof.fdata.156088.fdata /tmp/prof.fdata.156113.fdata /tmp/prof.fdata.156119.fdata /tmp/prof.fdata.156139.fdata /tmp/prof.fdata.156150.fdata /tmp/prof.fdata.156180.fdata /tmp/prof.fdata.156213.fdata /tmp/prof.fdata.156218.fdata /tmp/prof.fdata.156235.fdata /tmp/prof.fdata.156270.fdata /tmp/prof.fdata.156291.fdata /tmp/prof.fdata.156300.fdata /tmp/prof.fdata.156311.fdata /tmp/prof.fdata.156324.fdata /tmp/prof.fdata.156334.fdata /tmp/prof.fdata.156342.fdata /tmp/prof.fdata.156369.fdata /tmp/prof.fdata.156377.fdata /tmp/prof.fdata.156387.fdata /tmp/prof.fdata.156440.fdata /tmp/prof.fdata.156442.fdata /tmp/prof.fdata.156472.fdata /tmp/prof.fdata.156506.fdata /tmp/prof.fdata.156545.fdata /tmp/prof.fdata.156555.fdata /tmp/prof.fdata.156564.fdata /tmp/prof.fdata.156624.fdata /tmp/prof.fdata.156638.fdata /tmp/prof.fdata.156703.fdata /tmp/prof.fdata.156711.fdata /tmp/prof.fdata.156774.fdata /tmp/prof.fdata.156820.fdata /tmp/prof.fdata.156821.fdata /tmp/prof.fdata.156827.fdata /tmp/prof.fdata.156830.fdata /tmp/prof.fdata.156837.fdata /tmp/prof.fdata.156849.fdata /tmp/prof.fdata.156852.fdata /tmp/prof.fdata.156863.fdata /tmp/prof.fdata.156879.fdata /tmp/prof.fdata.156889.fdata /tmp/prof.fdata.156894.fdata /tmp/prof.fdata.156938.fdata /tmp/prof.fdata.156958.fdata /tmp/prof.fdata.156963.fdata /tmp/prof.fdata.156979.fdata /tmp/prof.fdata.156992.fdata /tmp/prof.fdata.157003.fdata /tmp/prof.fdata.157059.fdata /tmp/prof.fdata.157070.fdata /tmp/prof.fdata.157091.fdata /tmp/prof.fdata.157099.fdata /tmp/prof.fdata.157124.fdata /tmp/prof.fdata.157132.fdata /tmp/prof.fdata.157137.fdata /tmp/prof.fdata.157169.fdata /tmp/prof.fdata.157194.fdata /tmp/prof.fdata.157204.fdata /tmp/prof.fdata.157213.fdata /tmp/prof.fdata.157231.fdata /tmp/prof.fdata.157274.fdata /tmp/prof.fdata.157291.fdata /tmp/prof.fdata.157310.fdata /tmp/prof.fdata.157313.fdata /tmp/prof.fdata.157321.fdata /tmp/prof.fdata.157362.fdata /tmp/prof.fdata.157375.fdata /tmp/prof.fdata.157384.fdata /tmp/prof.fdata.157401.fdata /tmp/prof.fdata.157408.fdata /tmp/prof.fdata.157430.fdata /tmp/prof.fdata.157439.fdata /tmp/prof.fdata.157451.fdata /tmp/prof.fdata.157468.fdata /tmp/prof.fdata.157477.fdata /tmp/prof.fdata.157488.fdata /tmp/prof.fdata.157505.fdata /tmp/prof.fdata.157512.fdata /tmp/prof.fdata.157538.fdata /tmp/prof.fdata.157603.fdata /tmp/prof.fdata.157621.fdata /tmp/prof.fdata.157645.fdata /tmp/prof.fdata.157718.fdata /tmp/prof.fdata.157758.fdata /tmp/prof.fdata.157763.fdata /tmp/prof.fdata.157795.fdata /tmp/prof.fdata.157802.fdata /tmp/prof.fdata.157821.fdata /tmp/prof.fdata.157830.fdata /tmp/prof.fdata.157850.fdata /tmp/prof.fdata.157906.fdata /tmp/prof.fdata.157921.fdata /tmp/prof.fdata.157925.fdata /tmp/prof.fdata.157931.fdata /tmp/prof.fdata.157939.fdata /tmp/prof.fdata.157963.fdata /tmp/prof.fdata.157975.fdata /tmp/prof.fdata.157985.fdata /tmp/prof.fdata.158045.fdata /tmp/prof.fdata.158047.fdata /tmp/prof.fdata.158060.fdata /tmp/prof.fdata.158068.fdata /tmp/prof.fdata.158141.fdata /tmp/prof.fdata.158158.fdata /tmp/prof.fdata.158186.fdata /tmp/prof.fdata.158191.fdata /tmp/prof.fdata.158220.fdata /tmp/prof.fdata.158229.fdata /tmp/prof.fdata.158261.fdata /tmp/prof.fdata.158314.fdata /tmp/prof.fdata.158318.fdata /tmp/prof.fdata.158330.fdata /tmp/prof.fdata.158355.fdata /tmp/prof.fdata.158400.fdata /tmp/prof.fdata.158439.fdata /tmp/prof.fdata.158459.fdata /tmp/prof.fdata.158472.fdata /tmp/prof.fdata.158477.fdata /tmp/prof.fdata.158504.fdata /tmp/prof.fdata.159203.fdata /tmp/prof.fdata.160806.fdata /tmp/prof.fdata.161447.fdata /tmp/prof.fdata.161481.fdata /tmp/prof.fdata.162416.fdata /tmp/prof.fdata.162482.fdata /tmp/prof.fdata.164677.fdata /tmp/prof.fdata.164684.fdata /tmp/prof.fdata.164717.fdata /tmp/prof.fdata.164775.fdata /tmp/prof.fdata.164795.fdata /tmp/prof.fdata.164847.fdata /tmp/prof.fdata.164848.fdata /tmp/prof.fdata.164849.fdata /tmp/prof.fdata.164850.fdata /tmp/prof.fdata.164851.fdata /tmp/prof.fdata.164852.fdata /tmp/prof.fdata.164856.fdata /tmp/prof.fdata.164857.fdata /tmp/prof.fdata.164858.fdata /tmp/prof.fdata.164862.fdata /tmp/prof.fdata.164863.fdata /tmp/prof.fdata.164864.fdata /tmp/prof.fdata.164872.fdata /tmp/prof.fdata.164873.fdata /tmp/prof.fdata.164874.fdata /tmp/prof.fdata.164876.fdata /tmp/prof.fdata.164879.fdata /tmp/prof.fdata.164881.fdata /tmp/prof.fdata.164882.fdata /tmp/prof.fdata.164884.fdata /tmp/prof.fdata.164887.fdata /tmp/prof.fdata.164888.fdata /tmp/prof.fdata.164891.fdata /tmp/prof.fdata.164893.fdata /tmp/prof.fdata.164894.fdata /tmp/prof.fdata.164897.fdata /tmp/prof.fdata.164901.fdata /tmp/prof.fdata.164902.fdata /tmp/prof.fdata.164903.fdata /tmp/prof.fdata.164904.fdata /tmp/prof.fdata.164916.fdata /tmp/prof.fdata.165207.fdata /tmp/prof.fdata.165210.fdata /tmp/prof.fdata.165214.fdata /tmp/prof.fdata.165216.fdata /tmp/prof.fdata.165298.fdata /tmp/prof.fdata.165308.fdata /tmp/prof.fdata.165312.fdata /tmp/prof.fdata.165317.fdata /tmp/prof.fdata.165324.fdata /tmp/prof.fdata.165327.fdata /tmp/prof.fdata.165335.fdata /tmp/prof.fdata.165342.fdata /tmp/prof.fdata.165358.fdata /tmp/prof.fdata.165365.fdata /tmp/prof.fdata.165373.fdata /tmp/prof.fdata.165375.fdata /tmp/prof.fdata.165379.fdata /tmp/prof.fdata.165389.fdata /tmp/prof.fdata.165397.fdata /tmp/prof.fdata.165401.fdata /tmp/prof.fdata.165406.fdata /tmp/prof.fdata.165435.fdata /tmp/prof.fdata.165456.fdata /tmp/prof.fdata.165465.fdata /tmp/prof.fdata.165466.fdata /tmp/prof.fdata.165473.fdata /tmp/prof.fdata.165480.fdata /tmp/prof.fdata.165487.fdata /tmp/prof.fdata.165499.fdata /tmp/prof.fdata.165531.fdata /tmp/prof.fdata.165617.fdata /tmp/prof.fdata.165630.fdata /tmp/prof.fdata.165637.fdata /tmp/prof.fdata.165664.fdata /tmp/prof.fdata.165665.fdata /tmp/prof.fdata.165689.fdata /tmp/prof.fdata.165702.fdata /tmp/prof.fdata.165706.fdata /tmp/prof.fdata.165713.fdata /tmp/prof.fdata.165730.fdata /tmp/prof.fdata.165734.fdata /tmp/prof.fdata.165785.fdata /tmp/prof.fdata.165820.fdata /tmp/prof.fdata.165829.fdata /tmp/prof.fdata.165833.fdata /tmp/prof.fdata.165880.fdata /tmp/prof.fdata.165889.fdata /tmp/prof.fdata.165900.fdata /tmp/prof.fdata.165923.fdata /tmp/prof.fdata.165952.fdata /tmp/prof.fdata.165960.fdata /tmp/prof.fdata.165970.fdata /tmp/prof.fdata.165975.fdata /tmp/prof.fdata.166016.fdata /tmp/prof.fdata.166035.fdata /tmp/prof.fdata.166052.fdata /tmp/prof.fdata.166068.fdata /tmp/prof.fdata.166090.fdata /tmp/prof.fdata.166100.fdata /tmp/prof.fdata.166111.fdata /tmp/prof.fdata.166140.fdata /tmp/prof.fdata.166149.fdata /tmp/prof.fdata.166161.fdata /tmp/prof.fdata.166190.fdata /tmp/prof.fdata.166250.fdata /tmp/prof.fdata.166259.fdata /tmp/prof.fdata.166266.fdata /tmp/prof.fdata.166280.fdata /tmp/prof.fdata.166281.fdata /tmp/prof.fdata.166307.fdata /tmp/prof.fdata.166366.fdata /tmp/prof.fdata.166418.fdata /tmp/prof.fdata.166420.fdata /tmp/prof.fdata.166431.fdata /tmp/prof.fdata.166444.fdata /tmp/prof.fdata.166477.fdata /tmp/prof.fdata.166484.fdata /tmp/prof.fdata.166499.fdata /tmp/prof.fdata.166500.fdata /tmp/prof.fdata.166514.fdata /tmp/prof.fdata.166520.fdata /tmp/prof.fdata.166523.fdata /tmp/prof.fdata.166525.fdata /tmp/prof.fdata.166538.fdata /tmp/prof.fdata.166542.fdata /tmp/prof.fdata.166550.fdata /tmp/prof.fdata.166567.fdata /tmp/prof.fdata.166570.fdata /tmp/prof.fdata.166575.fdata /tmp/prof.fdata.166594.fdata /tmp/prof.fdata.166598.fdata /tmp/prof.fdata.166605.fdata /tmp/prof.fdata.166608.fdata /tmp/prof.fdata.166616.fdata /tmp/prof.fdata.166666.fdata /tmp/prof.fdata.166691.fdata /tmp/prof.fdata.166699.fdata /tmp/prof.fdata.166705.fdata /tmp/prof.fdata.166712.fdata /tmp/prof.fdata.166744.fdata /tmp/prof.fdata.166832.fdata /tmp/prof.fdata.166861.fdata /tmp/prof.fdata.166864.fdata /tmp/prof.fdata.166883.fdata /tmp/prof.fdata.166887.fdata /tmp/prof.fdata.166944.fdata /tmp/prof.fdata.166950.fdata /tmp/prof.fdata.166960.fdata /tmp/prof.fdata.166969.fdata /tmp/prof.fdata.166975.fdata /tmp/prof.fdata.166979.fdata /tmp/prof.fdata.167009.fdata /tmp/prof.fdata.167022.fdata /tmp/prof.fdata.167030.fdata /tmp/prof.fdata.167032.fdata /tmp/prof.fdata.167054.fdata /tmp/prof.fdata.167056.fdata /tmp/prof.fdata.167061.fdata /tmp/prof.fdata.167065.fdata /tmp/prof.fdata.167124.fdata /tmp/prof.fdata.167138.fdata /tmp/prof.fdata.167172.fdata /tmp/prof.fdata.167186.fdata /tmp/prof.fdata.167197.fdata /tmp/prof.fdata.167299.fdata /tmp/prof.fdata.167305.fdata /tmp/prof.fdata.167313.fdata /tmp/prof.fdata.167343.fdata /tmp/prof.fdata.167347.fdata /tmp/prof.fdata.167354.fdata /tmp/prof.fdata.167357.fdata /tmp/prof.fdata.167379.fdata /tmp/prof.fdata.167408.fdata /tmp/prof.fdata.167430.fdata /tmp/prof.fdata.167438.fdata /tmp/prof.fdata.167445.fdata /tmp/prof.fdata.167471.fdata /tmp/prof.fdata.167474.fdata /tmp/prof.fdata.167479.fdata /tmp/prof.fdata.167528.fdata /tmp/prof.fdata.167529.fdata /tmp/prof.fdata.167540.fdata /tmp/prof.fdata.167553.fdata /tmp/prof.fdata.167602.fdata /tmp/prof.fdata.167611.fdata /tmp/prof.fdata.167665.fdata /tmp/prof.fdata.167683.fdata /tmp/prof.fdata.167709.fdata /tmp/prof.fdata.167733.fdata /tmp/prof.fdata.167789.fdata /tmp/prof.fdata.167799.fdata /tmp/prof.fdata.167828.fdata /tmp/prof.fdata.167841.fdata /tmp/prof.fdata.167848.fdata /tmp/prof.fdata.167854.fdata /tmp/prof.fdata.167867.fdata /tmp/prof.fdata.167903.fdata /tmp/prof.fdata.167916.fdata /tmp/prof.fdata.167939.fdata /tmp/prof.fdata.167946.fdata /tmp/prof.fdata.167962.fdata /tmp/prof.fdata.167974.fdata /tmp/prof.fdata.167979.fdata /tmp/prof.fdata.167987.fdata /tmp/prof.fdata.168019.fdata /tmp/prof.fdata.168024.fdata /tmp/prof.fdata.168052.fdata /tmp/prof.fdata.168073.fdata /tmp/prof.fdata.168083.fdata /tmp/prof.fdata.168085.fdata /tmp/prof.fdata.168093.fdata /tmp/prof.fdata.168108.fdata /tmp/prof.fdata.168123.fdata /tmp/prof.fdata.168136.fdata /tmp/prof.fdata.168158.fdata /tmp/prof.fdata.168167.fdata /tmp/prof.fdata.168173.fdata /tmp/prof.fdata.168179.fdata /tmp/prof.fdata.168184.fdata /tmp/prof.fdata.168189.fdata /tmp/prof.fdata.168190.fdata /tmp/prof.fdata.168233.fdata /tmp/prof.fdata.168280.fdata /tmp/prof.fdata.168289.fdata /tmp/prof.fdata.168293.fdata /tmp/prof.fdata.168311.fdata /tmp/prof.fdata.168326.fdata /tmp/prof.fdata.168330.fdata /tmp/prof.fdata.168335.fdata /tmp/prof.fdata.168347.fdata /tmp/prof.fdata.168360.fdata /tmp/prof.fdata.168391.fdata /tmp/prof.fdata.168411.fdata /tmp/prof.fdata.168439.fdata /tmp/prof.fdata.168471.fdata /tmp/prof.fdata.168498.fdata /tmp/prof.fdata.168511.fdata /tmp/prof.fdata.168514.fdata /tmp/prof.fdata.168520.fdata /tmp/prof.fdata.168535.fdata /tmp/prof.fdata.168537.fdata /tmp/prof.fdata.168550.fdata /tmp/prof.fdata.168577.fdata /tmp/prof.fdata.168582.fdata /tmp/prof.fdata.168638.fdata /tmp/prof.fdata.168660.fdata /tmp/prof.fdata.168684.fdata /tmp/prof.fdata.168706.fdata /tmp/prof.fdata.168711.fdata /tmp/prof.fdata.168715.fdata /tmp/prof.fdata.168759.fdata /tmp/prof.fdata.168769.fdata /tmp/prof.fdata.168774.fdata /tmp/prof.fdata.168775.fdata /tmp/prof.fdata.168784.fdata /tmp/prof.fdata.168791.fdata /tmp/prof.fdata.168795.fdata /tmp/prof.fdata.168832.fdata /tmp/prof.fdata.168850.fdata /tmp/prof.fdata.168888.fdata /tmp/prof.fdata.168900.fdata /tmp/prof.fdata.168917.fdata /tmp/prof.fdata.168919.fdata /tmp/prof.fdata.168925.fdata /tmp/prof.fdata.168929.fdata /tmp/prof.fdata.168942.fdata /tmp/prof.fdata.168943.fdata /tmp/prof.fdata.168947.fdata /tmp/prof.fdata.168948.fdata /tmp/prof.fdata.168998.fdata /tmp/prof.fdata.169013.fdata /tmp/prof.fdata.169016.fdata /tmp/prof.fdata.169043.fdata /tmp/prof.fdata.169086.fdata /tmp/prof.fdata.169097.fdata /tmp/prof.fdata.169101.fdata /tmp/prof.fdata.169105.fdata /tmp/prof.fdata.169111.fdata /tmp/prof.fdata.169117.fdata /tmp/prof.fdata.169135.fdata /tmp/prof.fdata.169136.fdata /tmp/prof.fdata.169151.fdata /tmp/prof.fdata.169177.fdata /tmp/prof.fdata.169188.fdata /tmp/prof.fdata.169195.fdata /tmp/prof.fdata.169198.fdata /tmp/prof.fdata.169203.fdata /tmp/prof.fdata.169216.fdata /tmp/prof.fdata.169223.fdata /tmp/prof.fdata.169226.fdata /tmp/prof.fdata.169239.fdata /tmp/prof.fdata.169242.fdata /tmp/prof.fdata.169322.fdata /tmp/prof.fdata.169326.fdata /tmp/prof.fdata.169331.fdata /tmp/prof.fdata.169334.fdata /tmp/prof.fdata.169337.fdata /tmp/prof.fdata.169344.fdata /tmp/prof.fdata.169387.fdata /tmp/prof.fdata.169409.fdata /tmp/prof.fdata.169411.fdata /tmp/prof.fdata.169442.fdata /tmp/prof.fdata.169453.fdata /tmp/prof.fdata.169461.fdata /tmp/prof.fdata.169465.fdata /tmp/prof.fdata.169482.fdata /tmp/prof.fdata.169485.fdata /tmp/prof.fdata.169501.fdata /tmp/prof.fdata.169508.fdata /tmp/prof.fdata.169551.fdata /tmp/prof.fdata.169565.fdata /tmp/prof.fdata.169620.fdata /tmp/prof.fdata.169629.fdata /tmp/prof.fdata.169633.fdata /tmp/prof.fdata.169641.fdata /tmp/prof.fdata.169645.fdata /tmp/prof.fdata.169651.fdata /tmp/prof.fdata.169683.fdata /tmp/prof.fdata.169693.fdata /tmp/prof.fdata.169708.fdata /tmp/prof.fdata.169716.fdata /tmp/prof.fdata.169736.fdata /tmp/prof.fdata.169745.fdata /tmp/prof.fdata.169750.fdata /tmp/prof.fdata.169760.fdata /tmp/prof.fdata.169773.fdata /tmp/prof.fdata.169774.fdata /tmp/prof.fdata.169776.fdata /tmp/prof.fdata.169799.fdata /tmp/prof.fdata.169806.fdata /tmp/prof.fdata.169808.fdata /tmp/prof.fdata.169826.fdata /tmp/prof.fdata.169835.fdata /tmp/prof.fdata.169849.fdata /tmp/prof.fdata.169868.fdata /tmp/prof.fdata.169902.fdata /tmp/prof.fdata.169904.fdata /tmp/prof.fdata.169906.fdata /tmp/prof.fdata.169912.fdata /tmp/prof.fdata.169916.fdata /tmp/prof.fdata.169971.fdata /tmp/prof.fdata.169975.fdata /tmp/prof.fdata.169980.fdata /tmp/prof.fdata.169984.fdata /tmp/prof.fdata.169993.fdata /tmp/prof.fdata.170002.fdata /tmp/prof.fdata.170013.fdata /tmp/prof.fdata.170026.fdata /tmp/prof.fdata.170032.fdata /tmp/prof.fdata.170035.fdata /tmp/prof.fdata.170047.fdata /tmp/prof.fdata.170062.fdata /tmp/prof.fdata.170065.fdata /tmp/prof.fdata.170068.fdata /tmp/prof.fdata.170073.fdata /tmp/prof.fdata.170076.fdata /tmp/prof.fdata.170078.fdata /tmp/prof.fdata.170087.fdata /tmp/prof.fdata.170091.fdata /tmp/prof.fdata.170127.fdata /tmp/prof.fdata.170136.fdata /tmp/prof.fdata.170137.fdata /tmp/prof.fdata.170138.fdata /tmp/prof.fdata.170144.fdata /tmp/prof.fdata.170163.fdata /tmp/prof.fdata.170173.fdata /tmp/prof.fdata.170192.fdata /tmp/prof.fdata.170196.fdata /tmp/prof.fdata.170207.fdata /tmp/prof.fdata.170211.fdata /tmp/prof.fdata.170228.fdata /tmp/prof.fdata.170237.fdata /tmp/prof.fdata.170246.fdata /tmp/prof.fdata.170247.fdata /tmp/prof.fdata.170255.fdata /tmp/prof.fdata.170269.fdata /tmp/prof.fdata.170270.fdata /tmp/prof.fdata.170335.fdata /tmp/prof.fdata.170373.fdata /tmp/prof.fdata.170381.fdata /tmp/prof.fdata.170390.fdata /tmp/prof.fdata.170460.fdata /tmp/prof.fdata.170473.fdata /tmp/prof.fdata.170502.fdata /tmp/prof.fdata.170521.fdata /tmp/prof.fdata.170536.fdata /tmp/prof.fdata.170550.fdata /tmp/prof.fdata.170606.fdata /tmp/prof.fdata.170616.fdata /tmp/prof.fdata.170627.fdata /tmp/prof.fdata.170633.fdata /tmp/prof.fdata.170644.fdata /tmp/prof.fdata.170648.fdata /tmp/prof.fdata.170651.fdata /tmp/prof.fdata.170690.fdata /tmp/prof.fdata.170700.fdata /tmp/prof.fdata.170722.fdata /tmp/prof.fdata.170730.fdata /tmp/prof.fdata.170740.fdata /tmp/prof.fdata.170754.fdata /tmp/prof.fdata.170756.fdata /tmp/prof.fdata.170763.fdata /tmp/prof.fdata.170803.fdata /tmp/prof.fdata.170809.fdata /tmp/prof.fdata.170830.fdata /tmp/prof.fdata.170847.fdata /tmp/prof.fdata.170858.fdata /tmp/prof.fdata.170874.fdata /tmp/prof.fdata.170885.fdata /tmp/prof.fdata.170921.fdata /tmp/prof.fdata.170945.fdata /tmp/prof.fdata.170966.fdata /tmp/prof.fdata.171005.fdata /tmp/prof.fdata.171009.fdata /tmp/prof.fdata.171065.fdata /tmp/prof.fdata.171067.fdata /tmp/prof.fdata.171084.fdata /tmp/prof.fdata.171173.fdata /tmp/prof.fdata.171174.fdata /tmp/prof.fdata.171220.fdata /tmp/prof.fdata.171237.fdata /tmp/prof.fdata.171243.fdata /tmp/prof.fdata.171287.fdata /tmp/prof.fdata.171291.fdata /tmp/prof.fdata.171304.fdata /tmp/prof.fdata.171354.fdata /tmp/prof.fdata.171362.fdata /tmp/prof.fdata.171414.fdata /tmp/prof.fdata.171425.fdata /tmp/prof.fdata.171450.fdata /tmp/prof.fdata.171457.fdata /tmp/prof.fdata.171463.fdata /tmp/prof.fdata.171473.fdata /tmp/prof.fdata.171486.fdata /tmp/prof.fdata.171519.fdata /tmp/prof.fdata.171564.fdata /tmp/prof.fdata.171604.fdata /tmp/prof.fdata.171658.fdata /tmp/prof.fdata.171717.fdata /tmp/prof.fdata.171745.fdata /tmp/prof.fdata.171791.fdata /tmp/prof.fdata.171793.fdata /tmp/prof.fdata.171815.fdata /tmp/prof.fdata.171907.fdata /tmp/prof.fdata.171976.fdata /tmp/prof.fdata.172011.fdata /tmp/prof.fdata.172028.fdata /tmp/prof.fdata.172050.fdata /tmp/prof.fdata.172210.fdata /tmp/prof.fdata.172237.fdata /tmp/prof.fdata.172396.fdata /tmp/prof.fdata.172465.fdata /tmp/prof.fdata.172482.fdata /tmp/prof.fdata.172509.fdata /tmp/prof.fdata.173924.fdata /tmp/prof.fdata.173972.fdata /tmp/prof.fdata.173995.fdata /tmp/prof.fdata.174060.fdata /tmp/prof.fdata.174126.fdata /tmp/prof.fdata.174138.fdata /tmp/prof.fdata.174192.fdata /tmp/prof.fdata.174249.fdata /tmp/prof.fdata.174448.fdata /tmp/prof.fdata.175037.fdata /tmp/prof.fdata.175044.fdata /tmp/prof.fdata.175050.fdata /tmp/prof.fdata.175070.fdata /tmp/prof.fdata.175080.fdata /tmp/prof.fdata.175084.fdata /tmp/prof.fdata.175105.fdata /tmp/prof.fdata.175147.fdata /tmp/prof.fdata.175155.fdata /tmp/prof.fdata.175233.fdata /tmp/prof.fdata.175243.fdata /tmp/prof.fdata.175269.fdata /tmp/prof.fdata.175317.fdata /tmp/prof.fdata.175318.fdata /tmp/prof.fdata.175319.fdata /tmp/prof.fdata.175320.fdata /tmp/prof.fdata.175321.fdata /tmp/prof.fdata.175322.fdata /tmp/prof.fdata.175326.fdata /tmp/prof.fdata.175327.fdata /tmp/prof.fdata.175328.fdata /tmp/prof.fdata.175332.fdata /tmp/prof.fdata.175333.fdata /tmp/prof.fdata.175334.fdata /tmp/prof.fdata.175340.fdata /tmp/prof.fdata.175346.fdata /tmp/prof.fdata.175349.fdata /tmp/prof.fdata.175351.fdata /tmp/prof.fdata.175354.fdata /tmp/prof.fdata.175356.fdata /tmp/prof.fdata.175357.fdata /tmp/prof.fdata.175362.fdata /tmp/prof.fdata.175363.fdata /tmp/prof.fdata.175364.fdata /tmp/prof.fdata.175365.fdata /tmp/prof.fdata.175366.fdata /tmp/prof.fdata.175368.fdata /tmp/prof.fdata.175369.fdata /tmp/prof.fdata.175370.fdata /tmp/prof.fdata.175372.fdata /tmp/prof.fdata.175373.fdata /tmp/prof.fdata.175374.fdata /tmp/prof.fdata.175382.fdata /tmp/prof.fdata.175590.fdata /tmp/prof.fdata.175642.fdata /tmp/prof.fdata.175651.fdata /tmp/prof.fdata.175660.fdata /tmp/prof.fdata.175661.fdata /tmp/prof.fdata.175782.fdata /tmp/prof.fdata.175786.fdata /tmp/prof.fdata.175790.fdata /tmp/prof.fdata.175795.fdata /tmp/prof.fdata.175798.fdata /tmp/prof.fdata.175805.fdata /tmp/prof.fdata.175806.fdata /tmp/prof.fdata.175818.fdata /tmp/prof.fdata.175822.fdata /tmp/prof.fdata.175853.fdata /tmp/prof.fdata.175862.fdata /tmp/prof.fdata.175874.fdata /tmp/prof.fdata.175895.fdata /tmp/prof.fdata.175912.fdata /tmp/prof.fdata.175921.fdata /tmp/prof.fdata.175960.fdata /tmp/prof.fdata.176000.fdata /tmp/prof.fdata.176015.fdata /tmp/prof.fdata.176022.fdata /tmp/prof.fdata.176043.fdata /tmp/prof.fdata.176048.fdata /tmp/prof.fdata.176078.fdata /tmp/prof.fdata.176088.fdata /tmp/prof.fdata.176114.fdata /tmp/prof.fdata.176162.fdata /tmp/prof.fdata.176163.fdata /tmp/prof.fdata.176164.fdata /tmp/prof.fdata.176165.fdata /tmp/prof.fdata.176166.fdata /tmp/prof.fdata.176167.fdata /tmp/prof.fdata.176171.fdata /tmp/prof.fdata.176172.fdata /tmp/prof.fdata.176173.fdata /tmp/prof.fdata.176177.fdata /tmp/prof.fdata.176178.fdata /tmp/prof.fdata.176179.fdata /tmp/prof.fdata.176186.fdata /tmp/prof.fdata.176189.fdata /tmp/prof.fdata.176191.fdata /tmp/prof.fdata.176192.fdata /tmp/prof.fdata.176194.fdata /tmp/prof.fdata.176198.fdata /tmp/prof.fdata.176199.fdata /tmp/prof.fdata.176200.fdata /tmp/prof.fdata.176202.fdata /tmp/prof.fdata.176205.fdata /tmp/prof.fdata.176208.fdata /tmp/prof.fdata.176210.fdata /tmp/prof.fdata.176212.fdata /tmp/prof.fdata.176213.fdata /tmp/prof.fdata.176214.fdata /tmp/prof.fdata.176215.fdata /tmp/prof.fdata.176218.fdata /tmp/prof.fdata.176219.fdata /tmp/prof.fdata.176220.fdata /tmp/prof.fdata.176496.fdata /tmp/prof.fdata.176498.fdata /tmp/prof.fdata.176500.fdata /tmp/prof.fdata.176505.fdata /tmp/prof.fdata.176510.fdata /tmp/prof.fdata.176559.fdata /tmp/prof.fdata.176565.fdata /tmp/prof.fdata.176570.fdata /tmp/prof.fdata.176576.fdata /tmp/prof.fdata.176588.fdata /tmp/prof.fdata.176593.fdata /tmp/prof.fdata.176607.fdata /tmp/prof.fdata.176609.fdata /tmp/prof.fdata.176613.fdata /tmp/prof.fdata.176618.fdata /tmp/prof.fdata.176620.fdata /tmp/prof.fdata.176624.fdata /tmp/prof.fdata.176626.fdata /tmp/prof.fdata.176630.fdata /tmp/prof.fdata.176633.fdata /tmp/prof.fdata.176643.fdata /tmp/prof.fdata.176654.fdata /tmp/prof.fdata.176662.fdata /tmp/prof.fdata.176666.fdata /tmp/prof.fdata.176675.fdata /tmp/prof.fdata.176685.fdata /tmp/prof.fdata.176706.fdata /tmp/prof.fdata.176732.fdata /tmp/prof.fdata.176740.fdata /tmp/prof.fdata.176745.fdata /tmp/prof.fdata.176754.fdata /tmp/prof.fdata.176757.fdata /tmp/prof.fdata.176764.fdata /tmp/prof.fdata.176768.fdata /tmp/prof.fdata.176775.fdata /tmp/prof.fdata.176787.fdata /tmp/prof.fdata.176788.fdata /tmp/prof.fdata.176852.fdata /tmp/prof.fdata.176858.fdata /tmp/prof.fdata.176871.fdata /tmp/prof.fdata.176891.fdata /tmp/prof.fdata.176926.fdata /tmp/prof.fdata.176939.fdata /tmp/prof.fdata.176962.fdata /tmp/prof.fdata.177016.fdata /tmp/prof.fdata.177017.fdata /tmp/prof.fdata.177044.fdata /tmp/prof.fdata.177051.fdata /tmp/prof.fdata.177064.fdata /tmp/prof.fdata.177074.fdata /tmp/prof.fdata.177081.fdata /tmp/prof.fdata.177086.fdata /tmp/prof.fdata.177089.fdata /tmp/prof.fdata.177104.fdata /tmp/prof.fdata.177107.fdata /tmp/prof.fdata.177108.fdata /tmp/prof.fdata.177133.fdata /tmp/prof.fdata.177138.fdata /tmp/prof.fdata.177148.fdata /tmp/prof.fdata.177173.fdata /tmp/prof.fdata.177216.fdata /tmp/prof.fdata.177218.fdata /tmp/prof.fdata.177221.fdata /tmp/prof.fdata.177223.fdata /tmp/prof.fdata.177224.fdata /tmp/prof.fdata.177228.fdata /tmp/prof.fdata.177270.fdata /tmp/prof.fdata.177271.fdata /tmp/prof.fdata.177289.fdata /tmp/prof.fdata.177298.fdata /tmp/prof.fdata.177331.fdata /tmp/prof.fdata.177358.fdata /tmp/prof.fdata.177362.fdata /tmp/prof.fdata.177373.fdata /tmp/prof.fdata.177391.fdata /tmp/prof.fdata.177394.fdata /tmp/prof.fdata.177398.fdata /tmp/prof.fdata.177401.fdata /tmp/prof.fdata.177408.fdata /tmp/prof.fdata.177428.fdata /tmp/prof.fdata.177434.fdata /tmp/prof.fdata.177446.fdata /tmp/prof.fdata.177484.fdata /tmp/prof.fdata.177494.fdata /tmp/prof.fdata.177499.fdata /tmp/prof.fdata.177500.fdata /tmp/prof.fdata.177504.fdata /tmp/prof.fdata.177542.fdata /tmp/prof.fdata.177544.fdata /tmp/prof.fdata.177560.fdata /tmp/prof.fdata.177564.fdata /tmp/prof.fdata.177575.fdata /tmp/prof.fdata.177582.fdata /tmp/prof.fdata.177585.fdata /tmp/prof.fdata.177590.fdata /tmp/prof.fdata.177591.fdata /tmp/prof.fdata.177594.fdata /tmp/prof.fdata.177599.fdata /tmp/prof.fdata.177604.fdata /tmp/prof.fdata.177639.fdata /tmp/prof.fdata.177654.fdata /tmp/prof.fdata.177660.fdata /tmp/prof.fdata.177676.fdata /tmp/prof.fdata.177696.fdata /tmp/prof.fdata.177707.fdata /tmp/prof.fdata.177727.fdata /tmp/prof.fdata.177743.fdata /tmp/prof.fdata.177745.fdata /tmp/prof.fdata.177774.fdata /tmp/prof.fdata.177781.fdata /tmp/prof.fdata.177785.fdata /tmp/prof.fdata.177817.fdata /tmp/prof.fdata.177818.fdata /tmp/prof.fdata.177833.fdata /tmp/prof.fdata.177834.fdata /tmp/prof.fdata.177844.fdata /tmp/prof.fdata.177862.fdata /tmp/prof.fdata.177889.fdata /tmp/prof.fdata.177909.fdata /tmp/prof.fdata.177961.fdata /tmp/prof.fdata.177971.fdata /tmp/prof.fdata.178017.fdata /tmp/prof.fdata.178020.fdata /tmp/prof.fdata.178027.fdata /tmp/prof.fdata.178028.fdata /tmp/prof.fdata.178036.fdata /tmp/prof.fdata.178037.fdata /tmp/prof.fdata.178091.fdata /tmp/prof.fdata.178119.fdata /tmp/prof.fdata.178134.fdata /tmp/prof.fdata.178137.fdata /tmp/prof.fdata.178142.fdata /tmp/prof.fdata.178152.fdata /tmp/prof.fdata.178173.fdata /tmp/prof.fdata.178180.fdata /tmp/prof.fdata.178189.fdata /tmp/prof.fdata.178220.fdata /tmp/prof.fdata.178252.fdata /tmp/prof.fdata.178262.fdata /tmp/prof.fdata.178288.fdata /tmp/prof.fdata.178305.fdata /tmp/prof.fdata.178306.fdata /tmp/prof.fdata.178307.fdata /tmp/prof.fdata.178308.fdata /tmp/prof.fdata.178309.fdata /tmp/prof.fdata.178310.fdata /tmp/prof.fdata.178314.fdata /tmp/prof.fdata.178315.fdata /tmp/prof.fdata.178316.fdata /tmp/prof.fdata.178320.fdata /tmp/prof.fdata.178321.fdata /tmp/prof.fdata.178322.fdata /tmp/prof.fdata.178326.fdata /tmp/prof.fdata.178330.fdata /tmp/prof.fdata.178331.fdata /tmp/prof.fdata.178332.fdata /tmp/prof.fdata.178336.fdata /tmp/prof.fdata.178337.fdata /tmp/prof.fdata.178450.fdata /tmp/prof.fdata.178452.fdata /tmp/prof.fdata.178458.fdata /tmp/prof.fdata.178465.fdata /tmp/prof.fdata.178471.fdata /tmp/prof.fdata.178510.fdata /tmp/prof.fdata.178605.fdata /tmp/prof.fdata.178615.fdata /tmp/prof.fdata.178641.fdata /tmp/prof.fdata.178689.fdata /tmp/prof.fdata.178690.fdata /tmp/prof.fdata.178691.fdata /tmp/prof.fdata.178692.fdata /tmp/prof.fdata.178693.fdata /tmp/prof.fdata.178694.fdata /tmp/prof.fdata.178698.fdata /tmp/prof.fdata.178699.fdata /tmp/prof.fdata.178700.fdata /tmp/prof.fdata.178704.fdata /tmp/prof.fdata.178705.fdata /tmp/prof.fdata.178706.fdata /tmp/prof.fdata.178715.fdata /tmp/prof.fdata.178718.fdata /tmp/prof.fdata.178721.fdata /tmp/prof.fdata.178723.fdata /tmp/prof.fdata.178725.fdata /tmp/prof.fdata.178726.fdata /tmp/prof.fdata.178727.fdata /tmp/prof.fdata.178728.fdata /tmp/prof.fdata.178730.fdata /tmp/prof.fdata.178731.fdata /tmp/prof.fdata.178733.fdata /tmp/prof.fdata.178736.fdata /tmp/prof.fdata.178738.fdata /tmp/prof.fdata.178739.fdata /tmp/prof.fdata.178740.fdata /tmp/prof.fdata.178744.fdata /tmp/prof.fdata.178745.fdata /tmp/prof.fdata.178755.fdata /tmp/prof.fdata.178758.fdata /tmp/prof.fdata.179044.fdata /tmp/prof.fdata.179046.fdata /tmp/prof.fdata.179048.fdata /tmp/prof.fdata.179058.fdata /tmp/prof.fdata.179131.fdata /tmp/prof.fdata.179146.fdata /tmp/prof.fdata.179150.fdata /tmp/prof.fdata.179172.fdata /tmp/prof.fdata.179178.fdata /tmp/prof.fdata.179189.fdata /tmp/prof.fdata.179193.fdata /tmp/prof.fdata.179197.fdata /tmp/prof.fdata.179204.fdata /tmp/prof.fdata.179206.fdata /tmp/prof.fdata.179210.fdata /tmp/prof.fdata.179225.fdata /tmp/prof.fdata.179226.fdata /tmp/prof.fdata.179227.fdata /tmp/prof.fdata.179234.fdata /tmp/prof.fdata.179250.fdata /tmp/prof.fdata.179279.fdata /tmp/prof.fdata.179283.fdata /tmp/prof.fdata.179285.fdata /tmp/prof.fdata.179289.fdata /tmp/prof.fdata.179297.fdata /tmp/prof.fdata.179312.fdata /tmp/prof.fdata.179323.fdata /tmp/prof.fdata.179329.fdata /tmp/prof.fdata.179334.fdata /tmp/prof.fdata.179339.fdata /tmp/prof.fdata.179355.fdata /tmp/prof.fdata.179383.fdata /tmp/prof.fdata.179443.fdata /tmp/prof.fdata.179522.fdata /tmp/prof.fdata.179539.fdata /tmp/prof.fdata.179546.fdata /tmp/prof.fdata.179551.fdata /tmp/prof.fdata.179599.fdata /tmp/prof.fdata.179614.fdata /tmp/prof.fdata.179627.fdata /tmp/prof.fdata.179642.fdata /tmp/prof.fdata.179652.fdata /tmp/prof.fdata.179659.fdata /tmp/prof.fdata.179663.fdata /tmp/prof.fdata.179667.fdata /tmp/prof.fdata.179689.fdata /tmp/prof.fdata.179704.fdata /tmp/prof.fdata.179732.fdata /tmp/prof.fdata.179781.fdata /tmp/prof.fdata.179810.fdata /tmp/prof.fdata.179814.fdata /tmp/prof.fdata.179815.fdata /tmp/prof.fdata.179904.fdata /tmp/prof.fdata.179910.fdata /tmp/prof.fdata.179913.fdata /tmp/prof.fdata.179981.fdata /tmp/prof.fdata.179991.fdata /tmp/prof.fdata.179995.fdata /tmp/prof.fdata.180002.fdata /tmp/prof.fdata.180028.fdata /tmp/prof.fdata.180032.fdata /tmp/prof.fdata.180046.fdata /tmp/prof.fdata.180050.fdata /tmp/prof.fdata.180057.fdata /tmp/prof.fdata.180065.fdata /tmp/prof.fdata.180070.fdata /tmp/prof.fdata.180077.fdata /tmp/prof.fdata.180083.fdata /tmp/prof.fdata.180094.fdata /tmp/prof.fdata.180108.fdata /tmp/prof.fdata.180122.fdata /tmp/prof.fdata.180166.fdata /tmp/prof.fdata.180187.fdata /tmp/prof.fdata.180195.fdata /tmp/prof.fdata.180198.fdata /tmp/prof.fdata.180213.fdata /tmp/prof.fdata.180224.fdata /tmp/prof.fdata.180226.fdata /tmp/prof.fdata.180240.fdata /tmp/prof.fdata.180242.fdata /tmp/prof.fdata.180247.fdata /tmp/prof.fdata.180275.fdata /tmp/prof.fdata.180290.fdata /tmp/prof.fdata.180324.fdata /tmp/prof.fdata.180389.fdata /tmp/prof.fdata.180403.fdata /tmp/prof.fdata.180410.fdata /tmp/prof.fdata.180433.fdata /tmp/prof.fdata.180441.fdata /tmp/prof.fdata.180448.fdata /tmp/prof.fdata.180456.fdata /tmp/prof.fdata.180462.fdata /tmp/prof.fdata.180466.fdata /tmp/prof.fdata.180470.fdata /tmp/prof.fdata.180475.fdata /tmp/prof.fdata.180498.fdata /tmp/prof.fdata.180502.fdata /tmp/prof.fdata.180506.fdata /tmp/prof.fdata.180525.fdata /tmp/prof.fdata.180529.fdata /tmp/prof.fdata.180532.fdata /tmp/prof.fdata.180549.fdata /tmp/prof.fdata.180557.fdata /tmp/prof.fdata.180568.fdata /tmp/prof.fdata.180574.fdata /tmp/prof.fdata.180578.fdata /tmp/prof.fdata.180603.fdata /tmp/prof.fdata.180614.fdata /tmp/prof.fdata.180617.fdata /tmp/prof.fdata.180625.fdata /tmp/prof.fdata.180632.fdata /tmp/prof.fdata.180650.fdata /tmp/prof.fdata.180659.fdata /tmp/prof.fdata.180669.fdata /tmp/prof.fdata.180683.fdata /tmp/prof.fdata.180693.fdata /tmp/prof.fdata.180696.fdata /tmp/prof.fdata.180697.fdata /tmp/prof.fdata.180710.fdata /tmp/prof.fdata.180723.fdata /tmp/prof.fdata.180740.fdata /tmp/prof.fdata.180763.fdata /tmp/prof.fdata.180774.fdata /tmp/prof.fdata.180796.fdata /tmp/prof.fdata.180804.fdata /tmp/prof.fdata.180811.fdata /tmp/prof.fdata.180844.fdata /tmp/prof.fdata.180854.fdata /tmp/prof.fdata.180859.fdata /tmp/prof.fdata.180864.fdata /tmp/prof.fdata.180867.fdata /tmp/prof.fdata.180877.fdata /tmp/prof.fdata.180881.fdata /tmp/prof.fdata.180890.fdata /tmp/prof.fdata.180891.fdata /tmp/prof.fdata.180895.fdata /tmp/prof.fdata.180905.fdata /tmp/prof.fdata.180929.fdata /tmp/prof.fdata.180938.fdata /tmp/prof.fdata.180941.fdata /tmp/prof.fdata.180954.fdata /tmp/prof.fdata.180989.fdata /tmp/prof.fdata.181001.fdata /tmp/prof.fdata.181011.fdata /tmp/prof.fdata.181014.fdata /tmp/prof.fdata.181025.fdata /tmp/prof.fdata.181033.fdata /tmp/prof.fdata.181037.fdata /tmp/prof.fdata.181101.fdata /tmp/prof.fdata.181105.fdata /tmp/prof.fdata.181106.fdata /tmp/prof.fdata.181153.fdata /tmp/prof.fdata.181181.fdata /tmp/prof.fdata.181220.fdata /tmp/prof.fdata.181229.fdata /tmp/prof.fdata.181246.fdata /tmp/prof.fdata.181255.fdata /tmp/prof.fdata.181278.fdata /tmp/prof.fdata.181303.fdata /tmp/prof.fdata.181318.fdata /tmp/prof.fdata.181327.fdata /tmp/prof.fdata.181332.fdata /tmp/prof.fdata.181368.fdata /tmp/prof.fdata.181399.fdata /tmp/prof.fdata.181438.fdata /tmp/prof.fdata.181496.fdata /tmp/prof.fdata.181525.fdata /tmp/prof.fdata.181534.fdata /tmp/prof.fdata.181548.fdata /tmp/prof.fdata.181556.fdata /tmp/prof.fdata.181576.fdata /tmp/prof.fdata.181628.fdata /tmp/prof.fdata.181637.fdata /tmp/prof.fdata.181643.fdata /tmp/prof.fdata.181644.fdata /tmp/prof.fdata.181691.fdata /tmp/prof.fdata.181702.fdata /tmp/prof.fdata.181742.fdata /tmp/prof.fdata.181755.fdata /tmp/prof.fdata.181769.fdata /tmp/prof.fdata.181770.fdata /tmp/prof.fdata.181830.fdata /tmp/prof.fdata.181857.fdata /tmp/prof.fdata.181888.fdata /tmp/prof.fdata.181909.fdata /tmp/prof.fdata.181947.fdata /tmp/prof.fdata.181962.fdata /tmp/prof.fdata.181985.fdata /tmp/prof.fdata.182049.fdata /tmp/prof.fdata.182081.fdata /tmp/prof.fdata.182100.fdata /tmp/prof.fdata.182131.fdata /tmp/prof.fdata.182154.fdata /tmp/prof.fdata.182156.fdata /tmp/prof.fdata.182205.fdata /tmp/prof.fdata.182282.fdata /tmp/prof.fdata.182297.fdata /tmp/prof.fdata.182331.fdata /tmp/prof.fdata.182382.fdata /tmp/prof.fdata.182383.fdata /tmp/prof.fdata.182384.fdata /tmp/prof.fdata.182385.fdata /tmp/prof.fdata.182386.fdata /tmp/prof.fdata.182387.fdata /tmp/prof.fdata.182391.fdata /tmp/prof.fdata.182392.fdata /tmp/prof.fdata.182393.fdata /tmp/prof.fdata.182397.fdata /tmp/prof.fdata.182398.fdata /tmp/prof.fdata.182399.fdata /tmp/prof.fdata.182402.fdata /tmp/prof.fdata.182405.fdata /tmp/prof.fdata.182408.fdata /tmp/prof.fdata.182477.fdata /tmp/prof.fdata.182480.fdata /tmp/prof.fdata.182483.fdata /tmp/prof.fdata.182499.fdata /tmp/prof.fdata.182512.fdata /tmp/prof.fdata.182541.fdata /tmp/prof.fdata.182589.fdata /tmp/prof.fdata.182590.fdata /tmp/prof.fdata.182591.fdata /tmp/prof.fdata.182592.fdata /tmp/prof.fdata.182593.fdata /tmp/prof.fdata.182594.fdata /tmp/prof.fdata.182598.fdata /tmp/prof.fdata.182599.fdata /tmp/prof.fdata.182600.fdata /tmp/prof.fdata.182604.fdata /tmp/prof.fdata.182605.fdata /tmp/prof.fdata.182606.fdata /tmp/prof.fdata.182611.fdata /tmp/prof.fdata.182616.fdata /tmp/prof.fdata.182617.fdata /tmp/prof.fdata.182619.fdata /tmp/prof.fdata.182620.fdata /tmp/prof.fdata.182624.fdata /tmp/prof.fdata.182625.fdata /tmp/prof.fdata.182626.fdata /tmp/prof.fdata.182627.fdata /tmp/prof.fdata.182781.fdata /tmp/prof.fdata.182782.fdata /tmp/prof.fdata.182787.fdata /tmp/prof.fdata.182788.fdata /tmp/prof.fdata.182791.fdata /tmp/prof.fdata.182795.fdata /tmp/prof.fdata.182796.fdata /tmp/prof.fdata.182799.fdata /tmp/prof.fdata.182802.fdata /tmp/prof.fdata.182805.fdata /tmp/prof.fdata.182850.fdata /tmp/prof.fdata.182905.fdata /tmp/prof.fdata.182936.fdata /tmp/prof.fdata.182949.fdata /tmp/prof.fdata.182978.fdata > /tmp/tmp-multistage/bolt.profdata`
Merging data from /tmp/prof.fdata.151917.fdata...
Merging data from /tmp/prof.fdata.151925.fdata...
Merging data from /tmp/prof.fdata.151928.fdata...
Merging data from /tmp/prof.fdata.151931.fdata...
---
Merging data from /tmp/prof.fdata.182936.fdata...
Merging data from /tmp/prof.fdata.182949.fdata...
Merging data from /tmp/prof.fdata.182978.fdata...
Profile from 1337 files merged.
stage-build INFO: LLVM BOLT statistics
stage-build INFO: /tmp/tmp-multistage/bolt.profdata: 144.12 MiB
stage-build INFO: /tmp/prof.fdata: 29.01 GiB
stage-build INFO: Profile file count: 1337
stage-build INFO: Stage `Gather profiles (LLVM BOLT)` ended: OK (715.18s)
stage-build INFO: Clearing LLVM build files
stage-build INFO: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm`
stage-build INFO: Deleting directory `/checkout/obj/build/x86_64-unknown-linux-gnu/lld`
stage-build INFO: Stage `Final build` starts
stage-build INFO: Executing `python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest bootstrap --llvm-profile-use /tmp/tmp-multistage/llvm-pgo.profdata --rust-profile-use /tmp/tmp-multistage/rustc-pgo.profdata --llvm-bolt-profile-use /tmp/tmp-multistage/bolt.profdata`
    Finished dev [unoptimized] target(s) in 0.06s
Generating unstable book md files (x86_64-unknown-linux-gnu)
Building stage0 tool unstable-book-gen (x86_64-unknown-linux-gnu)
---
    Finished release [optimized] target(s) in 4.76s
Dist build-manifest-nightly-x86_64-unknown-linux-gnu
 finished in 1.817 seconds
Build completed successfully in 0:40:53
stage-build INFO: Stage `Final build` ended: OK (2454.00s)
stage-build INFO: Timer results
---------------------------------------------------------
Build rustc (LLVM PGO):                 1815.67s (21.47%)
Gather profiles (LLVM PGO):              418.73s ( 4.95%)
Build rustc (rustc PGO):                 584.46s ( 6.91%)
Gather profiles (rustc PGO):             806.32s ( 9.53%)
Build rustc (LLVM BOLT):                1662.92s (19.66%)
Gather profiles (LLVM BOLT):             715.18s ( 8.46%)
Final build:                            2454.00s (29.02%)
Total duration:                         8457.27s
stage-build INFO: Rustc binary size
rustc:                              2.63 MiB
rustdoc:                           13.97 MiB
libLLVM-15-rust-1.68.0-nightly.so:    134.35 MiB
---
  IMAGE: dist-x86_64-linux
  AWS_ACCESS_KEY_ID: 
  AWS_SECRET_ACCESS_KEY: 
##[endgroup]
cp: cannot stat 'obj/build/metrics.json': No such file or directory
##[error]Process completed with exit code 1.
