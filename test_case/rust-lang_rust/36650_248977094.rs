
[ 58%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TargetRegisterInfo.cpp.o
In file included from /Users/jturner/Source/rust/src/llvm/lib/ExecutionEngine/Orc/OrcRemoteTargetRPCAPI.cpp:10:
In file included from /Users/jturner/Source/rust/src/llvm/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetRPCAPI.h:21:
In file included from /Users/jturner/Source/rust/src/llvm/include/llvm/ExecutionEngine/Orc/RPCUtils.h:35:
/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/../include/c++/v1/future:1447:23: error: 'future_error' is unavailable: introduced in macOS 10.8
                      future_error(make_error_code(future_errc::broken_promise))
                      ^
/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/../include/c++/v1/future:502:63: note: 'future_error' has been explicitly marked unavailable here
class _LIBCPP_EXCEPTION_ABI _LIBCPP_AVAILABILITY_FUTURE_ERROR future_error
                                                              ^
/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/../include/c++/v1/future:1621:23: error: 'future_error' is unavailable: introduced in macOS 10.8
                      future_error(make_error_code(future_errc::broken_promise))
                      ^
/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/../include/c++/v1/future:502:63: note: 'future_error' has been explicitly marked unavailable here
class _LIBCPP_EXCEPTION_ABI _LIBCPP_AVAILABILITY_FUTURE_ERROR future_error
                                                              ^
2 errors generated.
make[2]: *** [lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/OrcRemoteTargetRPCAPI.cpp.o] Error 1
make[1]: *** [lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
