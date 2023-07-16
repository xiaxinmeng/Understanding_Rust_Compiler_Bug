plain

failures:

---- tool_paths::target_in_environment_contains_lower_case stdout ----
running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build -v --target x86_64-pc-windows-msvc`
thread 'tool_paths::target_in_environment_contains_lower_case' panicked at '
test failed running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build -v --target x86_64-pc-windows-msvc`
error: process exited with code 0 (expected 101)

--- stderr
--- stderr
   Compiling foo v0.0.1 (D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\tmp\cit\t2034\foo)
     Running `rustc --crate-name foo src\main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=74785c1c2d8cc586 --out-dir D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\tmp\cit\t2034\foo\target\x86_64-pc-windows-msvc\debug\deps --target x86_64-pc-windows-msvc -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\tmp\cit\t2034\foo\target\x86_64-pc-windows-msvc\debug\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\tmp\cit\t2034\foo\target\debug\deps`
', src\tools\cargo\tests\testsuite\tool_paths.rs:381:11


failures:
