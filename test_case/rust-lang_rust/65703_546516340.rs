plain
2019-10-25T19:29:23.8640216Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T19:29:23.8829976Z ##[command]git config gc.auto 0
2019-10-25T19:29:23.8902869Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T19:29:23.8963923Z ##[command]git config --get-all http.proxy
2019-10-25T19:29:23.9107894Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65703/merge:refs/remotes/pull/65703/merge
---
2019-10-25T20:49:44.8044184Z configure: build.locked-deps    := True
2019-10-25T20:49:44.8044318Z configure: llvm.ccache          := sccache
2019-10-25T20:49:44.8044579Z configure: build.cargo-native-static := True
2019-10-25T20:49:44.8044873Z configure: dist.missing-tools   := True
2019-10-25T20:49:44.8045883Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-10-25T20:49:44.8046623Z configure: writing `config.toml` in current directory
2019-10-25T20:49:44.8046840Z configure: 
2019-10-25T20:49:44.8047279Z configure: run `python /checkout/x.py --help`
2019-10-25T20:49:44.8047483Z configure: 
---
2019-10-25T20:59:34.6222193Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./DetermineGCCCompatible.cmake
2019-10-25T20:59:34.6222431Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindSphinx.cmake
2019-10-25T20:59:34.6254732Z cargo:root=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm
2019-10-25T20:59:34.6254804Z  finished in 460.653
2019-10-25T20:59:34.6266104Z running: "cmake" "/checkout/src/llvm-project/llvm" "-DCMAKE_SYSTEM_NAME=Windows" "-DCMAKE_RC_COMPILER=/usr/bin/i686-w64-mingw32-windres" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=2" "-DLLVM_TARGET_ARCH=i686" "-DLLVM_DEFAULT_TARGET_TRIPLE=i686-pc-windows-gnu" "-DLLVM_BUILD_32_BITS=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NATIVE_BUILD=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" "-DLLVM_VERSION_SUFFIX=-rust-1.40.0-dev" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=i686-w64-mingw32-gcc" "-DCMAKE_CXX_COMPILER=i686-w64-mingw32-g++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/i686-pc-windows-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
2019-10-25T20:59:34.9288073Z -- The CXX compiler identification is GNU 5.3.1
2019-10-25T20:59:34.9305311Z -- The ASM compiler identification is GNU
2019-10-25T20:59:34.9309418Z -- Found assembler: /usr/bin/i686-w64-mingw32-gcc
2019-10-25T20:59:34.9394901Z -- Check for working C compiler: /usr/bin/i686-w64-mingw32-gcc
---
2019-10-25T21:00:50.4429117Z [  1%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DJB.cpp.obj
2019-10-25T21:00:51.4875298Z [  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeEmitterGen.cpp.obj
2019-10-25T21:00:51.7607395Z [  1%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Error.cpp.obj
2019-10-25T21:00:53.8782530Z [  1%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj
2019-10-25T21:00:55.3442407Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:59:13: error: 'mutex' in namespace 'std' does not name a type
2019-10-25T21:00:55.3442579Z  static std::mutex ErrorHandlerMutex;
2019-10-25T21:00:55.3442632Z              ^
2019-10-25T21:00:55.3443006Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:60:13: error: 'mutex' in namespace 'std' does not name a type
2019-10-25T21:00:55.3443084Z  static std::mutex BadAllocErrorHandlerMutex;
2019-10-25T21:00:55.3443131Z              ^
2019-10-25T21:00:55.3443465Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::install_fatal_error_handler(llvm::fatal_error_handler_t, void*)':
2019-10-25T21:00:55.3444088Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:19: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3444142Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3444196Z                    ^
2019-10-25T21:00:55.3444465Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:19: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3444719Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:29: error: template argument 1 is invalid
2019-10-25T21:00:55.3444769Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3444834Z                              ^
2019-10-25T21:00:55.3445107Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:36: error: 'ErrorHandlerMutex' was not declared in this scope
2019-10-25T21:00:55.3445160Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3445480Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3445480Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:66:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3445531Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3446073Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_fatal_error_handler()':
2019-10-25T21:00:55.3446073Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_fatal_error_handler()':
2019-10-25T21:00:55.3446328Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:19: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3446393Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3446525Z                    ^
2019-10-25T21:00:55.3446805Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:19: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3447075Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:29: error: template argument 1 is invalid
2019-10-25T21:00:55.3447206Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3447249Z                              ^
2019-10-25T21:00:55.3447540Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:36: error: 'ErrorHandlerMutex' was not declared in this scope
2019-10-25T21:00:55.3447601Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3447922Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3447922Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:75:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3447973Z    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3448021Z                                ^
2019-10-25T21:00:55.3448309Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::report_fatal_error(const llvm::Twine&, bool)':
2019-10-25T21:00:55.3448565Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:21: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3448616Z      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3448674Z                      ^
2019-10-25T21:00:55.3448924Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:21: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3449186Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:31: error: template argument 1 is invalid
2019-10-25T21:00:55.3449254Z      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3449295Z                                ^
2019-10-25T21:00:55.3449569Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:38: error: 'ErrorHandlerMutex' was not declared in this scope
2019-10-25T21:00:55.3449639Z      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3455712Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:33: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3455712Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:100:33: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3459622Z      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
2019-10-25T21:00:55.3459679Z                                  ^
2019-10-25T21:00:55.3466187Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::install_bad_alloc_error_handler(llvm::fatal_error_handler_t, void*)':
2019-10-25T21:00:55.3466505Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:19: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3466563Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3470449Z                    ^
2019-10-25T21:00:55.3471290Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:19: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3485841Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:29: error: template argument 1 is invalid
2019-10-25T21:00:55.3485915Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3485961Z                              ^
2019-10-25T21:00:55.3486279Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:36: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
2019-10-25T21:00:55.3486508Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3486880Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3486880Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:131:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3486934Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3487330Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_bad_alloc_error_handler()':
2019-10-25T21:00:55.3487330Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_bad_alloc_error_handler()':
2019-10-25T21:00:55.3487618Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:19: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3487687Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3487730Z                    ^
2019-10-25T21:00:55.3487981Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:19: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3488264Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:29: error: template argument 1 is invalid
2019-10-25T21:00:55.3488316Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3488359Z                              ^
2019-10-25T21:00:55.3488651Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:36: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
2019-10-25T21:00:55.3488712Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3489034Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3489034Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:140:31: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3489086Z    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3489412Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::report_bad_alloc_error(const char*, bool)':
2019-10-25T21:00:55.3489412Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::report_bad_alloc_error(const char*, bool)':
2019-10-25T21:00:55.3489679Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:21: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3490194Z      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3490259Z                      ^
2019-10-25T21:00:55.3490589Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:21: error: 'mutex' is not a member of 'std'
2019-10-25T21:00:55.3490876Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:31: error: template argument 1 is invalid
2019-10-25T21:00:55.3490950Z      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3490998Z                                ^
2019-10-25T21:00:55.3491305Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:38: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
2019-10-25T21:00:55.3491379Z      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3491727Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:33: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3491727Z /checkout/src/llvm-project/llvm/lib/Support/ErrorHandling.cpp:153:33: warning: unused variable 'Lock' [-Wunused-variable]
2019-10-25T21:00:55.3491800Z      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
2019-10-25T21:00:55.3491848Z                                  ^
2019-10-25T21:00:55.3511765Z lib/Support/CMakeFiles/LLVMSupport.dir/build.make:938: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj' failed
2019-10-25T21:00:55.3511884Z make[2]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj] Error 1
2019-10-25T21:00:55.3512191Z CMakeFiles/Makefile2:963: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/all' failed
2019-10-25T21:00:55.3512249Z make[1]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/all] Error 2
2019-10-25T21:00:55.3512317Z make[1]: *** Waiting for unfinished jobs....
2019-10-25T21:00:56.2289054Z [  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenHwModes.cpp.obj
2019-10-25T21:00:58.8009690Z [  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenInstruction.cpp.obj
2019-10-25T21:01:03.5748728Z [  2%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenMapTable.cpp.obj
2019-10-25T21:01:07.8405583Z [  2%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenRegisters.cpp.obj
---
2019-10-25T21:02:30.3724342Z [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.obj
2019-10-25T21:02:31.7562947Z [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.obj
2019-10-25T21:02:33.4047235Z [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.obj
2019-10-25T21:02:35.5676712Z [  3%] Built target obj.llvm-tblgen
2019-10-25T21:02:35.5688245Z Makefile:149: recipe for target 'all' failed
2019-10-25T21:02:35.5688440Z make: *** [all] Error 2
2019-10-25T21:02:35.5708066Z command did not execute successfully, got: exit code: 2
2019-10-25T21:02:35.5708102Z 
2019-10-25T21:02:35.5708102Z 
2019-10-25T21:02:35.5708439Z build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
2019-10-25T21:02:35.5712082Z  finished in 641.599
2019-10-25T21:02:35.5730711Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2019-10-25T21:02:35.5730816Z Build completed unsuccessfully in 0:11:12
2019-10-25T21:02:35.5779944Z == clock drift check ==
2019-10-25T21:02:35.5779944Z == clock drift check ==
2019-10-25T21:02:35.5799154Z   local time: Fri Oct 25 21:02:35 UTC 2019
2019-10-25T21:02:35.8562371Z   network time: Fri, 25 Oct 2019 21:02:35 GMT
2019-10-25T21:02:35.8562599Z == end clock drift check ==
2019-10-25T21:02:36.4544078Z 
2019-10-25T21:02:36.4647276Z ##[error]Bash exited with code '1'.
2019-10-25T21:02:36.4681923Z ##[section]Starting: Checkout
2019-10-25T21:02:36.4684199Z ==============================================================================
2019-10-25T21:02:36.4684408Z Task         : Get sources
2019-10-25T21:02:36.4684456Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
