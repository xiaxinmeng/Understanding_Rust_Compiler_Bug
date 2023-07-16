
[00:53:21] error: failed to run custom build command for `libgit2-sys v0.7.3`
[00:53:21] process didn't exit successfully: `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\release\build\libgit2-sys-de86c5586b57e17c\build-script-build` (exit code: 101)
[00:53:21] --- stdout
[00:53:21] running: "cmake" "C:\\Users\\appveyor\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\libgit2-sys-0.7.3\\libgit2" "-G" "Visual Studio 14 2015 Win64" "-DSTATIC_CRT=ON" "-DBUILD_SHARED_LIBS=OFF" "-DBUILD_CLAR=OFF" "-DCMAKE_INSTALL_PREFIX=C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\build\\libgit2-sys-8c462808fc2f0ee6\\out" "-DCMAKE_C_FLAGS= /GL- /IC:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\build\\libssh2-sys-032b4698ff5e0941\\out/include /DGIT_SSH /nologo /MT -m64" "-DCMAKE_C_FLAGS_RELEASE= /GL- /IC:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\build\\libssh2-sys-032b4698ff5e0941\\out/include /DGIT_SSH /nologo /MT -m64" "-DCMAKE_CXX_FLAGS= /nologo /MT -m64" "-DCMAKE_CXX_FLAGS_RELEASE= /nologo /MT -m64" "-DCMAKE_BUILD_TYPE=Release"
[00:53:21] -- The C compiler identification is MSVC 19.0.24215.1
[00:53:21] -- Check for working C compiler: C:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/bin/x86_amd64/cl.exe
[00:53:21] -- Check for working C compiler: C:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/bin/x86_amd64/cl.exe -- works
[00:53:21] -- Detecting C compiler ABI info
[00:53:21] -- Detecting C compiler ABI info - done
[00:53:21] -- Found PkgConfig: C:/msys64/mingw64/bin/pkg-config.exe (found version "0.29.2") 
[00:53:21] -- Looking for pthread.h
[00:53:21] -- Looking for pthread.h - not found
[00:53:21] -- Found Threads: TRUE  
[00:53:21] -- Performing Test HAVE_STRUCT_STAT_ST_MTIM
[00:53:21] -- Performing Test HAVE_STRUCT_STAT_ST_MTIM - Failed
[00:53:21] -- Performing Test HAVE_STRUCT_STAT_ST_MTIMESPEC
[00:53:21] -- Performing Test HAVE_STRUCT_STAT_ST_MTIMESPEC - Failed
[00:53:21] -- Performing Test HAVE_STRUCT_STAT_MTIME_NSEC
[00:53:21] -- Performing Test HAVE_STRUCT_STAT_MTIME_NSEC - Failed
[00:53:21] -- Looking for regcomp_l
[00:53:21] -- Looking for regcomp_l - not found
[00:53:21] -- Looking for futimens
[00:53:21] -- Looking for futimens - not found
[00:53:21] -- Looking for qsort_r
[00:53:21] -- Looking for qsort_r - not found
[00:53:21] -- Looking for qsort_s
[00:53:21] -- Looking for qsort_s - found
[00:53:21] -- Looking for clock_gettime in rt
[00:53:21] -- Looking for clock_gettime in rt - not found
[00:53:21] -- Found OpenSSL: optimized;C:/OpenSSL-Win64/lib/VC/libeay32MD.lib;debug;C:/OpenSSL-Win64/lib/VC/libeay32MDd.lib (found version "1.0.2l") 
[00:53:21] -- Could NOT find HTTP_Parser (missing: HTTP_PARSER_INCLUDE_DIR HTTP_PARSER_LIBRARY) 
[00:53:21] -- http-parser version 2 was not found or disabled; using bundled 3rd-party sources.
[00:53:21] -- Found ZLIB: C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/release/build/libz-sys-39c851ad55256a89/out/lib/zlib.lib (found version "1.2.11") 
[00:53:21] -- Checking for module 'libssh2'
[00:53:21] --   Found libssh2, version 1.8.1_DEV
[00:53:21] -- Configuring incomplete, errors occurred!
[00:53:21] See also "C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/release/build/libgit2-sys-8c462808fc2f0ee6/out/build/CMakeFiles/CMakeOutput.log".
[00:53:21] See also "C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/release/build/libgit2-sys-8c462808fc2f0ee6/out/build/CMakeFiles/CMakeError.log".
[00:53:21] 
[00:53:21] --- stderr
[00:53:21] /mingw64/bin/gettext.sh: line 88: export: `dashless
[00:53:21] ': not a valid identifier
[00:53:21] fatal: not a git repository (or any of the parent directories): .git
[00:53:21] CMake Error at cmake/Modules/FindPkgLibraries.cmake:17 (MESSAGE):
[00:53:21]   could not resolve ssh2
[00:53:21] Call Stack (most recent call first):
[00:53:21]   src/CMakeLists.txt:341 (FIND_PKGLIBRARIES)
[00:53:21] 
[00:53:21] 
[00:53:21] thread 'main' panicked at '
[00:53:21] command did not execute successfully, got: exit code: 1
[00:53:21] 
[00:53:21] build script failed, must exit now', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.31\src\lib.rs:643:5
[00:53:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
