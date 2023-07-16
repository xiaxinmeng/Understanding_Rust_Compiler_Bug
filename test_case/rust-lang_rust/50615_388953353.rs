
failures:
---- build::rustc_no_trans stdout ----
	running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe rustc -v -- -Zno-trans`
thread 'build::rustc_no_trans' panicked at '
Expected: execs
    but: exited with exit code: 101
--- stdout
--- stderr
   Compiling foo v0.0.1 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t266/foo)
     Running `rustc --crate-name foo src\main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -Zno-trans -C metadata=151c89d381d36d6f -C extra-filename=-151c89d381d36d6f --out-dir C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t266\foo\target\debug\deps -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t266\foo\target\debug\deps`
error: unknown debugging option: `no-trans`
error: Could not compile `foo`.
Caused by:
  process didn't exit successfully: `rustc --crate-name foo src\main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -Zno-trans -C metadata=151c89d381d36d6f -C extra-filename=-151c89d381d36d6f --out-dir C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t266\foo\target\debug\deps -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t266\foo\target\debug\deps` (exit code: 101)
', tools\cargo\tests\testsuite\hamcrest.rs:13:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
