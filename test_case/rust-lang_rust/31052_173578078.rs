

failures:

---- [run-pass] run-pass/sepcomp-lib-lto.rs stdout ----

error: compilation failed!
status: exit code: 1
command: PATH="x86_64-pc-windows-msvc/stage2/bin;C:\bot\slave\auto-win-msvc-64-opt\build\obj\x86_64-pc-windows-msvc\stage2\bin;C:\bot\slave\auto-win-msvc-64-opt\build\obj\x86_64-pc-windows-msvc\llvm\Release\lib;C:\mingw-w64\x86_64-4.9.1-win32-seh-rt_v3-rev1\mingw64\bin;C:\msys64\mingw64\bin;C:\msys64\usr\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\msys64\usr\bin;C:\python27;C:\python27\scripts;C:\program files (x86)\inno setup 5;C:\program files (x86)\CMake\bin;C:\Program Files (x86)\Windows Kits\8.1\Windows Performance Toolkit;C:\Program Files\Microsoft SQL Server\110\Tools\Binn;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.0" x86_64-pc-windows-msvc/stage2/bin/rustc.exe C:/bot/slave/auto-win-msvc-64-opt/build/src/test/run-pass/sepcomp-lib-lto.rs -L x86_64-pc-windows-msvc/test/run-pass/ --target=x86_64-pc-windows-msvc -L x86_64-pc-windows-msvc/test/run-pass\sepcomp-lib-lto.stage2-x86_64-pc-windows-msvc.run-pass.libaux -o x86_64-pc-windows-msvc/test/run-pass\sepcomp-lib-lto.stage2-x86_64-pc-windows-msvc.exe --cfg rtopt -C rpath -O -L x86_64-pc-windows-msvc/rt -C lto
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
Instruction does not dominate all uses!
  %oldret10 = insertvalue %str_slice %oldret8, i64 %1209, 1
  %.sink.i.i.i = phi %str_slice [ %oldret10, %"_ZN3str6traits45str.ops..Index$LT$ops..Range$LT$usize$GT$$GT$5index20h11b34fcbe1b5d5d0JVSE.exit.i.i.i" ], [ %oldret22, %"_ZN3str6traits45str.ops..Index$LT$ops..Range$LT$usize$GT$$GT$5index20h11b34fcbe1b5d5d0JVSE.exit539.i.i.i" ]
Instruction does not dominate all uses!
  %oldret22 = insertvalue %str_slice %oldret20, i64 %1223, 1
  %.sink.i.i.i = phi %str_slice [ %oldret10, %"_ZN3str6traits45str.ops..Index$LT$ops..Range$LT$usize$GT$$GT$5index20h11b34fcbe1b5d5d0JVSE.exit.i.i.i" ], [ %oldret22, %"_ZN3str6traits45str.ops..Index$LT$ops..Range$LT$usize$GT$$GT$5index20h11b34fcbe1b5d5d0JVSE.exit539.i.i.i" ]
Instruction does not dominate all uses!
  %1213 = getelementptr inbounds i8, i8* %1194, i64 2
  %oldret20 = insertvalue %str_slice undef, i8* %1213, 0
Instruction does not dominate all uses!
  %1223 = add i64 %1195, -3
  %oldret22 = insertvalue %str_slice %oldret20, i64 %1223, 1
Instruction does not dominate all uses!
  %1199 = getelementptr inbounds i8, i8* %1194, i64 3
  %oldret8 = insertvalue %str_slice undef, i8* %1199, 0
Instruction does not dominate all uses!
  %1209 = add i64 %1195, -4
  %oldret10 = insertvalue %str_slice %oldret8, i64 %1209, 1
LLVM ERROR: Broken function found, compilation aborted!

------------------------------------------

thread '[run-pass] run-pass/sepcomp-lib-lto.rs' panicked at 'explicit panic', C:/bot/slave/auto-win-msvc-64-opt/build/src/compiletest\runtest.rs:1505


failures:
    [run-pass] run-pass/sepcomp-lib-lto.rs
