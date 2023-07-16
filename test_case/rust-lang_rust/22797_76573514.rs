
---- [run-pass] run-pass/issue-13304.rs stdout ----

error: test run failed!
status: exit code: 101
command: PATH="x86_64-pc-windows-gnu/stage2/bin/rustlib/x86_64-pc-windows-gnu/lib;C:\bot\slave\auto-win-64-opt\build\obj\x86_64-pc-windows-gnu\stage2\bin;C:\program files\mingw-w64\x86_64-4.9.1-win32-seh-rt_v3-rev1\mingw64\bin;C:\msys64\usr\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\msys64\usr\bin;C:\python27;C:\python27\scripts;C:\program files (x86)\inno setup 5;C:\program files (x86)\CMake\bin" x86_64-pc-windows-gnu\test\run-pass\issue-13304.stage2-x86_64-pc-windows-gnu.exe 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os(6) }', C:\bot\slave\auto-win-64-opt\build\src\libcore\result.rs:744
thread '<main>' panicked at 'assertion failed: out.status.success()', C:\bot\slave\auto-win-64-opt\build\src\test\run-pass\issue-13304.rs:34

------------------------------------------

thread '[run-pass] run-pass/issue-13304.rs' panicked at 'explicit panic', C:\bot\slave\auto-win-64-opt\build\src\compiletest\runtest.rs:1466


---- [run-pass] run-pass/issue-14456.rs stdout ----

error: test run failed!
status: exit code: 101
command: PATH="x86_64-pc-windows-gnu/stage2/bin/rustlib/x86_64-pc-windows-gnu/lib;C:\bot\slave\auto-win-64-opt\build\obj\x86_64-pc-windows-gnu\stage2\bin;C:\program files\mingw-w64\x86_64-4.9.1-win32-seh-rt_v3-rev1\mingw64\bin;C:\msys64\usr\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\msys64\usr\bin;C:\python27;C:\python27\scripts;C:\program files (x86)\inno setup 5;C:\program files (x86)\CMake\bin" x86_64-pc-windows-gnu\test\run-pass\issue-14456.stage2-x86_64-pc-windows-gnu.exe 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'assertion failed: p.wait().unwrap().success()', C:\bot\slave\auto-win-64-opt\build\src\test\run-pass\issue-14456.rs:41

------------------------------------------

thread '[run-pass] run-pass/issue-14456.rs' panicked at 'explicit panic', C:\bot\slave\auto-win-64-opt\build\src\compiletest\runtest.rs:1466

