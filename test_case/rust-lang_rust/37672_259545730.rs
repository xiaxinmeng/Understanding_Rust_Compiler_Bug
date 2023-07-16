
$ clang --target=msp430 -c foo.c -o foo.o -v
clang version 3.8.0-2ubuntu4 (tags/RELEASE_380/final)
Target: msp430
Thread model: posix
InstalledDir: /usr/bin
 "/usr/lib/llvm-3.8/bin/clang" -cc1 -triple msp430 -S -disable-free -disable-llvm-verifier -main-file-name foo.c -mrelocation-model static -mthread-model posix -mdisable-fp-elim -fmath-errno -no-integrated-as -mconstructor-aliases -v -dwarf-column-info -debugger-tuning=gdb -coverage-file /home/japaric/foo.o -resource-dir /usr/lib/llvm-3.8/bin/../lib/clang/3.8.0 -fno-dwarf-directory-asm -fdebug-compilation-dir /home/japaric -ferror-limit 19 -fmessage-length 174 -fobjc-runtime=gcc -fdiagnostics-show-option -fcolor-diagnostics -o /tmp/foo-35f3f8.s -x c foo.c
clang -cc1 version 3.8.0 based upon LLVM 3.8.0 default target x86_64-pc-linux-gnu
#include "..." search starts here:
#include <...> search starts here:
 /usr/local/include
 /usr/lib/llvm-3.8/bin/../lib/clang/3.8.0/include
 /usr/include
End of search list.
 "/usr/bin/msp430-as" -o foo.o /tmp/foo-35f3f8.s <--
