plain
Updating files:  98% (31382/32022)
Updating files:  99% (31702/32022)
Updating files: 100% (32022/32022)
Updating files: 100% (32022/32022), done.
Note: switching to 'refs/remotes/pull/93790/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at 0292c968 Merge 0d17699d7da86045f96ef95513f9ba306ac68582 into 56cd04af5c389b6ab676ba16f59d9f70bc465090
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'0292c9681f81b44df7227387ff17dbd0c8ed08e0'
'0292c9681f81b44df7227387ff17dbd0c8ed08e0'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "RUST_CONFIGURE_ARGS": "--build=x86_64-pc-windows-msvc --enable-profiler",
 "SCRIPT": "./eric.sh"
##[endgroup]
adding extra environment variable RUST_CONFIGURE_ARGS
adding extra environment variable SCRIPT
##[group]Run src/ci/scripts/should-skip-this.sh
---
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler
  SCRIPT: ./eric.sh
Executing the job since there is no skip rule preventing the execution
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler
  SCRIPT: ./eric.sh
##[group]Run rust-lang/simpleinfra/github-actions/cancel-outdated-builds@master
with:
  github_token: ***
  check_every_seconds: 60
  check_every_seconds: 60
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler
  SCRIPT: ./eric.sh
Successfully daemonized cancel-outdated-builds.
##[group]Run src/ci/scripts/collect-cpu-stats.sh
src/ci/scripts/collect-cpu-stats.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler
  SCRIPT: ./eric.sh
---
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler
  SCRIPT: ./eric.sh
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler
  SCRIPT: ./eric.sh
Attempting with retry: curl -f https://ci-mirrors.rust-lang.org/rustc/LLVM-12.0.0-win64.exe -o LLVM-12.0.0-win64.exe
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

---
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: ./eric.sh
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: ./eric.sh
##[endgroup]
##[group]Run src/ci/scripts/disable-git-crlf-conversion.sh
src/ci/scripts/disable-git-crlf-conversion.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: ./eric.sh
##[endgroup]
##[group]Run src/ci/scripts/install-msys2.sh
src/ci/scripts/install-msys2.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: ./eric.sh
##[endgroup]
##[group]Run src/ci/scripts/install-mingw.sh
src/ci/scripts/install-mingw.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: ./eric.sh
##[endgroup]
warning: mingw-w64-x86_64-binutils-2.37-4 is up to date -- skipping
warning: mingw-w64-x86_64-crt-git-9.0.0.6373.5be8fcd83-1 is up to date -- skipping
warning: mingw-w64-x86_64-gcc-11.2.0-8 is up to date -- skipping
---
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: ./eric.sh
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

---
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: ./eric.sh
##[endgroup]
##[group]Run src/ci/scripts/disable-git-crlf-conversion.sh
src/ci/scripts/disable-git-crlf-conversion.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: ./eric.sh
##[endgroup]
---
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: ./eric.sh
##[endgroup]
file:C:/Program Files/Git/etc/gitconfig diff.astextplain.textconv=astextplain
file:C:/Program Files/Git/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
file:C:/Program Files/Git/etc/gitconfig filter.lfs.smudge=git-lfs smudge -- %f
---
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: ./eric.sh
##[endgroup]
Skipping. This is only run when merging to the beta or stable branches.
##[group]Run src/ci/scripts/verify-stable-version-number.sh
src/ci/scripts/verify-stable-version-number.sh
src/ci/scripts/verify-stable-version-number.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: ./eric.sh
##[endgroup]
This script only works on the stable channel. Skipping the check.
##[group]Run src/ci/scripts/run-build-from-ci.sh
src/ci/scripts/run-build-from-ci.sh
src/ci/scripts/run-build-from-ci.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: ./eric.sh
  AWS_ACCESS_KEY_ID: 
  AWS_SECRET_ACCESS_KEY: 
  TOOLSTATE_REPO_ACCESS_TOKEN: 
##[endgroup]
---
+ for i in {1..200}
+ echo 1
+ python3 ../x.py test --stage 2 compiler/rustc_macros
1
C:\hostedtoolcache\windows\Python\3.10.1\x64\python3.exe: can't open file 'D:\\a\\rust\\x.py': [Errno 2] No such file or directory
