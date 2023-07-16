


---- [run-pass] run-pass/issue-30490.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="x86_64-pc-windows-gnu/stage2/bin;C:\bot\slave\auto-win-gnu-64-nopt-t\build\obj\x86_64-pc-windows-gnu\stage2\bin;C:\bot\slave\auto-win-gnu-64-nopt-t\build\obj\x86_64-pc-windows-gnu\llvm\Release\lib;C:\mingw-w64\x86_64-4.9.1-win32-seh-rt_v3-rev1\mingw64\bin;C:\msys64\usr\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\msys64\usr\bin;C:\python27;C:\python27\scripts;C:\program files (x86)\inno setup 5;C:\program files (x86)\CMake\bin;C:\Program Files (x86)\Windows Kits\8.1\Windows Performance Toolkit;C:\Program Files\Microsoft SQL Server\110\Tools\Binn;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.0;C:\msys64\mingw64\bin" x86_64-pc-windows-gnu/stage2/bin/rustc.exe C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs -L x86_64-pc-windows-gnu/test/run-pass/ --target=x86_64-pc-windows-gnu -L x86_64-pc-windows-gnu/test/run-pass\issue-30490.stage2-x86_64-pc-windows-gnu.run-pass.libaux -C prefer-dynamic -o x86_64-pc-windows-gnu/test/run-pass\issue-30490.stage2-x86_64-pc-windows-gnu.exe --cfg rtopt -L x86_64-pc-windows-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs:102:26: 102:36 error: failed to resolve. Use of undeclared type or module `libc` [E0433]
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs:102      assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
                                                                                                        ^~~~~~~~~~
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs:102:6: 102:61 note: in this expansion of assert_eq! (defined in <std macros>)
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs:102:26: 102:36 help: run `rustc --explain E0433` to see a detailed explanation
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs:102:26: 102:36 error: unresolved name `libc::pipe` [E0425]
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs:102      assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
                                                                                                        ^~~~~~~~~~
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs:102:6: 102:61 note: in this expansion of assert_eq! (defined in <std macros>)
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs:102:26: 102:36 help: run `rustc --explain E0425` to see a detailed explanation
<std macros>:5:8: 5:18 error: the type of this value must be known in this context
<std macros>:5 if ! ( * left_val == * right_val ) {
                      ^~~~~~~~~~
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs:102:6: 102:61 note: in this expansion of assert_eq! (defined in <std macros>)
<std macros>:5:22: 5:33 error: the type of this value must be known in this context
<std macros>:5 if ! ( * left_val == * right_val ) {
                                    ^~~~~~~~~~~
C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/test/run-pass/issue-30490.rs:102:6: 102:61 note: in this expansion of assert_eq! (defined in <std macros>)
error: aborting due to 4 previous errors

------------------------------------------

thread '[run-pass] run-pass/issue-30490.rs' panicked at 'explicit panic', C:/bot/slave/auto-win-gnu-64-nopt-t/build/src/compiletest\runtest.rs:1505
