
---- [codegen-units] codegen-units\partitioning\local-inlining.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="C:\bot\slave\auto-win-msvc-32-opt\build\obj\i686-pc-windows-msvc/stage2/bin;C:\bot\slave\auto-win-msvc-32-opt\build\obj\i686-pc-windows-msvc\stage2\bin;C:\bot\slave\auto-win-msvc-32-opt\build\obj\i686-pc-windows-msvc\llvm\Release\lib;C:\mingw-w64\i686-4.9.1-win32-dwarf-rt_v3-rev1\mingw32\bin;C:\Python27;C:\msys64\mingw32\bin;C:\msys64\usr\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\msys64\usr\bin;C:\python27;C:\python27\scripts;C:\program files (x86)\inno setup 5;C:\program files (x86)\CMake\bin;C:\Program Files (x86)\Windows Kits\8.1\Windows Performance Toolkit;C:\Program Files\Microsoft SQL Server\110\Tools\Binn;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.0" i686-pc-windows-msvc/stage2/bin/rustc.exe C:/bot/slave/auto-win-msvc-32-opt/build/src/test/codegen-units/partitioning\local-inlining.rs -L i686-pc-windows-msvc/test/codegen-units/ --target=i686-pc-windows-msvc -L i686-pc-windows-msvc/test/codegen-units/partitioning\local-inlining.stage2-i686-pc-windows-msvc.codegen-units.libaux -C prefer-dynamic -o i686-pc-windows-msvc/test/codegen-units/partitioning\local-inlining.stage2-i686-pc-windows-msvc.exe --cfg rtopt -C rpath -O -L i686-pc-windows-msvc/rt -Zprint-trans-items=lazy -Zincremental=tmp
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: could not load dep-graph from `tmp\dep_graph.rbml`: The system cannot find the file specified. (os error 2)
error: aborting due to previous error
