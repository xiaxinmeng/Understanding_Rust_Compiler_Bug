bash
% clang hello.c
Stack dump:
0.      Program arguments: /home/lzutao/.local/bin/clang-8 -cc1 -triple x86_64-unknown-linux-gnu -emit-obj -mrelax-all -disable-free -disable-llvm-verifier -discard-value-names -main-file-name hello.c -mrelocation-model static -mthread-model posix -mdisable-fp-elim -fmat
h-errno -masm-verbose -mconstructor-aliases -munwind-tables -fuse-init-array -target-cpu x86-64 -dwarf-column-info -debugger-tuning=gdb -resource-dir /home/lzutao/.local/lib/clang/8.0.0 -internal-isystem /usr/local/include -internal-isystem /home/lzutao/.local/lib/clang/
8.0.0/include -internal-externc-isystem /home/lzutao/.local/include -internal-externc-isystem /usr/lib/gcc/x86_64-linux-gnu/6/include-fixed -internal-externc-isystem /usr/include -internal-externc-isystem /usr/local/include -internal-externc-isystem /usr/include/x86_64-l
inux-gnu -fdebug-compilation-dir /home/lzutao/forked/rust/check -ferror-limit 19 -fmessage-length 135 -fobjc-runtime=gcc -fdiagnostics-show-option -fcolor-diagnostics -o /tmp/hello-27434e.o -x c hello.c -faddrsig
1.      clang-8: error: unable to execute command: Alarm clock
clang-8: error: clang frontend command failed due to signal (use -v to see invocation)
clang version 8.0.0
Target: x86_64-unknown-linux-gnu
Thread model: posix
InstalledDir: /home/lzutao/.local/bin
clang-8: note: diagnostic msg: PLEASE submit a bug report to https://bugs.llvm.org/ and include the crash backtrace, preprocessed source, and associated run script.
clang-8: note: diagnostic msg:
********************

PLEASE ATTACH THE FOLLOWING FILES TO THE BUG REPORT:
Preprocessed source(s) and associated run script(s) are located at:
clang-8: note: diagnostic msg: /tmp/hello-943593.c
clang-8: note: diagnostic msg: /tmp/hello-943593.sh
clang-8: note: diagnostic msg:

********************
