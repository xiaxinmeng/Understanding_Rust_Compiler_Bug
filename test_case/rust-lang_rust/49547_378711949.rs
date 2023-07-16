
test workspaces::workspace_with_transitive_dev_deps ... ok
failures:
---- rustflags::two_matching_in_config stdout ----
	running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe run`
thread 'rustflags::two_matching_in_config' panicked at '
Expected: execs
    but: exited with exit code: 3221225477
--- stdout
--- stderr
   Compiling foo v0.0.1 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t983/foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target\debug\foo.exe`
error: process didn't exit successfully: `target\debug\foo.exe` (exit code: 3221225477)
', tools\cargo\tests\testsuite\hamcrest.rs:13:9
failures:
    rustflags::two_matching_in_config
test result: FAILED. 1199 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out
