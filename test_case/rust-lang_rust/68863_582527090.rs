plain
2020-02-05T16:01:48.9204160Z -- Looking for arc4random - found
2020-02-05T16:01:49.0047570Z -- Looking for backtrace
2020-02-05T16:01:49.1913070Z -- Looking for backtrace - found
2020-02-05T16:01:49.1916320Z -- backtrace facility detected in default set of libraries
2020-02-05T16:01:49.1923330Z -- Found Backtrace: /Applications/Xcode_11.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk/usr/include  
2020-02-05T16:01:49.3706430Z -- Performing Test C_SUPPORTS_WERROR_UNGUARDED_AVAILABILITY_NEW - Success
2020-02-05T16:01:49.3727350Z -- Looking for _Unwind_Backtrace
2020-02-05T16:01:49.5587470Z -- Looking for _Unwind_Backtrace - found
2020-02-05T16:01:49.5606660Z -- Looking for getpagesize
---
2020-02-05T17:02:56.6294000Z -- Detecting C compiler ABI info - done
2020-02-05T17:02:56.6448540Z -- Detecting C compile features
2020-02-05T17:02:56.6455600Z -- Detecting C compile features - done
2020-02-05T17:02:56.6534020Z -- Check for working CXX compiler: /Users/runner/runners/2.164.7/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++
2020-02-05T17:02:56.8056580Z -- Check for working CXX compiler: /Users/runner/runners/2.164.7/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++ -- broken
2020-02-05T17:02:56.8058870Z CMake Error at /usr/local/Cellar/cmake/3.16.2/share/cmake/Modules/CMakeTestCXXCompiler.cmake:53 (message):
2020-02-05T17:02:56.8059540Z 
2020-02-05T17:02:56.8060810Z     "/Users/runner/runners/2.164.7/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++"
2020-02-05T17:02:56.8060990Z 
2020-02-05T17:02:56.8061080Z   is not able to compile a simple test program.
2020-02-05T17:02:56.8061080Z   is not able to compile a simple test program.
2020-02-05T17:02:56.8061140Z 
2020-02-05T17:02:56.8061200Z   It fails with the following output:
2020-02-05T17:02:56.8061260Z 
2020-02-05T17:02:56.8061830Z     Change Dir: /Users/runner/runners/2.164.7/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeTmp
2020-02-05T17:02:56.8061950Z     
2020-02-05T17:02:56.8062520Z     Run Build Command(s):/usr/bin/make cmTC_47b7c/fast && /Applications/Xcode_11.2.1.app/Contents/Developer/usr/bin/make -f CMakeFiles/cmTC_47b7c.dir/build.make CMakeFiles/cmTC_47b7c.dir/build
2020-02-05T17:02:56.8062680Z     Building CXX object CMakeFiles/cmTC_47b7c.dir/testCXXCompiler.cxx.o
2020-02-05T17:02:56.8063610Z     /Users/runner/runners/2.164.7/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++    -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin  -isysroot /Applications/Xcode_11.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk -mmacosx-version-min=10.7   -o CMakeFiles/cmTC_47b7c.dir/testCXXCompiler.cxx.o -c /Users/runner/runners/2.164.7/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeTmp/testCXXCompiler.cxx
2020-02-05T17:02:56.8064810Z     clang-9: warning: include path for libstdc++ headers not found; pass '-stdlib=libc++' on the command line to use the libc++ standard library instead [-Wstdlibcxx-not-found]
2020-02-05T17:02:56.8064940Z     Linking CXX executable cmTC_47b7c
2020-02-05T17:02:56.8065450Z     /usr/local/Cellar/cmake/3.16.2/bin/cmake -E cmake_link_script CMakeFiles/cmTC_47b7c.dir/link.txt --verbose=1
2020-02-05T17:02:56.8066310Z     /Users/runner/runners/2.164.7/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++   -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin  -isysroot /Applications/Xcode_11.2.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk -mmacosx-version-min=10.7 -Wl,-search_paths_first -Wl,-headerpad_max_install_names   CMakeFiles/cmTC_47b7c.dir/testCXXCompiler.cxx.o  -o cmTC_47b7c 
2020-02-05T17:02:56.8066860Z     ld: library not found for -lstdc++
2020-02-05T17:02:56.8067320Z     clang-9: error: linker command failed with exit code 1 (use -v to see invocation)
2020-02-05T17:02:56.8067440Z     make[1]: *** [cmTC_47b7c] Error 1
2020-02-05T17:02:56.8067510Z     make: *** [cmTC_47b7c/fast] Error 2
2020-02-05T17:02:56.8067640Z     
2020-02-05T17:02:56.8067680Z 
2020-02-05T17:02:56.8067750Z   
2020-02-05T17:02:56.8067780Z 
2020-02-05T17:02:56.8067780Z 
2020-02-05T17:02:56.8067860Z   CMake will not be able to correctly generate this project.
2020-02-05T17:02:56.8067940Z Call Stack (most recent call first):
2020-02-05T17:02:56.8068010Z   CMakeLists.txt:14 (project)
2020-02-05T17:02:56.8068050Z 
2020-02-05T17:02:56.8068090Z 
2020-02-05T17:02:56.8103700Z -- Configuring incomplete, errors occurred!
2020-02-05T17:02:56.8104320Z See also "/Users/runner/runners/2.164.7/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeOutput.log".
2020-02-05T17:02:56.8105100Z See also "/Users/runner/runners/2.164.7/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeError.log".
2020-02-05T17:02:56.8145070Z command did not execute successfully, got: exit code: 1
2020-02-05T17:02:56.8145150Z 
2020-02-05T17:02:56.8145150Z 
2020-02-05T17:02:56.8145680Z build script failed, must exit now', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.42/src/lib.rs:861:5
2020-02-05T17:02:56.8146980Z  finished in 1.008
2020-02-05T17:02:56.8164940Z failed to run: /Users/runner/runners/2.164.7/work/1/s/build/bootstrap/debug/bootstrap dist
2020-02-05T17:02:56.8165170Z Build completed unsuccessfully in 1:11:37
2020-02-05T17:02:56.8215660Z == clock drift check ==
2020-02-05T17:02:56.8215660Z == clock drift check ==
2020-02-05T17:02:56.8243110Z   local time: Wed Feb  5 17:02:56 UTC 2020
2020-02-05T17:02:56.8889380Z   network time: Wed, 05 Feb 2020 17:02:56 GMT
2020-02-05T17:02:56.8891540Z == end clock drift check ==
2020-02-05T17:02:56.8932690Z 
2020-02-05T17:02:56.9030100Z ##[error]Bash exited with code '1'.
2020-02-05T17:02:56.9074930Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-02-05T17:02:56.9077750Z ==============================================================================
2020-02-05T17:02:56.9077850Z Task         : Get sources
2020-02-05T17:02:56.9077930Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
