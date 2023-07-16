plain
2020-02-12T18:01:24.6321740Z -- Looking for arc4random - found
2020-02-12T18:01:24.7112990Z -- Looking for backtrace
2020-02-12T18:01:24.9086040Z -- Looking for backtrace - found
2020-02-12T18:01:24.9088690Z -- backtrace facility detected in default set of libraries
2020-02-12T18:01:24.9093740Z -- Found Backtrace: /Applications/Xcode_11.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk/usr/include  
2020-02-12T18:01:25.0902540Z -- Performing Test C_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW - Success
2020-02-12T18:01:25.0916880Z -- Looking for _Unwind_Backtrace
2020-02-12T18:01:25.2723060Z -- Looking for _Unwind_Backtrace - found
2020-02-12T18:01:25.2735460Z -- Looking for getpagesize
---
2020-02-12T19:03:00.9022520Z -- Detecting C compiler ABI info - done
2020-02-12T19:03:00.9170850Z -- Detecting C compile features
2020-02-12T19:03:00.9179300Z -- Detecting C compile features - done
2020-02-12T19:03:00.9268560Z -- Check for working CXX compiler: /Users/runner/runners/2.164.8/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++
2020-02-12T19:03:01.0839170Z -- Check for working CXX compiler: /Users/runner/runners/2.164.8/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++ -- broken
2020-02-12T19:03:01.0841210Z CMake Error at /usr/local/Cellar/cmake/3.16.2/share/cmake/Modules/CMakeTestCXXCompiler.cmake:53 (message):
2020-02-12T19:03:01.0841600Z 
2020-02-12T19:03:01.0842240Z     "/Users/runner/runners/2.164.8/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++"
2020-02-12T19:03:01.0842340Z 
2020-02-12T19:03:01.0842430Z   is not able to compile a simple test program.
2020-02-12T19:03:01.0842430Z   is not able to compile a simple test program.
2020-02-12T19:03:01.0842560Z 
2020-02-12T19:03:01.0842620Z   It fails with the following output:
2020-02-12T19:03:01.0842720Z 
2020-02-12T19:03:01.0843600Z     Change Dir: /Users/runner/runners/2.164.8/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeTmp
2020-02-12T19:03:01.0843760Z     
2020-02-12T19:03:01.0844840Z     Run Build Command(s):/usr/bin/make cmTC_ab8bb/fast && /Applications/Xcode_11.2.1.app/Contents/Developer/usr/bin/make -f CMakeFiles/cmTC_ab8bb.dir/build.make CMakeFiles/cmTC_ab8bb.dir/build
2020-02-12T19:03:01.0845000Z     Building CXX object CMakeFiles/cmTC_ab8bb.dir/testCXXCompiler.cxx.o
2020-02-12T19:03:01.0845930Z     /Users/runner/runners/2.164.8/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++    -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin  -isysroot /Applications/Xcode_11.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk -mmacosx-version-min=10.7   -o CMakeFiles/cmTC_ab8bb.dir/testCXXCompiler.cxx.o -c /Users/runner/runners/2.164.8/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeTmp/testCXXCompiler.cxx
2020-02-12T19:03:01.0846670Z     clang-9: warning: include path for libstdc++ headers not found; pass '-stdlib=libc++' on the command line to use the libc++ standard library instead [-Wstdlibcxx-not-found]
2020-02-12T19:03:01.0846820Z     Linking CXX executable cmTC_ab8bb
2020-02-12T19:03:01.0847330Z     /usr/local/Cellar/cmake/3.16.2/bin/cmake -E cmake_link_script CMakeFiles/cmTC_ab8bb.dir/link.txt --verbose=1
2020-02-12T19:03:01.0848190Z     /Users/runner/runners/2.164.8/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++   -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin  -isysroot /Applications/Xcode_11.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk -mmacosx-version-min=10.7 -Wl,-search_paths_first -Wl,-headerpad_max_install_names   CMakeFiles/cmTC_ab8bb.dir/testCXXCompiler.cxx.o  -o cmTC_ab8bb 
2020-02-12T19:03:01.0848750Z     ld: library not found for -lstdc++
2020-02-12T19:03:01.0849220Z     clang-9: error: linker command failed with exit code 1 (use -v to see invocation)
2020-02-12T19:03:01.0849320Z     make[1]: *** [cmTC_ab8bb] Error 1
2020-02-12T19:03:01.0849400Z     make: *** [cmTC_ab8bb/fast] Error 2
2020-02-12T19:03:01.0849550Z     
2020-02-12T19:03:01.0849580Z 
2020-02-12T19:03:01.0849650Z   
2020-02-12T19:03:01.0849680Z 
2020-02-12T19:03:01.0849680Z 
2020-02-12T19:03:01.0849760Z   CMake will not be able to correctly generate this project.
2020-02-12T19:03:01.0849830Z Call Stack (most recent call first):
2020-02-12T19:03:01.0849900Z   CMakeLists.txt:14 (project)
2020-02-12T19:03:01.0849950Z 
2020-02-12T19:03:01.0849980Z 
2020-02-12T19:03:01.0884370Z -- Configuring incomplete, errors occurred!
2020-02-12T19:03:01.0885220Z See also "/Users/runner/runners/2.164.8/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeOutput.log".
2020-02-12T19:03:01.0885880Z See also "/Users/runner/runners/2.164.8/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeError.log".
2020-02-12T19:03:01.0924930Z command did not execute successfully, got: exit code: 1
2020-02-12T19:03:01.0925000Z 
2020-02-12T19:03:01.0925000Z 
2020-02-12T19:03:01.0925540Z build script failed, must exit now', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.42/src/lib.rs:861:5
2020-02-12T19:03:01.0925860Z  finished in 0.953
2020-02-12T19:03:01.0944390Z failed to run: /Users/runner/runners/2.164.8/work/1/s/build/bootstrap/debug/bootstrap dist
2020-02-12T19:03:01.0944670Z Build completed unsuccessfully in 1:12:15
2020-02-12T19:03:01.0994390Z == clock drift check ==
2020-02-12T19:03:01.0994390Z == clock drift check ==
2020-02-12T19:03:01.1036680Z   local time: Wed Feb 12 19:03:01 UTC 2020
2020-02-12T19:03:01.1522770Z   network time: Wed, 12 Feb 2020 19:03:01 GMT
2020-02-12T19:03:01.1524230Z == end clock drift check ==
2020-02-12T19:03:01.1564750Z 
2020-02-12T19:03:01.1672200Z ##[error]Bash exited with code '1'.
2020-02-12T19:03:01.1714490Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-02-12T19:03:01.1717480Z ==============================================================================
2020-02-12T19:03:01.1717780Z Task         : Get sources
2020-02-12T19:03:01.1717870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
