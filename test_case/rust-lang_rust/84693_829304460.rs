plain
[3693/4641] Building CXX object tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGAtomic.cpp.o
[3694/4641] Building CXX object tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGCUDANV.cpp.o
[3695/4641] Building CXX object tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGBuiltin.cpp.o
FAILED: tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGBuiltin.cpp.o 
sccache /usr/bin/clang++  -DGTEST_HAS_RTTI=0 -D_DEBUG -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Itools/clang/lib/CodeGen -I/checkout/src/llvm-project/clang/lib/CodeGen -I/checkout/src/llvm-project/clang/include -Itools/clang/include -Iinclude -I/checkout/src/llvm-project/llvm/include -ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fPIC -fvisibility-inlines-hidden -Werror=date-time -Werror=unguarded-availability-new -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -pedantic -Wno-long-long -Wimplicit-fallthrough -Wcovered-switch-default -Wno-noexcept-type -Wnon-virtual-dtor -Wdelete-non-virtual-dtor -Wstring-conversion -fdiagnostics-color -ffunction-sections -fdata-sections -fno-common -Woverloaded-virtual -Wno-nested-anon-types -O3     -fno-exceptions -fno-rtti -UNDEBUG -std=c++14 -MD -MT tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGBuiltin.cpp.o -MF tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGBuiltin.cpp.o.d -o tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGBuiltin.cpp.o -c /checkout/src/llvm-project/clang/lib/CodeGen/CGBuiltin.cpp
/checkout/src/llvm-project/clang/lib/CodeGen/CGBuiltin.cpp:17175:15: error: no member named 'getWithNewType' in 'llvm::Type'
        SrcT->getWithNewType(llvm::IntegerType::get(getLLVMContext(), 32));
1 error generated.
[3696/4641] Building CXX object tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGCleanup.cpp.o
[3697/4641] Building CXX object tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGCoroutine.cpp.o
[3698/4641] Building CXX object tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGBlocks.cpp.o
---
[3707/4641] Building CXX object tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGDebugInfo.cpp.o
[3708/4641] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilder.cpp.o
[3709/4641] Building CXX object tools/clang/lib/CodeGen/CMakeFiles/obj.clangCodeGen.dir/CGExpr.cpp.o
[3710/4641] Building CXX object tools/clang/lib/Sema/CMakeFiles/obj.clangSema.dir/SemaDeclAttr.cpp.o
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1

 finished in 557.232 seconds
 finished in 557.232 seconds
build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 build
Build completed unsuccessfully in 0:10:45
