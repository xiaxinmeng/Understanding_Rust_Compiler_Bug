

failures:

---- [run-pass] run-pass\panic-recover-propagate.rs stdout ----

error: test run failed!
status: exit code: 3221225725
command: PATH="C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-tools\i686-pc-windows-gnu\release\deps;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-rustc\i686-pc-windows-gnu\release\deps;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-test\i686-pc-windows-gnu\release\deps;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-std\i686-pc-windows-gnu\release\deps;C:\mingw-w64\i686-4.9.1-win32-dwarf-rt_v3-rev1\mingw32\bin;C:\Python27;C:\msys64\mingw32\bin;C:\msys64\usr\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\msys64\usr\bin;C:\python27;C:\python27\scripts;C:\program files (x86)\inno setup 5;C:\program files (x86)\CMake\bin;C:\Program Files (x86)\Windows Kits\8.1\Windows Performance Toolkit;C:\Program Files\Microsoft SQL Server\110\Tools\Binn;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.0" C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\run-pass\panic-recover-propagate.stage2-i686-pc-windows-gnu.exe 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

thread '<unknown>' has overflowed its stack

------------------------------------------

thread '[run-pass] run-pass\panic-recover-propagate.rs' panicked at 'explicit panic', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\tools\compiletest\src\runtest.rs:2243
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass\panic-recover-propagate.rs

test result: FAILED. 2459 passed; 1 failed; 14 ignored; 0 measured



command did not execute successfully: "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\stage2-tools\\i686-pc-windows-gnu\\release\\compiletest.exe" "--compile-lib-path" "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\stage2\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "--rustc-path" "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--rustdoc-path" "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\stage2\\bin\\rustdoc.exe" "--src-base" "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\src/test\\run-pass" "--build-base" "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\test\\run-pass" "--stage-id" "stage2-i686-pc-windows-gnu" "--mode" "run-pass" "--target" "i686-pc-windows-gnu" "--host" "i686-pc-windows-gnu" "--llvm-filecheck" "C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=C:\\bot\\slave\\auto-win-gnu-32-opt-rustbuild\\build\\obj\\build\\i686-pc-windows-gnu\\rust-test-helpers" "--docck-python" "python" "--lldb-python" "python" "--gdb-version" "GNU gdb (GDB) 7.8" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp" "--android-cross-path" ""
expected success, got: exit code: 101
