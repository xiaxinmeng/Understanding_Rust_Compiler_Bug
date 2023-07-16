plain
Updating files:  98% (32228/32885)
Updating files:  99% (32557/32885)
Updating files: 100% (32885/32885)
Updating files: 100% (32885/32885), done.
HEAD is now at 841e5765 Merge 8e1d6168d65dde432c242f62f035959e38311100 into 3d18f945cab17c98a2d4bedd174c47a24ecfe8f4
[command]/usr/local/bin/git log -1 --format='%H'
'841e5765e338c1f2d2c975ed9c2303a11a670d36'
'841e5765e338c1f2d2c975ed9c2303a11a670d36'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: dist-x86_64-apple
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "SCRIPT": "PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin",
 "RUSTC_RETRY_LINKER_ON_SEGFAULT": 1,
 "MACOSX_DEPLOYMENT_TARGET": 10.7,
 "NO_LLVM_ASSERTIONS": 1,
 "NO_DEBUG_ASSERTIONS": 1,
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
##[endgroup]
Executing the job since there is no skip rule preventing the execution
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
##[endgroup]
##[group]Run rust-lang/simpleinfra/github-actions/cancel-outdated-builds@master
with:
  github_token: ***
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
##[endgroup]
Successfully daemonized cancel-outdated-builds.
##[group]Run src/ci/scripts/collect-cpu-stats.sh
src/ci/scripts/collect-cpu-stats.sh
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
##[endgroup]
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
##[endgroup]
Attempting with retry: curl -f https://ci-mirrors.rust-lang.org/rustc/clang%2Bllvm-12.0.0-x86_64-apple-darwin.tar.xz -o clang+llvm-12.0.0-x86_64-apple-darwin.tar.xz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
  NO_LLVM_ASSERTIONS: 1
  NO_OVERFLOW_CHECKS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: PGO_HOST=x86_64-apple-darwin ./src/ci/pgo.sh python3 ./x.py dist --host=x86_64-apple-darwin --target=x86_64-apple-darwin
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.1.sdk
  AR: ar
---
      Serial Number (system): VMhVfklF1tEw
      Hardware UUID: 4203018E-580F-C1B5-9525-B745CECA79EB
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

/usr/local/bin/python3: can't open file '/Users/runner/work/rust/rust/../x.py': [Errno 2] No such file or directory
hw.byteorder: 1234
hw.memsize: 15032385536
hw.activecpu: 3
hw.optional.adx: 0
