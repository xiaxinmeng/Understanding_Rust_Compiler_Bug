plain
2019-11-25T09:00:45.0334552Z ##[command]git config gc.auto 0
2019-11-25T09:00:45.0413230Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T09:00:45.0490123Z ##[command]git config --get-all http.proxy
2019-11-25T09:00:45.0623814Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin
2019-11-25T09:00:46.1829507Z fatal: unable to access 'https://github.com/rust-lang/rust/': GnuTLS recv error (-24): Decryption has failed.
2019-11-25T09:00:46.3030433Z ##[warning]Git fetch failed with exit code 128, back off 2.263 seconds before retry.
---
2019-11-25T09:26:16.9603625Z [ 19%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceExpander.cpp.o
2019-11-25T09:26:17.1700236Z [ 19%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumGlobals.cpp.o
2019-11-25T09:26:17.1982117Z [ 19%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceWriter.cpp.o
2019-11-25T09:26:17.4143044Z [ 19%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FileHeaderReader.cpp.o
2019-11-25T09:26:17.5574146Z sccache: encountered fatal error
2019-11-25T09:26:17.5574277Z sccache: error : Invalid checksum
2019-11-25T09:26:17.5576297Z sccache:  cause: Invalid checksum
2019-11-25T09:26:17.5576835Z make[2]: *** [lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumGlobals.cpp.o] Error 254
2019-11-25T09:26:17.5583695Z make[1]: *** [lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/all] Error 2
2019-11-25T09:26:17.5583794Z make[1]: *** Waiting for unfinished jobs....
2019-11-25T09:26:17.6271606Z [ 19%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/LogBuilderConsumer.cpp.o
2019-11-25T09:26:17.8650232Z [ 19%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.o
2019-11-25T09:26:17.9322241Z [ 19%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordInitializer.cpp.o
2019-11-25T09:26:18.1373495Z [ 19%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordPrinter.cpp.o
2019-11-25T09:26:18.1373495Z [ 19%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordPrinter.cpp.o
2019-11-25T09:26:18.2052854Z sccache: encountered fatal error
2019-11-25T09:26:18.2053025Z sccache: error : Invalid checksum
2019-11-25T09:26:18.2053151Z sccache:  cause: Invalid checksum
2019-11-25T09:26:18.2066034Z make[2]: *** [lib/XRay/CMakeFiles/LLVMXRay.dir/Profile.cpp.o] Error 254
2019-11-25T09:26:18.2066398Z make[2]: *** Waiting for unfinished jobs....
2019-11-25T09:26:18.3380471Z make[1]: *** [lib/XRay/CMakeFiles/LLVMXRay.dir/all] Error 2
2019-11-25T09:26:18.3380631Z make: *** [all] Error 2
2019-11-25T09:26:18.3397268Z command did not execute successfully, got: exit code: 2
2019-11-25T09:26:18.3397319Z 
2019-11-25T09:26:18.3397319Z 
2019-11-25T09:26:18.3397693Z build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
2019-11-25T09:26:18.3403603Z  finished in 69.235
2019-11-25T09:26:18.3424378Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host arm-unknown-linux-gnueabi --target arm-unknown-linux-gnueabi
2019-11-25T09:26:18.3424602Z Build completed unsuccessfully in 0:20:57
2019-11-25T09:26:18.3480883Z == clock drift check ==
2019-11-25T09:26:18.3480883Z == clock drift check ==
2019-11-25T09:26:18.3496536Z   local time: Mon Nov 25 09:26:18 UTC 2019
2019-11-25T09:26:18.3794675Z   network time: Mon, 25 Nov 2019 09:26:18 GMT
2019-11-25T09:26:18.3796578Z == end clock drift check ==
2019-11-25T09:26:19.4229842Z 
2019-11-25T09:26:19.4250542Z ##[error]Bash exited with code '1'.
2019-11-25T09:26:19.4278144Z ##[section]Starting: Checkout
2019-11-25T09:26:19.4280656Z ==============================================================================
2019-11-25T09:26:19.4280964Z Task         : Get sources
2019-11-25T09:26:19.4281069Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
