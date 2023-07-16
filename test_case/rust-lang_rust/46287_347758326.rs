rust
[00:12:31] In file included from /Users/travis/build/rust-lang/rust/src/llvm/lib/Support/ThreadPool.cpp:14:
[00:12:31] In file included from /Users/travis/build/rust-lang/rust/src/llvm/include/llvm/Support/ThreadPool.h:30:
[00:12:31] /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/../include/c++/v1/future:1447:23: error: 'future_error' is unavailable: introduced in macOS 10.8
[00:12:31]                       future_error(make_error_code(future_errc::broken_promise))
[00:12:31]                       ^
[00:12:31] /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/../include/c++/v1/future:502:63: note: 'future_error' has been explicitly marked unavailable here
[00:12:31] class _LIBCPP_EXCEPTION_ABI _LIBCPP_AVAILABILITY_FUTURE_ERROR future_error
[00:12:31]                                                               ^
[00:12:31] /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/../include/c++/v1/future:1621:23: error: 'future_error' is unavailable: introduced in macOS 10.8
[00:12:31]                       future_error(make_error_code(future_errc::broken_promise))
[00:12:31]                       ^
[00:12:31] /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/../include/c++/v1/future:502:63: note: 'future_error' has been explicitly marked unavailable here
[00:12:31] class _LIBCPP_EXCEPTION_ABI _LIBCPP_AVAILABILITY_FUTURE_ERROR future_error
[00:12:31]                                                               ^
[00:12:31] 2 errors generated.
[00:12:31] make[3]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/ThreadPool.cpp.o] Error 1
[00:12:31] make[2]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/all] Error 2
