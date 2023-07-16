plain
##[endgroup]
##[group]Determining the checkout info
##[endgroup]
##[group]Checking out the ref
[command]"C:\Program Files\Git\bin\git.exe" checkout --progress --force -B try refs/remotes/origin/try
Updating files:  13% (3438/26440)
Updating files:  14% (3702/26440)
Updating files:  15% (3966/26440)
Updating files:  16% (4231/26440)
---
Updating files:  98% (25912/26440)
Updating files:  99% (26176/26440)
Updating files: 100% (26440/26440)
Updating files: 100% (26440/26440), done.
Branch 'try' set up to track remote branch 'try' from 'origin'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'4875739ceccafbe93c6868c1e4c43d51ae412ab2'
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
thread 'main' panicked at 'command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\llvm-config.exe" "--components" "--libfiles"
expected success, got: exit code: 1', src\build_helper\lib.rs:139:9
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/05b6023675d77979637b04a350c85903fbf59257\/library\std\src\panicking.rs:493
   1: std::panicking::begin_panic_fmt
             at /rustc/05b6023675d77979637b04a350c85903fbf59257\/library\std\src\panicking.rs:435
             at .\src\build_helper\lib.rs:139
   3: bootstrap::dist::maybe_install_llvm
             at .\src\bootstrap\dist.rs:1841
             at .\src\bootstrap\dist.rs:1841
   4: bootstrap::dist::{{impl}}::run
             at .\src\bootstrap\dist.rs:1985
   5: bootstrap::builder::Builder::ensure<bootstrap::dist::RustDev>
             at .\src\bootstrap\builder.rs:1484
   6: bootstrap::dist::{{impl}}::make_run
             at .\src\bootstrap\dist.rs:1944
   7: bootstrap::builder::StepDescription::maybe_run
             at .\src\bootstrap\builder.rs:179
   8: bootstrap::builder::StepDescription::run
             at .\src\bootstrap\builder.rs:200
   9: bootstrap::builder::Builder::run_step_descriptions
             at .\src\bootstrap\builder.rs:570
  10: bootstrap::builder::Builder::execute_cli
             at .\src\bootstrap\builder.rs:561
  11: bootstrap::Build::build
             at .\src\bootstrap\lib.rs:510
  12: bootstrap::main
             at .\src\bootstrap\bin\main.rs:30
  13: core::ops::function::FnOnce::call_once<fn(),tuple<>>
             at /rustc/05b6023675d77979637b04a350c85903fbf59257\library\core\src\ops\function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
