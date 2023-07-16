
llvm[3]: ======= Finished Linking Release+Asserts Executable llvm-config (without symbols)
make[3]: Leaving directory `/home/steve/src/rust/x86_64-unknown-linux-gnu/llvm/tools/llvm-config'
make[3]: Entering directory `/home/steve/src/rust/x86_64-unknown-linux-gnu/llvm/tools/bugpoint'
llvm[3]: Compiling BugDriver.cpp for Release+Asserts build
llvm[3]: Compiling CrashDebugger.cpp for Release+Asserts build
llvm[3]: Compiling ExecutionDriver.cpp for Release+Asserts build
llvm[3]: Compiling ExtractFunction.cpp for Release+Asserts build
llvm[3]: Compiling FindBugs.cpp for Release+Asserts build
llvm[3]: Compiling Miscompilation.cpp for Release+Asserts build
llvm[3]: Compiling OptimizerDriver.cpp for Release+Asserts build
llvm[3]: Compiling ToolRunner.cpp for Release+Asserts build
llvm[3]: Compiling bugpoint.cpp for Release+Asserts build
llvm[3]: Linking Release+Asserts executable bugpoint (without symbols)
/home/steve/src/rust/x86_64-unknown-linux-gnu/llvm/tools/bugpoint/Release+Asserts/bugpoint.o: In function `_GLOBAL__sub_I_bugpoint.cpp':
bugpoint.cpp:(.text.startup._GLOBAL__sub_I_bugpoint.cpp+0xc71): undefined reference to `llvm::createJumpInstrTablesPass()'
bugpoint.cpp:(.text.startup._GLOBAL__sub_I_bugpoint.cpp+0xd8b): undefined reference to `llvm::createCodeGenPreparePass(llvm::TargetMachine const*)'
collect2: error: ld returned 1 exit status
make[3]: *** [/home/steve/src/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/bin/bugpoint] Error 1
make[3]: Leaving directory `/home/steve/src/rust/x86_64-unknown-linux-gnu/llvm/tools/bugpoint'
make[2]: *** [bugpoint/.makeall] Error 2
make[2]: Leaving directory `/home/steve/src/rust/x86_64-unknown-linux-gnu/llvm/tools'
make[1]: *** [all] Error 1
make[1]: Leaving directory `/home/steve/src/rust/x86_64-unknown-linux-gnu/llvm'
make: *** [/home/steve/src/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/bin/llvm-config] Error 2

