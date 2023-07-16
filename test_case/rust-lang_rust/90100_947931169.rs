plain
Updating files:  98% (30215/30831)
Updating files:  99% (30523/30831)
Updating files: 100% (30831/30831)
Updating files: 100% (30831/30831), done.
Switched to a new branch 'try'
Branch 'try' set up to track remote branch 'try' from 'origin'.
[command]/usr/local/bin/git log -1 --format='%H'
'a2b2936730df129423af44dab918d380df6ccdd4'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "SCRIPT": "./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc",
 "RUSTC_RETRY_LINKER_ON_SEGFAULT": 1,
 "MACOSX_DEPLOYMENT_TARGET": 10.7,
 "NO_LLVM_ASSERTIONS": 1,
 "NO_DEBUG_ASSERTIONS": 1,
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
##[endgroup]
Executing the job since there is no skip rule preventing the execution
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
##[endgroup]
channel verification is only executed on PR builds
##[group]Run src/ci/scripts/collect-cpu-stats.sh
src/ci/scripts/collect-cpu-stats.sh
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
##[endgroup]
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
##[endgroup]
Attempting with retry: curl -f https://ci-mirrors.rust-lang.org/rustc/clang%2Bllvm-12.0.0-x86_64-apple-darwin.tar.xz -o clang+llvm-12.0.0-x86_64-apple-darwin.tar.xz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
  NO_DEBUG_ASSERTIONS: 1
  NO_LLVM_ASSERTIONS: 1
  RUSTC_RETRY_LINKER_ON_SEGFAULT: 1
  RUST_CONFIGURE_ARGS: --host=x86_64-apple-darwin --target=x86_64-apple-darwin,aarch64-apple-ios,x86_64-apple-ios,aarch64-apple-ios-sim --enable-full-tools --enable-sanitizers --enable-profiler --set rust.jemalloc --set llvm.ninja=false
  SCRIPT: ./x.py dist --exclude src/doc && ./x.py dist --target=x86_64-apple-darwin src/doc
  CC: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang
  CXX: /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang++
  SDKROOT: /Applications/Xcode_13.0.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX11.3.sdk
  AR: ar
---
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMDtxeXoSpZY
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
exit status: 0
running: "xcrun" "--show-sdk-path" "--sdk" "iphoneos"
exit status: 0
Skipping Set({"src/doc"}) because it is excluded
Set({"src/librustc"}) not skipped for "bootstrap::dist::RustcDocs" -- not in ["src/doc"]
Set({}) not skipped for "bootstrap::dist::Mingw" -- not in ["src/doc"]
Set({"src/librustc"}) not skipped for "bootstrap::dist::Rustc" -- not in ["src/doc"]
Set({"library/std"}) not skipped for "bootstrap::dist::Std" -- not in ["src/doc"]
Set({"rustc-dev"}) not skipped for "bootstrap::dist::RustcDev" -- not in ["src/doc"]
Set({"analysis"}) not skipped for "bootstrap::dist::Analysis" -- not in ["src/doc"]
Set({"src"}) not skipped for "bootstrap::dist::Src" -- not in ["src/doc"]
Set({"cargo"}) not skipped for "bootstrap::dist::Cargo" -- not in ["src/doc"]
Set({"rls"}) not skipped for "bootstrap::dist::Rls" -- not in ["src/doc"]
Set({"rust-analyzer"}) not skipped for "bootstrap::dist::RustAnalyzer" -- not in ["src/doc"]
Set({"rustfmt"}) not skipped for "bootstrap::dist::Rustfmt" -- not in ["src/doc"]
Set({"rust-demangler"}) not skipped for "bootstrap::dist::RustDemangler" -- not in ["src/doc"]
Set({"clippy"}) not skipped for "bootstrap::dist::Clippy" -- not in ["src/doc"]
Set({"miri"}) not skipped for "bootstrap::dist::Miri" -- not in ["src/doc"]
Set({"llvm-tools"}) not skipped for "bootstrap::dist::LlvmTools" -- not in ["src/doc"]
Set({"rust-dev"}) not skipped for "bootstrap::dist::RustDev" -- not in ["src/doc"]
Set({"extended"}) not skipped for "bootstrap::dist::Extended" -- not in ["src/doc"]
Set({"rust-demangler"}) not skipped for "bootstrap::dist::RustDemangler" -- not in ["src/doc"]
Set({"cargo"}) not skipped for "bootstrap::dist::Cargo" -- not in ["src/doc"]
Set({"rustfmt"}) not skipped for "bootstrap::dist::Rustfmt" -- not in ["src/doc"]
Set({"rustfmt"}) not skipped for "bootstrap::dist::Rustfmt" -- not in ["src/doc"]
Set({"rls"}) not skipped for "bootstrap::dist::Rls" -- not in ["src/doc"]
Set({"rust-analyzer"}) not skipped for "bootstrap::dist::RustAnalyzer" -- not in ["src/doc"]
Set({"llvm-tools"}) not skipped for "bootstrap::dist::LlvmTools" -- not in ["src/doc"]
Set({"clippy"}) not skipped for "bootstrap::dist::Clippy" -- not in ["src/doc"]
Set({"miri"}) not skipped for "bootstrap::dist::Miri" -- not in ["src/doc"]
Set({"analysis"}) not skipped for "bootstrap::dist::Analysis" -- not in ["src/doc"]
Set({"reproducible"}) not skipped for "bootstrap::dist::ReproducibleArtifacts" -- not in ["src/doc"]
Set({"src/librustc"}) not skipped for "bootstrap::dist::RustcDocs" -- not in ["src/doc"]
Set({"src/librustc"}) not skipped for "bootstrap::dist::RustcDocs" -- not in ["src/doc"]
Set({}) not skipped for "bootstrap::dist::Mingw" -- not in ["src/doc"]
Set({"src/librustc"}) not skipped for "bootstrap::dist::Rustc" -- not in ["src/doc"]
---
    Finished release [optimized + debuginfo] target(s) in 1m 41s
