plain
##[endgroup]
##[group]Determining the checkout info
##[endgroup]
##[group]Checking out the ref
[command]"C:\Program Files\Git\bin\git.exe" checkout --progress --force -B try refs/remotes/origin/try
Updating files:  14% (3702/26440)
Updating files:  15% (3966/26440)
Updating files:  16% (4231/26440)
Updating files:  17% (4495/26440)
---
Updating files:  98% (25912/26440)
Updating files:  99% (26176/26440)
Updating files: 100% (26440/26440)
Updating files: 100% (26440/26440), done.
Branch 'try' set up to track remote branch 'try' from 'origin'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'6a89b6fdbf69aca0c7faa2cbbdf18e66d1510303'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
file:.git/config remote.origin.url=https://github.com/rust-lang-ci/rust
file:.git/config remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
file:.git/config gc.auto=0
file:.git/config http.https://github.com/.extraheader=AUTHORIZATION: basic ***
file:.git/config branch.try.remote=origin
file:.git/config branch.try.merge=refs/heads/try
file:.git/config submodule.library/backtrace.url=https://github.com/rust-lang/backtrace-rs.git
file:.git/config submodule.library/stdarch.active=true
file:.git/config submodule.library/stdarch.url=https://github.com/rust-lang/stdarch.git
file:.git/config submodule.src/doc/edition-guide.active=true
---
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\llvm-finished-building`
llvm-config: error: component libraries and shared library

llvm-config: error: missing: D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\gtest.lib
llvm-config: error: missing: D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\gtest_main.lib
llvm-config: error: missing: D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMTestingSupport.lib
thread 'main' panicked at 'command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\llvm-config.exe" "--libfiles"
expected success, got: exit code: 1', src\build_helper\lib.rs:139:9
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 0:39:09
