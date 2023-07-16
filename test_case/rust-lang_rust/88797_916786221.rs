plain
Resolving deltas:  99% (3281/3314)
Resolving deltas: 100% (3314/3314)
Resolving deltas: 100% (3314/3314), done.
From https://github.com/rust-lang/rust
 * [new ref]           d8316fe334e8eff3e3b8a7ecdcfce6903c8f105c -> pull/88797/merge
##[group]Determining the checkout info
##[endgroup]
##[group]Checking out the ref
[command]"C:\Program Files\Git\bin\git.exe" checkout --progress --force refs/remotes/pull/88797/merge
---
Updating files:  98% (29643/30247)
Updating files:  99% (29945/30247)
Updating files: 100% (30247/30247)
Updating files: 100% (30247/30247), done.
Note: switching to 'refs/remotes/pull/88797/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at d8316fe3 Merge 7b134348dea089c3f6248e82d2e32b960261fd41 into 497ee321af3b8496eaccd7af7b437f18bab81abf
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'd8316fe334e8eff3e3b8a7ecdcfce6903c8f105c'
'd8316fe334e8eff3e3b8a7ecdcfce6903c8f105c'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
  EXTRA_VARIABLES: {
 "RUST_CONFIGURE_ARGS": "--build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler",
 "SCRIPT": "python x.py dist",
 "DIST_REQUIRE_ALL_TOOLS": 0,
 "WINDOWS_SDK_HACK": 1,
 "CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION": "10.0.19041.0"
##[endgroup]
##[endgroup]
adding extra environment variable CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION
adding extra environment variable RUST_CONFIGURE_ARGS
adding extra environment variable SCRIPT
adding extra environment variable SCRIPT
adding extra environment variable WINDOWS_SDK_HACK
##[group]Run src/ci/scripts/should-skip-this.sh
src/ci/scripts/should-skip-this.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
##[endgroup]
Executing the job since there is no skip rule preventing the execution
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
##[endgroup]
##[group]Run rust-lang/simpleinfra/github-actions/cancel-outdated-builds@master
with:
  github_token: ***
  github_token: ***
  check_every_seconds: 60
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
##[endgroup]
Successfully daemonized cancel-outdated-builds.
##[group]Run src/ci/scripts/collect-cpu-stats.sh
src/ci/scripts/collect-cpu-stats.sh
src/ci/scripts/collect-cpu-stats.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
##[endgroup]
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
##[endgroup]
Attempting with retry: curl -f https://ci-mirrors.rust-lang.org/rustc/LLVM-12.0.0-win64.exe -o LLVM-12.0.0-win64.exe
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
##[endgroup]
##[group]Run src/ci/scripts/install-wix.sh
src/ci/scripts/install-wix.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
##[group]Run src/ci/scripts/disable-git-crlf-conversion.sh
src/ci/scripts/disable-git-crlf-conversion.sh
src/ci/scripts/disable-git-crlf-conversion.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
##[group]Run src/ci/scripts/install-msys2.sh
src/ci/scripts/install-msys2.sh
src/ci/scripts/install-msys2.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
##[group]Run src/ci/scripts/install-mingw.sh
src/ci/scripts/install-mingw.sh
src/ci/scripts/install-mingw.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
warning: mingw-w64-x86_64-binutils-2.37-4 is up to date -- skipping
warning: mingw-w64-x86_64-crt-git-9.0.0.6294.f5ac9206e-1 is up to date -- skipping
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
##[group]Run src/ci/scripts/disable-git-crlf-conversion.sh
src/ci/scripts/disable-git-crlf-conversion.sh
src/ci/scripts/disable-git-crlf-conversion.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
file:C:/Program Files/Git/etc/gitconfig diff.astextplain.textconv=astextplain
file:C:/Program Files/Git/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
---
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
Skipping. This is only run when merging to the beta or stable branches.
##[group]Run src/ci/scripts/run-build-from-ci.sh
##[group]Run src/ci/scripts/run-build-from-ci.sh
src/ci/scripts/run-build-from-ci.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION: 10.0.19041.0
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  SCRIPT: python x.py dist
  WINDOWS_SDK_HACK: 1
  WIX: /d/a/rust/rust/wix
  AWS_ACCESS_KEY_ID: 
  AWS_SECRET_ACCESS_KEY: 
  TOOLSTATE_REPO_ACCESS_TOKEN: 
---
   Compiling unwind v0.0.0 (D:\a\rust\rust\library\unwind)
   Compiling profiler_builtins v0.0.0 (D:\a\rust\rust\library\profiler_builtins)
The following warnings were emitted during compilation:

warning: D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\int_util.c(45,10): fatal error: 'stdlib.h' file not found
warning: #include <stdlib.h>
warning: 1 error generated.

error: failed to run custom build command for `compiler_builtins v0.1.49`


Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-std\release\build\compiler_builtins-be62898fe508dfc2\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-changed=build.rs
  cargo:compiler-rt=C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\compiler_builtins-0.1.49\compiler-rt
  cargo:rustc-cfg=feature="unstable"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\absvdi2.c
  cargo:rustc-cfg=__absvdi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\absvsi2.c
  cargo:rustc-cfg=__absvsi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\absvti2.c
  cargo:rustc-cfg=__absvti2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\addvdi3.c
  cargo:rustc-cfg=__addvdi3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\addvsi3.c
  cargo:rustc-cfg=__addvsi3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\addvti3.c
  cargo:rustc-cfg=__addvti3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\clzdi2.c
  cargo:rustc-cfg=__clzdi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\clzsi2.c
  cargo:rustc-cfg=__clzsi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\clzti2.c
  cargo:rustc-cfg=__clzti2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\cmpdi2.c
  cargo:rustc-cfg=__cmpdi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\cmpti2.c
  cargo:rustc-cfg=__cmpti2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\ctzdi2.c
  cargo:rustc-cfg=__ctzdi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\ctzsi2.c
  cargo:rustc-cfg=__ctzsi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\ctzti2.c
  cargo:rustc-cfg=__ctzti2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\divdc3.c
  cargo:rustc-cfg=__divdc3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\divsc3.c
  cargo:rustc-cfg=__divsc3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\divxc3.c
  cargo:rustc-cfg=__divxc3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\extendhfsf2.c
  cargo:rustc-cfg=__extendhfsf2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\ffsti2.c
  cargo:rustc-cfg=__ffsti2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\x86_64/floatdisf.c
  cargo:rustc-cfg=__floatdisf="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\x86_64/floatdixf.c
  cargo:rustc-cfg=__floatdixf="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\int_util.c
  cargo:rustc-cfg=__int_util="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\muldc3.c
  cargo:rustc-cfg=__muldc3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\mulsc3.c
  cargo:rustc-cfg=__mulsc3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\mulvdi3.c
  cargo:rustc-cfg=__mulvdi3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\mulvsi3.c
  cargo:rustc-cfg=__mulvsi3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\mulvti3.c
  cargo:rustc-cfg=__mulvti3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\mulxc3.c
  cargo:rustc-cfg=__mulxc3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\negdf2.c
  cargo:rustc-cfg=__negdf2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\negdi2.c
  cargo:rustc-cfg=__negdi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\negsf2.c
  cargo:rustc-cfg=__negsf2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\negti2.c
  cargo:rustc-cfg=__negti2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\negvdi2.c
  cargo:rustc-cfg=__negvdi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\negvsi2.c
  cargo:rustc-cfg=__negvsi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\negvti2.c
  cargo:rustc-cfg=__negvti2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\paritydi2.c
  cargo:rustc-cfg=__paritydi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\paritysi2.c
  cargo:rustc-cfg=__paritysi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\parityti2.c
  cargo:rustc-cfg=__parityti2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\popcountdi2.c
  cargo:rustc-cfg=__popcountdi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\popcountsi2.c
  cargo:rustc-cfg=__popcountsi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\popcountti2.c
  cargo:rustc-cfg=__popcountti2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\powixf2.c
  cargo:rustc-cfg=__powixf2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\subvdi3.c
  cargo:rustc-cfg=__subvdi3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\subvsi3.c
  cargo:rustc-cfg=__subvsi3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\subvti3.c
  cargo:rustc-cfg=__subvti3="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\truncdfhf2.c
  cargo:rustc-cfg=__truncdfhf2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\truncdfsf2.c
  cargo:rustc-cfg=__truncdfsf2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\truncsfhf2.c
  cargo:rustc-cfg=__truncsfhf2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\ucmpdi2.c
  cargo:rustc-cfg=__ucmpdi2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\ucmpti2.c
  cargo:rustc-cfg=__ucmpti2="optimized-c"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\apple_versioning.c
  cargo:rustc-cfg=apple_versioning="optimized-c"
  TARGET = Some("x86_64-pc-windows-msvc")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-pc-windows-msvc")
  CC_x86_64-pc-windows-msvc = None
  CC_x86_64_pc_windows_msvc = None
  HOST_CC = None
  CC = Some("D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe")
  CFLAGS_x86_64-pc-windows-msvc = None
  CFLAGS_x86_64_pc_windows_msvc = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("crt-static,fxsr,sse,sse2")
  DEBUG = Some("true")
  CC_x86_64-pc-windows-msvc = None
  CC_x86_64_pc_windows_msvc = None
  HOST_CC = None
  CC = Some("D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe")
  CFLAGS_x86_64-pc-windows-msvc = None
  CFLAGS_x86_64_pc_windows_msvc = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("crt-static,fxsr,sse,sse2")
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\absvdi2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\absvdi2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\absvsi2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\absvsi2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\absvti2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\absvti2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\addvdi3.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\addvdi3.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\addvsi3.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\addvsi3.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\addvti3.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\addvti3.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\clzdi2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\clzdi2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\clzsi2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\clzsi2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\clzti2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\clzti2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\cmpdi2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\cmpdi2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\cmpti2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\cmpti2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\ctzdi2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\ctzdi2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\ctzsi2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\ctzsi2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\ctzti2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\ctzti2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\divdc3.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\divdc3.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\divsc3.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\divsc3.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\divxc3.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\divxc3.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\extendhfsf2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\extendhfsf2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\ffsti2.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\ffsti2.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\floatdisf.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\x86_64/floatdisf.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\floatdixf.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\x86_64/floatdixf.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\int_util.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\int_util.c"
  cargo:warning=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\int_util.c(45,10): fatal error: 'stdlib.h' file not found
  cargo:warning=#include <stdlib.h>
  cargo:warning=         ^~~~~~~~~~
  cargo:warning=1 error generated.

  --- stderr



  error occurred: Command "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "-m64" "/Zl" "-D__func__=__FUNCTION__" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d9fe7db75a49f05a\\out\\int_util.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\int_util.c" with args "clang-cl.exe" did not execute successfully (status code exit code: 1).

warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:10
