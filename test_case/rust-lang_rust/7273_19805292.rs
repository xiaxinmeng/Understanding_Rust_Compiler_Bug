
[...]
tail -n +2 /c/bot/slave/auto-win-32-opt/build/src/llvm/unittests/ExecutionEngine/JIT/JITTests.def > /c/bot/slave/auto-win-32-opt/build/obj/llvm/i686-pc-mingw32/unittests/ExecutionEngine/JIT/JITTests.exports
make[4]: Leaving directory `/c/bot/slave/auto-win-32-opt/build/obj/llvm/i686-pc-mingw32/unittests/ExecutionEngine/JIT'
make[3]: Leaving directory `/c/bot/slave/auto-win-32-opt/build/obj/llvm/i686-pc-mingw32/unittests/ExecutionEngine'
make[2]: Leaving directory `/c/bot/slave/auto-win-32-opt/build/obj/llvm/i686-pc-mingw32/unittests'
make[1]: Leaving directory `/c/bot/slave/auto-win-32-opt/build/obj/llvm/i686-pc-mingw32'
TAIL: can't open +2
make[4]: *** [/c/bot/slave/auto-win-32-opt/build/obj/llvm/i686-pc-mingw32/unittests/ExecutionEngine/JIT/JITTests.exports] Error 2
make[3]: *** [JIT/.makeall] Error 2
make[2]: *** [ExecutionEngine/.makeall] Error 2
make[1]: *** [all] Error 1
make: *** [/c/bot/slave/auto-win-32-opt/build/obj/llvm/i686-pc-mingw32/Release+Asserts/bin/llvm-config.exe] Error 2
program finished with exit code 2
elapsedTime=1155.463000
