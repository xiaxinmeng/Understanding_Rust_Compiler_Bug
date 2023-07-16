plain
debconf: falling back to frontend: Readline
debconf: unable to initialize frontend: Readline
debconf: (This frontend requires a controlling tty.)
debconf: falling back to frontend: Teletype
Updating certificates in /etc/ssl/certs... WARNING: Skipping duplicate certificate brasil.gov.br.pem
Running hooks in /etc/ca-certificates/update.d....done.
Setting up libgmp3c2 (2:4.3.2+dfsg-1) ...
Setting up libmpfr4 (3.0.0-2) ...
Setting up cpp-4.4 (4.4.5-8) ...
---
Step 29/35 : ENV HOSTS=x86_64-unknown-linux-gnu
 ---> Running in ca3ae8b29cdc
Removing intermediate container ca3ae8b29cdc
 ---> 963e2f202182
Step 30/35 : ENV RUST_CONFIGURE_ARGS       --enable-full-tools       --enable-sanitizers       --enable-profiler       --enable-compiler-docs       --set target.x86_64-unknown-linux-gnu.linker=clang       --set target.x86_64-unknown-linux-gnu.ar=/rustroot/bin/llvm-ar       --set target.x86_64-unknown-linux-gnu.ranlib=/rustroot/bin/llvm-ranlib       --set llvm.thin-lto=true       --set llvm.ninja=false       --set llvm.use-linker=lld       --set rust.use-lld=true       --set rust.jemalloc
Removing intermediate container bec55d7f16e1
 ---> 109eb7065b67
Step 31/35 : ENV SCRIPT ../src/ci/pgo.sh python3 ../x.py dist     --host $HOSTS --target $HOSTS     --include-default-paths     src/tools/build-manifest
 ---> Running in 547f0ddcfd80
---
-- Looking for pthread_create in pthreads - not found
-- Looking for pthread_create in pthread
-- Looking for pthread_create in pthread - found
-- Found Threads: TRUE  
-- Found ZLIB: /usr/lib/libz.so (found version "1.2.3.4") 
-- Looking for compress2 - found
-- Looking for xar_open in xar
-- Looking for xar_open in xar - not found
-- Looking for arc4random
---
Call Stack (most recent call first):
  CMakeLists.txt:698 (include)


-- Configuring incomplete, errors occurred!
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
command did not execute successfully, got: exit code: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
 finished in 7.717 seconds
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:01:35
