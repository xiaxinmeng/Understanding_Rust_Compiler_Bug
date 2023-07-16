
failures:
---- fix::local_paths_no_fix stdout ----
running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe fix --edition --allow-no-vcs`
thread 'fix::local_paths_no_fix' panicked at '
Expected: execs
    but: differences:
  4 - |[FINISHED] [..]|
    + |      Fixing src\lib.rs (1 fix)|
  5 -
    + |    Finished dev [unoptimized + debuginfo] target(s) in 0.35s|
other output:
``', tools\cargo\tests\testsuite\support\mod.rs:731:13
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    fix::local_paths_no_fix
