plain
2019-10-28T14:50:40.7085200Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T14:50:41.4829345Z ##[command]git config gc.auto 0
2019-10-28T14:50:41.4831568Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T14:50:41.4833209Z ##[command]git config --get-all http.proxy
2019-10-28T14:50:41.4837606Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65703/merge:refs/remotes/pull/65703/merge
---
2019-10-28T16:09:32.1766079Z configure: build.locked-deps    := True
2019-10-28T16:09:32.1766129Z configure: llvm.ccache          := sccache
2019-10-28T16:09:32.1766353Z configure: build.cargo-native-static := True
2019-10-28T16:09:32.1766588Z configure: dist.missing-tools   := True
2019-10-28T16:09:32.1766854Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-10-28T16:09:32.1766966Z configure: writing `config.toml` in current directory
2019-10-28T16:09:32.1767012Z configure: 
2019-10-28T16:09:32.1767247Z configure: run `python /checkout/x.py --help`
2019-10-28T16:09:32.1767295Z configure: 
---
2019-10-28T16:19:44.4768097Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./DetermineGCCCompatible.cmake
2019-10-28T16:19:44.4768394Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindSphinx.cmake
2019-10-28T16:19:44.4810395Z cargo:root=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm
2019-10-28T16:19:44.4810582Z  finished in 467.362
2019-10-28T16:19:44.4821782Z running: "cmake" "/checkout/src/llvm-project/llvm" "-DCMAKE_SYSTEM_NAME=Windows" "-DCMAKE_RC_COMPILER=/usr/bin/i686-w64-mingw32-windres" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=2" "-DLLVM_TARGET_ARCH=i686" "-DLLVM_DEFAULT_TARGET_TRIPLE=i686-pc-windows-gnu" "-DLLVM_BUILD_32_BITS=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NATIVE_BUILD=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" "-DLLVM_VERSION_SUFFIX=-rust-1.40.0-dev" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=i686-w64-mingw32-gcc" "-DCMAKE_CXX_COMPILER=i686-w64-mingw32-g++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/i686-pc-windows-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
2019-10-28T16:19:44.7909575Z -- The CXX compiler identification is GNU 5.3.1
2019-10-28T16:19:44.7930536Z -- The ASM compiler identification is GNU
2019-10-28T16:19:44.7934393Z -- Found assembler: /usr/bin/i686-w64-mingw32-gcc
2019-10-28T16:19:44.8024778Z -- Check for working C compiler: /usr/bin/i686-w64-mingw32-gcc
---
2019-10-28T16:21:03.1374707Z [  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CallingConvEmitter.cpp.obj
2019-10-28T16:21:03.5289371Z [  1%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Error.cpp.obj
2019-10-28T16:21:05.6410714Z [  1%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj
2019-10-28T16:21:06.4231659Z [  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeEmitterGen.cpp.obj
2019-10-28T16:21:07.1494731Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:59:13: error: 'mutex' in namespace 'std' does not name a type
2019-10-28T16:21:07.1499329Z  static std::mutex ErrorHandlerMutex;
2019-10-28T16:21:07.1503415Z              ^
2019-10-28T16:21:07.1508275Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:60:13: error: 'mutex' in namespace 'std' does not name a type
2019-10-28T16:21:07.1512503Z  static std::mutex BadAllocErrorHandlerMutex;
2019-10-28T16:21:07.1516933Z              ^
2019-10-28T16:21:07.1521263Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::install_fatal_error_handler(llvm::fatal_error_handler_t, void*)':
2019-10-28T16:21:07.1525307Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:19: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1529521Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1545505Z                    ^
2019-10-28T16:21:07.1546113Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:19: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1546745Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:29: error: template argument 1 is invalid
2019-10-28T16:21:07.1546838Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1546887Z                              ^
2019-10-28T16:21:07.1547400Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:36: error: 'ErrorHandlerMutex' was not declared in this scope
2019-10-28T16:21:07.1547479Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1547832Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1547832Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1547903Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1548257Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_fatal_error_handler()':
2019-10-28T16:21:07.1548257Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_fatal_error_handler()':
2019-10-28T16:21:07.1548567Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:19: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1548624Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1548669Z                    ^
2019-10-28T16:21:07.1548977Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:19: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1549259Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:29: error: template argument 1 is invalid
2019-10-28T16:21:07.1549314Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1549376Z                              ^
2019-10-28T16:21:07.1549797Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:36: error: 'ErrorHandlerMutex' was not declared in this scope
2019-10-28T16:21:07.1549852Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1550203Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1550203Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1550259Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1550318Z                                ^
2019-10-28T16:21:07.1550617Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::report_fatal_error(const llvm::Twine&, bool)':
2019-10-28T16:21:07.1550893Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:21: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1550963Z      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1551007Z                      ^
2019-10-28T16:21:07.1551274Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:21: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1551651Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:31: error: template argument 1 is invalid
2019-10-28T16:21:07.1551716Z      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1551761Z                                ^
2019-10-28T16:21:07.1552089Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:38: error: 'ErrorHandlerMutex' was not declared in this scope
2019-10-28T16:21:07.1552156Z      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1552626Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:33: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1552626Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:33: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1552684Z      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-28T16:21:07.1552731Z                                  ^
2019-10-28T16:21:07.1553065Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::install_bad_alloc_error_handler(llvm::fatal_error_handler_t, void*)':
2019-10-28T16:21:07.1553368Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:19: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1553444Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1553497Z                    ^
2019-10-28T16:21:07.1553785Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:19: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1554193Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:29: error: template argument 1 is invalid
2019-10-28T16:21:07.1554251Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1554296Z                              ^
2019-10-28T16:21:07.1554615Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:36: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
2019-10-28T16:21:07.1554676Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1555041Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1555041Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1555099Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1555454Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_bad_alloc_error_handler()':
2019-10-28T16:21:07.1555454Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_bad_alloc_error_handler()':
2019-10-28T16:21:07.1555752Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:19: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1555808Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1555870Z                    ^
2019-10-28T16:21:07.1556149Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:19: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1556696Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:29: error: template argument 1 is invalid
2019-10-28T16:21:07.1556795Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1556842Z                              ^
2019-10-28T16:21:07.1557162Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:36: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
2019-10-28T16:21:07.1557235Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1557586Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1557586Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1557660Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1558004Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::report_bad_alloc_error(const char*, bool)':
2019-10-28T16:21:07.1558004Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::report_bad_alloc_error(const char*, bool)':
2019-10-28T16:21:07.1558419Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:21: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1558487Z      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1558534Z                      ^
2019-10-28T16:21:07.1559237Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:21: error: 'mutex' is not a member of 'std'
2019-10-28T16:21:07.1559569Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:31: error: template argument 1 is invalid
2019-10-28T16:21:07.1559657Z      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1559703Z                                ^
2019-10-28T16:21:07.1560013Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:38: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
2019-10-28T16:21:07.1560086Z      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1560433Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:33: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1560433Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:33: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-28T16:21:07.1560506Z      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-28T16:21:07.1560555Z                                  ^
2019-10-28T16:21:07.1580243Z make[2]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj] Error 1
2019-10-28T16:21:07.1580952Z lib/Support/CMakeFiles/LLVMSupport.dir/build.make:938: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj' failed
2019-10-28T16:21:07.1581020Z make[1]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/all] Error 2
2019-10-28T16:21:07.1581199Z make[1]: *** Waiting for unfinished jobs....
2019-10-28T16:21:07.1581494Z CMakeFiles/Makefile2:963: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/all' failed
2019-10-28T16:21:11.0353663Z [  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenHwModes.cpp.obj
2019-10-28T16:21:13.5785434Z [  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenInstruction.cpp.obj
2019-10-28T16:21:18.3023495Z [  2%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenMapTable.cpp.obj
2019-10-28T16:21:20.2515874Z [  2%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenRegisters.cpp.obj
---
2019-10-28T16:22:43.1331894Z [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.obj
2019-10-28T16:22:45.9477103Z [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.obj
2019-10-28T16:22:46.3351055Z [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.obj
2019-10-28T16:22:48.6347113Z [  3%] Built target obj.llvm-tblgen
2019-10-28T16:22:48.6359954Z Makefile:149: recipe for target 'all' failed
2019-10-28T16:22:48.6362680Z make: *** [all] Error 2
2019-10-28T16:22:48.6385851Z thread 'main' panicked at '
2019-10-28T16:22:48.6387628Z command did not execute successfully, got: exit code: 2
2019-10-28T16:22:48.6388337Z 
2019-10-28T16:22:48.6388337Z 
2019-10-28T16:22:48.6389154Z build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
2019-10-28T16:22:48.6392107Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2019-10-28T16:22:48.6392522Z Build completed unsuccessfully in 0:11:23
2019-10-28T16:22:48.6454463Z == clock drift check ==
2019-10-28T16:22:48.6505078Z   local time: Mon Oct 28 16:22:48 UTC 2019
2019-10-28T16:22:48.6505078Z   local time: Mon Oct 28 16:22:48 UTC 2019
2019-10-28T16:22:48.9459574Z   network time: Mon, 28 Oct 2019 16:22:48 GMT
2019-10-28T16:22:48.9464266Z == end clock drift check ==
2019-10-28T16:22:49.5114062Z 
2019-10-28T16:22:49.5217572Z ##[error]Bash exited with code '1'.
2019-10-28T16:22:49.5257255Z ##[section]Starting: Checkout
2019-10-28T16:22:49.5258920Z ==============================================================================
2019-10-28T16:22:49.5259006Z Task         : Get sources
2019-10-28T16:22:49.5259054Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