Copying stage2 std from stage2 (x86_64-apple-darwin -> x86_64-apple-darwin / aarch64-apple-ios-sim)
Dist rust-std-nightly-aarch64-apple-ios-sim
 finished in 20.007 seconds
Set({"rustc-dev"}) not skipped for "bootstrap::dist::RustcDev" -- not in ["src/doc"]
 finished in 148.760 seconds
 finished in 148.760 seconds
Set({"analysis"}) not skipped for "bootstrap::dist::Analysis" -- not in ["src/doc"]
 finished in 11.487 seconds
Dist rust-analysis-nightly-aarch64-apple-ios
 finished in 10.618 seconds
Dist rust-analysis-nightly-x86_64-apple-ios
Dist rust-analysis-nightly-x86_64-apple-ios
 finished in 10.468 seconds
Dist rust-analysis-nightly-aarch64-apple-ios-sim
 finished in 10.536 seconds
Set({"src"}) not skipped for "bootstrap::dist::Src" -- not in ["src/doc"]
Set({"cargo"}) not skipped for "bootstrap::dist::Cargo" -- not in ["src/doc"]
 finished in 13.368 seconds
Building stage1 tool cargo (x86_64-apple-darwin)
---
   Compiling cargo-credential-1password v0.1.0 (/Users/runner/work/rust/rust/src/tools/cargo/crates/credential/cargo-credential-1password)
    Finished release [optimized] target(s) in 22.34s
Dist cargo-nightly-x86_64-apple-darwin
 finished in 8.731 seconds
Set({"rls"}) not skipped for "bootstrap::dist::Rls" -- not in ["src/doc"]
---
   Compiling rust-demangler v0.0.1 (/Users/runner/work/rust/rust/src/tools/rust-demangler)
    Finished release [optimized] target(s) in 14.60s
Dist rust-demangler-nightly-x86_64-apple-darwin
 finished in 0.717 seconds
Set({"clippy"}) not skipped for "bootstrap::dist::Clippy" -- not in ["src/doc"]
    Finished release [optimized] target(s) in 0.31s
Dist clippy-nightly-x86_64-apple-darwin
 finished in 3.926 seconds
Set({"miri"}) not skipped for "bootstrap::dist::Miri" -- not in ["src/doc"]
---
   Compiling cargo-miri v0.1.0 (/Users/runner/work/rust/rust/src/tools/miri/cargo-miri)
    Finished release [optimized] target(s) in 49.44s
Dist miri-nightly-x86_64-apple-darwin
 finished in 1.802 seconds
Set({"llvm-tools"}) not skipped for "bootstrap::dist::LlvmTools" -- not in ["src/doc"]
 finished in 44.054 seconds
 finished in 44.054 seconds
Set({"rust-dev"}) not skipped for "bootstrap::dist::RustDev" -- not in ["src/doc"]
To force LLVM to rebuild, remove the file `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/llvm-finished-building`
Dist rust-dev-nightly-x86_64-apple-darwin
 finished in 62.174 seconds
Dist extended stage1 (x86_64-apple-darwin)
Dist extended stage1 (x86_64-apple-darwin)
Set({"extended"}) not skipped for "bootstrap::dist::Extended" -- not in ["src/doc"]
Set({"rust-demangler"}) not skipped for "bootstrap::dist::RustDemangler" -- not in ["src/doc"]
Set({"cargo"}) not skipped for "bootstrap::dist::Cargo" -- not in ["src/doc"]
Set({"rustfmt"}) not skipped for "bootstrap::dist::Rustfmt" -- not in ["src/doc"]
Set({"rustfmt"}) not skipped for "bootstrap::dist::Rustfmt" -- not in ["src/doc"]
Set({"rls"}) not skipped for "bootstrap::dist::Rls" -- not in ["src/doc"]
Set({"rust-analyzer"}) not skipped for "bootstrap::dist::RustAnalyzer" -- not in ["src/doc"]
Set({"llvm-tools"}) not skipped for "bootstrap::dist::LlvmTools" -- not in ["src/doc"]
Set({"clippy"}) not skipped for "bootstrap::dist::Clippy" -- not in ["src/doc"]
Set({"miri"}) not skipped for "bootstrap::dist::Miri" -- not in ["src/doc"]
Set({"analysis"}) not skipped for "bootstrap::dist::Analysis" -- not in ["src/doc"]
 finished in 151.196 seconds
building pkg installer
pkgbuild: Adding top-level postinstall script
pkgbuild: Wrote package to /Users/runner/work/rust/rust/build/tmp/dist/combined-tarball/pkg/rustc.pkg
pkgbuild: Wrote package to /Users/runner/work/rust/rust/build/tmp/dist/combined-tarball/pkg/rustc.pkg
pkgbuild: Adding top-level postinstall script
pkgbuild: Wrote package to /Users/runner/work/rust/rust/build/tmp/dist/combined-tarball/pkg/cargo.pkg
thread 'main' panicked at 'could not read dir "/Users/runner/work/rust/rust/build/tmp/tarball/rust/x86_64-apple-darwin/rust-docs-nightly-x86_64-apple-darwin": Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/bootstrap/lib.rs:1472:25
Build completed unsuccessfully in 2:07:30
