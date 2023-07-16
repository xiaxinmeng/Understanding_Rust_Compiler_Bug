
command: PATH="C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2\bin;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-tools\i686-pc-windows-gnu\release\deps;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-rustc\i686-pc-windows-gnu\release\deps;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-test\i686-pc-windows-gnu\release\deps;C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2-std\i686-pc-windows-gnu\release\deps;C:\mingw-w64\i686-4.9.1-win32-dwarf-rt_v3-rev1\mingw32\bin;C:\Python27;C:\msys64\mingw32\bin;C:\msys64\usr\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\msys64\usr\bin;C:\python27;C:\python27\scripts;C:\program files (x86)\inno setup 5;C:\program files (x86)\CMake\bin;C:\Program Files (x86)\Windows Kits\8.1\Windows Performance Toolkit;C:\Program Files\Microsoft SQL Server\110\Tools\Binn;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.0;C:\Program Files\CMake\bin" C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\stage2\bin\rustc.exe C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src/test\ui\codemap_tests\two_files.rs -L C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\ui --target=i686-pc-windows-gnu -L C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\ui\codemap_tests\two_files.stage2-i686-pc-windows-gnu.ui.libaux -C prefer-dynamic -o C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\test\ui\codemap_tests\two_files.stage2-i686-pc-windows-gnu.exe -Crpath -O -Lnative=C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\obj\build\i686-pc-windows-gnu\rust-test-helpers
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0404]: `Bar` is not a trait
  --> C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src/test\ui\codemap_tests\two_files.rs:16:6
   |
16 | impl Bar for Baz { }
   |      ^^^
   |
   = note: type aliases cannot be used for traits

error: cannot continue compilation due to previous error


------------------------------------------

thread '[ui] ui\codemap_tests\two_files.rs' panicked at 'explicit panic', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\tools\compiletest\src\runtest.rs:2353
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [ui] ui\codemap_tests\two_files.rs

test result: FAILED. 10 passed; 1 failed; 1 ignored; 0 measured
