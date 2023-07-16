plain
2019-11-27T17:09:52.5881972Z     Updating crates.io index
2019-11-27T17:09:53.6544510Z error: failed to fetch `https://github.com/rust-lang/crates.io-index`
2019-11-27T17:09:53.6544643Z 
2019-11-27T17:09:53.6544706Z Caused by:
2019-11-27T17:09:53.6544831Z   SSL error: error:140E0197:SSL routines:SSL_shutdown:shutdown while in init; class=Ssl (16)
2019-11-27T17:09:53.6551725Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-27T17:09:53.6598553Z Makefile:67: recipe for target 'prepare' failed
2019-11-27T17:09:53.6605967Z make: *** [prepare] Error 1
2019-11-27T17:09:54.6620738Z Command failed. Attempt 2/5:
2019-11-27T17:09:54.7839995Z     Updating crates.io index
---
2019-11-27T17:43:10.1200237Z [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/APInt.cpp.o
2019-11-27T17:43:10.3387861Z [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/APSInt.cpp.o
2019-11-27T17:43:10.5370548Z [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMBuildAttrs.cpp.o
2019-11-27T17:43:10.7525556Z [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMAttributeParser.cpp.o
2019-11-27T17:43:10.7934245Z sccache: encountered fatal error
2019-11-27T17:43:10.7939056Z sccache: error : Invalid checksum
2019-11-27T17:43:10.7944047Z sccache:  cause: Invalid checksum
2019-11-27T17:43:10.7954620Z lib/Demangle/CMakeFiles/LLVMDemangle.dir/build.make:86: recipe for target 'lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.o' failed
2019-11-27T17:43:10.7959162Z make[3]: *** [lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.o] Error 254
2019-11-27T17:43:10.7965040Z make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
2019-11-27T17:43:10.7983586Z CMakeFiles/Makefile2:780: recipe for target 'lib/Demangle/CMakeFiles/LLVMDemangle.dir/all' failed
2019-11-27T17:43:10.7988141Z make[2]: *** [lib/Demangle/CMakeFiles/LLVMDemangle.dir/all] Error 2
2019-11-27T17:43:10.7993047Z make[2]: *** Waiting for unfinished jobs....
2019-11-27T17:43:11.0255235Z [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Allocator.cpp.o
2019-11-27T17:43:11.2290046Z [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamError.cpp.o
2019-11-27T17:43:11.4334886Z [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamReader.cpp.o
2019-11-27T17:43:11.6328825Z [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamRef.cpp.o
---
2019-11-27T17:43:16.3772278Z [  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/InitLLVM.cpp.o
2019-11-27T17:43:16.4794656Z [  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/IntEqClasses.cpp.o
2019-11-27T17:43:16.6190546Z [  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/IntervalMap.cpp.o
2019-11-27T17:43:16.7166853Z [  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ItaniumManglingCanonicalizer.cpp.o
2019-11-27T17:43:16.8691832Z sccache: encountered fatal error
2019-11-27T17:43:16.8693433Z sccache: error : corrupt deflate stream
2019-11-27T17:43:16.8693575Z sccache:  cause: corrupt deflate stream
2019-11-27T17:43:16.8694458Z lib/Support/CMakeFiles/LLVMSupport.dir/build.make:1190: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/IntervalMap.cpp.o' failed
2019-11-27T17:43:16.8694687Z make[3]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/IntervalMap.cpp.o] Error 254
2019-11-27T17:43:16.8697439Z make[3]: *** Waiting for unfinished jobs....
2019-11-27T17:43:18.4346776Z sccache: encountered fatal error
2019-11-27T17:43:18.4347461Z sccache: error : Invalid checksum
2019-11-27T17:43:18.4347647Z sccache:  cause: Invalid checksum
2019-11-27T17:43:18.4357903Z lib/Support/CMakeFiles/LLVMSupport.dir/build.make:1214: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/ItaniumManglingCanonicalizer.cpp.o' failed
2019-11-27T17:43:18.4358742Z make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
2019-11-27T17:43:18.4359319Z CMakeFiles/Makefile2:901: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/all' failed
2019-11-27T17:43:18.4360311Z Makefile:149: recipe for target 'all' failed
2019-11-27T17:43:18.4360800Z make[1]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
2019-11-27T17:43:18.4360800Z make[1]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
2019-11-27T17:43:18.4361046Z make[3]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/ItaniumManglingCanonicalizer.cpp.o] Error 254
2019-11-27T17:43:18.4361547Z make[2]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/all] Error 2
2019-11-27T17:43:18.4361722Z make[1]: *** [all] Error 2
2019-11-27T17:43:18.4379239Z command did not execute successfully, got: exit code: 2
2019-11-27T17:43:18.4379420Z 
2019-11-27T17:43:18.4379420Z 
2019-11-27T17:43:18.4379972Z build script failed, must exit now', /checkout/obj/build/tmp/distcheck/vendor/cmake/src/lib.rs:813:5
2019-11-27T17:43:18.4380542Z  finished in 23.509
2019-11-27T17:43:18.4399445Z failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test
2019-11-27T17:43:18.4399809Z Build completed unsuccessfully in 0:25:26
2019-11-27T17:43:18.4399809Z Build completed unsuccessfully in 0:25:26
2019-11-27T17:43:18.4456069Z Makefile:48: recipe for target 'check' failed
2019-11-27T17:43:18.4456576Z 
2019-11-27T17:43:18.4456787Z command did not execute successfully: "make" "check"
2019-11-27T17:43:18.4457340Z expected success, got: exit code: 2
2019-11-27T17:43:18.4457745Z 
---
2019-11-27T17:43:18.4547522Z   local time: Wed Nov 27 17:43:18 UTC 2019
2019-11-27T17:43:18.4855158Z   network time: Wed, 27 Nov 2019 17:43:18 GMT
2019-11-27T17:43:18.4856158Z == end clock drift check ==
2019-11-27T17:43:21.8684174Z 
2019-11-27T17:43:21.8794898Z ##[error]Bash exited with code '1'.
2019-11-27T17:43:21.8832040Z ##[section]Starting: Checkout
2019-11-27T17:43:21.8834612Z ==============================================================================
2019-11-27T17:43:21.8834731Z Task         : Get sources
2019-11-27T17:43:21.8834819Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
