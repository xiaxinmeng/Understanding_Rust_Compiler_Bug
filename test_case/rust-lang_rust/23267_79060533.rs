
---- [run-fail] run-fail/rt-set-exit-status-panic2.rs stdout ----

error: failure produced the wrong error: exit code: 0
status: exit code: 0
command: PATH="i686-pc-windows-gnu/stage2/bin/rustlib/i686-pc-windows-gnu/lib;C:\bot\slave\auto-win-32-opt\build\obj\i686-pc-windows-gnu\stage2\bin;C:\program files (x86)\mingw-w64\i686-4.9.1-win32-dwarf-rt_v3-rev1\mingw32\bin;C:\msys64\usr\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\msys64\usr\bin;C:\python27;C:\python27\scripts;C:\program files (x86)\inno setup 5;C:\program files (x86)\CMake\bin" i686-pc-windows-gnu/test/run-fail\rt-set-exit-status-panic2.stage2-i686-pc-windows-gnu.exe 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
ERROR:rt-set-exit-status-panic2: whatever
thread '<main>' panicked at 'explicit panic', C:/bot/slave/auto-win-32-opt/build/src/test/run-fail/rt-set-exit-status-panic2.rs:41

------------------------------------------

thread '[run-fail] run-fail/rt-set-exit-status-panic2.rs' panicked at 'explicit panic', C:/bot/slave/auto-win-32-opt/build/src/compiletest\runtest.rs:1479



failures:
    [run-fail] run-fail/rt-set-exit-status-panic2.rs
