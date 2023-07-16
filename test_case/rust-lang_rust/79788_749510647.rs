plain
##[endgroup]
##[group]Determining the checkout info
##[endgroup]
##[group]Checking out the ref
[command]"C:\Program Files\Git\bin\git.exe" checkout --progress --force -B try refs/remotes/origin/try
Updating files:  15% (3950/26327)
Updating files:  16% (4213/26327)
Updating files:  17% (4476/26327)
Updating files:  18% (4739/26327)
---
Updating files:  98% (25801/26327)
Updating files:  99% (26064/26327)
Updating files: 100% (26327/26327)
Updating files: 100% (26327/26327), done.
Branch 'try' set up to track remote branch 'try' from 'origin'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'ee87086071b8ae244b3f25d4ddc3ca99f7854b29'
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
Dist miri-nightly-x86_64-pc-windows-msvc
 finished in 3.813 seconds
Dist rust-nightly-x86_64-pc-windows-msvc
 finished in 158.529 seconds
thread 'main' panicked at 'could not read dir "D:\\a\\rust\\rust\\build\\tmp/dist\\work\\rustc-nightly-x86_64-pc-windows-msvc\\rustc": Os { code: 3, kind: NotFound, message: "The system cannot find the path specified." }', src\bootstrap\lib.rs:1339:25
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/21dea46d8347c8b4117c5567949703f0fbb51649\/library\std\src\panicking.rs:495
   1: std::panicking::begin_panic_fmt
             at /rustc/21dea46d8347c8b4117c5567949703f0fbb51649\/library\std\src\panicking.rs:437
   2: bootstrap::Build::read_dir
             at .\src\bootstrap\lib.rs:1339
   3: bootstrap::Build::cp_r
             at .\src\bootstrap\lib.rs:1234
   4: bootstrap::dist::{{impl}}::run::{{closure}}
             at .\src\bootstrap\dist.rs:1455
   5: bootstrap::dist::{{impl}}::run
             at .\src\bootstrap\dist.rs:1461
   6: bootstrap::builder::Builder::ensure<bootstrap::dist::Extended>
             at .\src\bootstrap\builder.rs:1487
   7: bootstrap::dist::{{impl}}::make_run
             at .\src\bootstrap\dist.rs:1252
   8: bootstrap::builder::StepDescription::maybe_run
             at .\src\bootstrap\builder.rs:179
   9: bootstrap::builder::StepDescription::run
             at .\src\bootstrap\builder.rs:200
  10: bootstrap::builder::Builder::run_step_descriptions
             at .\src\bootstrap\builder.rs:570
  11: bootstrap::builder::Builder::execute_cli
             at .\src\bootstrap\builder.rs:561
  12: bootstrap::Build::build
             at .\src\bootstrap\lib.rs:510
  13: bootstrap::main
             at .\src\bootstrap\bin\main.rs:30
  14: core::ops::function::FnOnce::call_once<fn(),tuple<>>
             at /rustc/21dea46d8347c8b4117c5567949703f0fbb51649\library\core\src\ops\function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
