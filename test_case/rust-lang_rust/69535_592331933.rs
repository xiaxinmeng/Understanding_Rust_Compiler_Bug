plain
2020-02-28T06:12:36.9096280Z -- Detecting C compiler ABI info - done
2020-02-28T06:12:36.9225550Z -- Detecting C compile features
2020-02-28T06:12:36.9232460Z -- Detecting C compile features - done
2020-02-28T06:12:36.9311740Z -- Check for working CXX compiler: /Users/runner/runners/2.165.0/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++
2020-02-28T06:12:37.0775810Z -- Check for working CXX compiler: /Users/runner/runners/2.165.0/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++ -- broken
2020-02-28T06:12:37.0778050Z CMake Error at /usr/local/Cellar/cmake/3.16.4/share/cmake/Modules/CMakeTestCXXCompiler.cmake:53 (message):
2020-02-28T06:12:37.0778690Z 
2020-02-28T06:12:37.0779900Z     "/Users/runner/runners/2.165.0/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++"
2020-02-28T06:12:37.0780270Z 
2020-02-28T06:12:37.0780510Z   is not able to compile a simple test program.
2020-02-28T06:12:37.0780510Z   is not able to compile a simple test program.
2020-02-28T06:12:37.0780810Z 
2020-02-28T06:12:37.0781030Z   It fails with the following output:
2020-02-28T06:12:37.0781220Z 
2020-02-28T06:12:37.0782160Z     Change Dir: /Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeTmp
2020-02-28T06:12:37.0782610Z     
2020-02-28T06:12:37.0783620Z     Run Build Command(s):/usr/bin/make cmTC_a82d2/fast && /Applications/Xcode_11.3.1.app/Contents/Developer/usr/bin/make -f CMakeFiles/cmTC_a82d2.dir/build.make CMakeFiles/cmTC_a82d2.dir/build
2020-02-28T06:12:37.0784340Z     Building CXX object CMakeFiles/cmTC_a82d2.dir/testCXXCompiler.cxx.o
2020-02-28T06:12:37.0786350Z     /Users/runner/runners/2.165.0/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++    -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin  -isysroot /Applications/Xcode_11.3.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk -mmacosx-version-min=10.8   -o CMakeFiles/cmTC_a82d2.dir/testCXXCompiler.cxx.o -c /Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeTmp/testCXXCompiler.cxx
2020-02-28T06:12:37.0788340Z     clang-9: warning: include path for libstdc++ headers not found; pass '-stdlib=libc++' on the command line to use the libc++ standard library instead [-Wstdlibcxx-not-found]
2020-02-28T06:12:37.0788940Z     Linking CXX executable cmTC_a82d2
2020-02-28T06:12:37.0789880Z     /usr/local/Cellar/cmake/3.16.4/bin/cmake -E cmake_link_script CMakeFiles/cmTC_a82d2.dir/link.txt --verbose=1
2020-02-28T06:12:37.0791800Z     /Users/runner/runners/2.165.0/work/1/s/clang+llvm-9.0.0-x86_64-darwin-apple/bin/clang++   -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin  -isysroot /Applications/Xcode_11.3.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.15.sdk -mmacosx-version-min=10.8 -Wl,-search_paths_first -Wl,-headerpad_max_install_names   CMakeFiles/cmTC_a82d2.dir/testCXXCompiler.cxx.o  -o cmTC_a82d2 
2020-02-28T06:12:37.0793240Z     ld: library not found for -lstdc++
2020-02-28T06:12:37.0794030Z     clang-9: error: linker command failed with exit code 1 (use -v to see invocation)
2020-02-28T06:12:37.0794470Z     make[1]: *** [cmTC_a82d2] Error 1
2020-02-28T06:12:37.0794780Z     make: *** [cmTC_a82d2/fast] Error 2
2020-02-28T06:12:37.0795170Z     
2020-02-28T06:12:37.0795300Z 
2020-02-28T06:12:37.0795440Z   
2020-02-28T06:12:37.0795560Z 
2020-02-28T06:12:37.0795560Z 
2020-02-28T06:12:37.0795820Z   CMake will not be able to correctly generate this project.
2020-02-28T06:12:37.0796160Z Call Stack (most recent call first):
2020-02-28T06:12:37.0796440Z   CMakeLists.txt:14 (project)
2020-02-28T06:12:37.0796630Z 
2020-02-28T06:12:37.0796780Z 
2020-02-28T06:12:37.0821990Z -- Configuring incomplete, errors occurred!
2020-02-28T06:12:37.0822950Z See also "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeOutput.log".
2020-02-28T06:12:37.0824100Z See also "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/native/sanitizers/build/CMakeFiles/CMakeError.log".
2020-02-28T06:12:37.0869750Z command did not execute successfully, got: exit code: 1
2020-02-28T06:12:37.0870010Z 
2020-02-28T06:12:37.0870010Z 
2020-02-28T06:12:37.0870980Z build script failed, must exit now', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.42/src/lib.rs:861:5
2020-02-28T06:12:37.0871950Z  finished in 0.972
2020-02-28T06:12:37.0887570Z failed to run: /Users/runner/runners/2.165.0/work/1/s/build/bootstrap/debug/bootstrap test
2020-02-28T06:12:37.0888070Z Build completed unsuccessfully in 0:22:06
2020-02-28T06:12:37.0939470Z == clock drift check ==
2020-02-28T06:12:37.0939470Z == clock drift check ==
2020-02-28T06:12:37.0983860Z   local time: Fri Feb 28 06:12:37 UTC 2020
2020-02-28T06:12:37.1374650Z   network time: Fri, 28 Feb 2020 06:12:37 GMT
2020-02-28T06:12:37.1376310Z == end clock drift check ==
2020-02-28T06:12:37.1417440Z 
2020-02-28T06:12:37.1531260Z ##[error]Bash exited with code '1'.
2020-02-28T06:12:37.1969150Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-28T06:12:37.1975790Z ==============================================================================
2020-02-28T06:12:37.1976190Z Task         : Get sources
2020-02-28T06:12:37.1976590Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
