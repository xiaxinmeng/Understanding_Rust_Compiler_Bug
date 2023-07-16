
----- /c/bot/slave/auto-win-gnu-64-opt/build/src/test/run-make/llvm-module-pass/ --------------------
------ stdout ---------------------------------------------
make[1]: Entering directory '/c/bot/slave/auto-win-gnu-64-opt/build/src/test/run-make/llvm-module-pass'
g++ -IC:\bot\slave\auto-win-gnu-64-opt\build\obj\x86_64-pc-windows-gnu\llvm\Release/include  -DNDEBUG -D__NO_CTYPE_INLINE -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -O2 -fomit-frame-pointer -std=gnu++11 -fvisibility-inlines-hidden -fno-exceptions -fno-rtti -Wcast-qual -c llvm-pass.so.cc -o /c/bot/slave/auto-win-gnu-64-opt/build/obj/x86_64-pc-windows-gnu/test/run-make/llvm-module-pass/libllvm-pass.o
Makefile:8: recipe for target '/c/bot/slave/auto-win-gnu-64-opt/build/obj/x86_64-pc-windows-gnu/test/run-make/llvm-module-pass/libllvm-pass.o' failed
make[1]: Leaving directory '/c/bot/slave/auto-win-gnu-64-opt/build/src/test/run-make/llvm-module-pass'

------ stderr ---------------------------------------------
llvm-pass.so.cc:15:28: fatal error: llvm/IR/Module.h: No such file or directory
 #include "llvm/IR/Module.h"
                            ^
compilation terminated.
make[1]: *** [/c/bot/slave/auto-win-gnu-64-opt/build/obj/x86_64-pc-windows-gnu/test/run-make/llvm-module-pass/libllvm-pass.o] Error 1

------        ---------------------------------------------

/c/bot/slave/auto-win-gnu-64-opt/build/mk/tests.mk:1064: recipe for target 'x86_64-pc-windows-gnu/test/run-make/llvm-module-pass-2-T-x86_64-pc-windows-gnu-H-x86_64-pc-windows-gnu.ok' failed
Zombie process: "\Device\HarddiskVolume1\dojob.exe"
make: *** [x86_64-pc-windows-gnu/test/run-make/llvm-module-pass-2-T-x86_64-pc-windows-gnu-H-x86_64-pc-windows-gnu.ok] Error 2
program finished with exit code 2
elapsedTime=655.183000
