plain
[00:01:47] checking whether linker accepts -Wl,--gc-sections... yes
[00:01:47] checking whether linker accepts -Wl,--hash-style=both... yes
[00:01:47] checking whether linker accepts -Wl,--no-undefined... yes
[00:01:47] checking whether linker accepts -Wl,--exclude-libs=ALL... yes
[00:01:47] checking whether linker accepts -Wl,--dynamic-list=./dynamic.list... yes
[00:01:47] checking whether linker accepts -lgcc_eh... yes
[00:01:47] using compiler runtime libraries: -lgcc -lgcc_eh
[00:01:47] checking preprocessor condition __ILP32__... false
[00:01:47] checking whether compiler's long double definition matches float.h... yes
---
[00:02:20] Step 10/12 : ENV HOSTS x86_64-unknown-linux-musl
[00:02:20]  ---> Running in 206e83d1ee99
[00:02:20]  ---> c9c10900a889
[00:02:20] Removing intermediate container 206e83d1ee99
[00:02:20] Step 11/12 : ENV RUSTFLAGS "-C target-feature=-crt-static"
[00:02:21]  ---> db9d614e87e6
[00:02:21] Removing intermediate container 87ea318b0395
[00:02:21] Step 12/12 : ENV SCRIPT python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:02:21]  ---> Running in 3b53bcc4e072
[00:02:21]  ---> Running in 3b53bcc4e072
[00:02:21]  ---> 38e23c89f1fb
[00:02:21] Removing intermediate container 3b53bcc4e072
[00:02:21] Successfully built 38e23c89f1fb
[00:02:21] Successfully tagged rust-ci:latest
[00:02:21] Built container sha256:38e23c89f1fb9269d14eb3c40bd7aee40db8eff12e8e6f20bf82f297c8dc64ee
[00:02:21] Uploading finished image to s3://rust-lang-ci-sccache2/docker/aff9cb5f157974aa7cbcbd03b437abba8029ce0a15de9880d50f06f2e4baf30d4703f4c6c2e093301974ea43fde630ef55aa763eee9e2fba63a8d64914c2566a
[00:02:21] 
[00:02:21] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:25] xargs: docker: terminated by signal 13

[00:02:25] travis_time:end:00c5043b:start=1539892261220086746,finish=1539892348566298704,duration=87346211958
[CI_JOB_NAME=dist-x86_64-musl]
[00:02:25] [CI_JOB_NAME=dist-x86_64-musl]
---

[00:42:00] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-musl
[00:42:00] running: "cmake" "/checkout/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-musl" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NATIVE_BUILD=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=/musl-x86_64/bin/musl-gcc" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=/musl-x86_64/bin/musl-gcc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -Wa,-mrelax-relocations=no -m64 -static" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -Wa,-mrelax-relocations=no -m64 -static" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-musl/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:42:00] -- The CXX compiler identification is GNU 5.4.0
[00:42:00] -- The ASM compiler identification is GNU
[00:42:00] -- Found assembler: /usr/local/bin/sccache
[00:42:00] -- Check for working C compiler: /usr/local/bin/sccache
---
[00:42:05] -- Looking for CrashReporterClient.h
[00:42:05] -- Looking for CrashReporterClient.h - not found
[00:42:05] -- Looking for linux/magic.h
[00:42:05] -- Looking for linux/magic.h - not found
[00:42:05] -- Looking for linux/nfs_fs.h
[00:42:05] -- Looking for linux/nfs_fs.h - not found
[00:42:05] -- Looking for linux/smb.h
[00:42:05] -- Looking for linux/smb.h - not found
[00:42:06] -- Looking for pthread_create in pthread - found
[00:42:06] -- Looking for pthread_getspecific in pthread
[00:42:06] -- Looking for pthread_getspecific in pthread - found
[00:42:06] -- Looking for pthread_rwlock_init in pthread
---
[00:42:13] -- Looking for __atomic_fetch_add_4 in atomic
[00:42:13] -- Looking for __atomic_fetch_add_4 in atomic - found
[00:42:13] -- Performing Test HAVE_CXX_ATOMICS_WITH_LIB
[00:42:13] -- Performing Test HAVE_CXX_ATOMICS_WITH_LIB - Failed
[00:42:13] CMake Error at cmake/modules/CheckAtomic.cmake:50 (message):
[00:42:13]   Host compiler must support std::atomic!
[00:42:13] Call Stack (most recent call first):
[00:42:13]   cmake/config-ix.cmake:323 (include)
[00:42:13] 
[00:42:13] 
[00:42:13] -- Configuring incomplete, errors occurred!
[00:42:13] -- Configuring incomplete, errors occurred!
[00:42:13] See also "/checkout/obj/build/x86_64-unknown-linux-musl/llvm/build/CMakeFiles/CMakeOutput.log".
[00:42:13] See also "/checkout/obj/build/x86_64-unknown-linux-musl/llvm/build/CMakeFiles/CMakeError.log".
[00:42:13] command did not execute successfully, got: exit code: 1
[00:42:13] 
[00:42:13] 
[00:42:13] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[00:42:13]  finished in 13.202
[00:42:13] travis_fold:end:llvm

[00:42:13] travis_time:end:llvm:start=1539894723470513708,finish=1539894736673476323,duration=13202962615
---
travis_time:end:1c93d778:start=1539894737653109223,finish=1539894737658024618,duration=4915395
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ef92e15
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b7d5f8c
travis_time:start:1b7d5f8c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12df8831
$ dmesg | grep -i kill
