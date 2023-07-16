
---- git::use_the_cli stdout ----
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build -v`
thread 'git::use_the_cli' panicked at '
Expected: execs
    but: exited with exit code: 101
--- stdout
--- stderr
    Updating git repository `file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t654/dep1`
     Running `git fetch --tags --quiet file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t654/dep1 refs/heads/*:refs/heads/*`
fatal: '/C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t654/dep1' does not appear to be a git repository
fatal: Could not read from remote repository.
Please make sure you have the correct access rights
and the repository exists.
error: failed to load source for a dependency on `dep1`
Caused by:
  Unable to update file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t654/dep1
Caused by:
  failed to clone into: C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t654\home\.cargo\git\db\dep1-9eccd544448225e8
Caused by:
  process didn't exit successfully: `git fetch --tags --quiet file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t654/dep1 refs/heads/*:refs/heads/*` (exit code: 128)
', tools\cargo\tests\testsuite\support\mod.rs:731:13
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    git::use_the_cli
test result: FAILED. 1406 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out
