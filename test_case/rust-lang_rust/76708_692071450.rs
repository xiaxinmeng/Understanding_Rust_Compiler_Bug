plain
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\llvm-finished-building`
llvm-config: error: component libraries and shared library

llvm-config: error: missing: D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\gtest.lib
llvm-config: error: missing: D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\gtest_main.lib
llvm-config: error: missing: D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMTestingSupport.lib
thread 'main' panicked at 'command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\llvm-config.exe" "--libfiles"
expected success, got: exit code: 1', src\build_helper\lib.rs:139:9
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 0:16:12
== clock drift check ==
  local time: Mon Sep 14 13:59:28 CUT 2020
  local time: Mon Sep 14 13:59:28 CUT 2020
  network time: Mon, 14 Sep 2020 13:59:28 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (5908) (python)
Terminate orphan process: pid (7528) (sccache)
