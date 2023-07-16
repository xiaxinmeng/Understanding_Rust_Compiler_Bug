plain
2020-02-06T11:31:05.7499270Z -- Looking for arc4random - found
2020-02-06T11:31:05.8251150Z -- Looking for backtrace
2020-02-06T11:31:06.0122910Z -- Looking for backtrace - found
2020-02-06T11:31:06.0124540Z -- backtrace facility detected in default set of libraries
2020-02-06T11:31:06.0133120Z -- Found Backtrace: /Applications/Xcode_11.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk/usr/include  
2020-02-06T11:31:06.1913270Z -- Performing Test C_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW - Success
2020-02-06T11:31:06.1926730Z -- Looking for _Unwind_Backtrace
2020-02-06T11:31:06.3745950Z -- Looking for _Unwind_Backtrace - found
2020-02-06T11:31:06.3759530Z -- Looking for getpagesize
---
2020-02-06T11:51:29.1177470Z -- Detecting C compiler ABI info - done
2020-02-06T11:51:29.1310340Z -- Detecting C compile features
2020-02-06T11:51:29.1317300Z -- Detecting C compile features - done
2020-02-06T11:51:29.1388620Z -- Check for working CXX compiler: /Users/runner/runners/2.164.7/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++
2020-02-06T11:51:29.2876950Z -- Check for working CXX compiler: /Users/runner/runners/2.164.7/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++ -- broken
2020-02-06T11:51:29.2878720Z CMake Error at /usr/local/Cellar/cmake/3.16.2/share/cmake/Modules/CMakeTestCXXCompiler.cmake:53 (message):
2020-02-06T11:51:29.2879260Z 
2020-02-06T11:51:29.2879950Z     "/Users/runner/runners/2.164.7/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++"
2020-02-06T11:51:29.2880090Z 
2020-02-06T11:51:29.2880180Z   is not able to compile a simple test program.
2020-02-06T11:51:29.2880180Z   is not able to compile a simple test program.
2020-02-06T11:51:29.2880230Z 
2020-02-06T11:51:29.2880300Z   It fails with the following output:
2020-02-06T11:51:29.2880340Z 
2020-02-06T11:51:29.2880890Z     Change Dir: /Users/runner/runners/2.164.7/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeTmp
2020-02-06T11:51:29.2881000Z     
2020-02-06T11:51:29.2881590Z     Run Build Command(s):/usr/bin/make cmTC_a43a4/fast && /Applications/Xcode_11.2.1.app/Contents/Developer/usr/bin/make -f CMakeFiles/cmTC_a43a4.dir/build.make CMakeFiles/cmTC_a43a4.dir/build
2020-02-06T11:51:29.2881740Z     Building CXX object CMakeFiles/cmTC_a43a4.dir/testCXXCompiler.cxx.o
2020-02-06T11:51:29.2882680Z     /Users/runner/runners/2.164.7/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++    -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin  -isysroot /Applications/Xcode_11.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk -mmacosx-version-min=10.7   -o CMakeFiles/cmTC_a43a4.dir/testCXXCompiler.cxx.o -c /Users/runner/runners/2.164.7/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeTmp/testCXXCompiler.cxx
2020-02-06T11:51:29.2883760Z     clang-9: warning: include path for libstdc++ headers not found; pass '-stdlib=libc++' on the command line to use the libc++ standard library instead [-Wstdlibcxx-not-found]
2020-02-06T11:51:29.2883900Z     Linking CXX executable cmTC_a43a4
2020-02-06T11:51:29.2884400Z     /usr/local/Cellar/cmake/3.16.2/bin/cmake -E cmake_link_script CMakeFiles/cmTC_a43a4.dir/link.txt --verbose=1
2020-02-06T11:51:29.2885290Z     /Users/runner/runners/2.164.7/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++   -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin  -isysroot /Applications/Xcode_11.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk -mmacosx-version-min=10.7 -Wl,-search_paths_first -Wl,-headerpad_max_install_names   CMakeFiles/cmTC_a43a4.dir/testCXXCompiler.cxx.o  -o cmTC_a43a4 
2020-02-06T11:51:29.2885860Z     ld: library not found for -lstdc++
2020-02-06T11:51:29.2886310Z     clang-9: error: linker command failed with exit code 1 (use -v to see invocation)
2020-02-06T11:51:29.2886430Z     make[1]: *** [cmTC_a43a4] Error 1
2020-02-06T11:51:29.2886510Z     make: *** [cmTC_a43a4/fast] Error 2
2020-02-06T11:51:29.2886640Z     
2020-02-06T11:51:29.2886680Z 
2020-02-06T11:51:29.2886730Z   
2020-02-06T11:51:29.2886780Z 
2020-02-06T11:51:29.2886780Z 
2020-02-06T11:51:29.2886850Z   CMake will not be able to correctly generate this project.
2020-02-06T11:51:29.2887370Z Call Stack (most recent call first):
2020-02-06T11:51:29.2887480Z   CMakeLists.txt:14 (project)
2020-02-06T11:51:29.2887540Z 
2020-02-06T11:51:29.2887570Z 
2020-02-06T11:51:29.2923760Z -- Configuring incomplete, errors occurred!
2020-02-06T11:51:29.2924330Z See also "/Users/runner/runners/2.164.7/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeOutput.log".
2020-02-06T11:51:29.2924970Z See also "/Users/runner/runners/2.164.7/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeError.log".
2020-02-06T11:51:29.2969040Z command did not execute successfully, got: exit code: 1
2020-02-06T11:51:29.2969120Z 
2020-02-06T11:51:29.2969120Z 
2020-02-06T11:51:29.2969900Z build script failed, must exit now', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.42/src/lib.rs:861:5
2020-02-06T11:51:29.2971190Z  finished in 0.892
2020-02-06T11:51:29.2986690Z failed to run: /Users/runner/runners/2.164.7/work/1/s/build/bootstrap/debug/bootstrap dist
2020-02-06T11:51:29.2986840Z Build completed unsuccessfully in 0:30:47
2020-02-06T11:51:29.3037340Z == clock drift check ==
2020-02-06T11:51:29.3037340Z == clock drift check ==
2020-02-06T11:51:29.3084440Z   local time: Thu Feb  6 11:51:29 UTC 2020
2020-02-06T11:51:29.3407790Z   network time: Thu, 06 Feb 2020 11:51:29 GMT
2020-02-06T11:51:29.3413870Z == end clock drift check ==
2020-02-06T11:51:29.3508890Z 
2020-02-06T11:51:29.3612100Z ##[error]Bash exited with code '1'.
2020-02-06T11:51:29.3660350Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-02-06T11:51:29.3663430Z ==============================================================================
2020-02-06T11:51:29.3663540Z Task         : Get sources
2020-02-06T11:51:29.3663610Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
