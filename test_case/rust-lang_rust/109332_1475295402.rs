plain
Requested labels: windows-latest-8core-32gb
Job defined at: rust-lang-ci/rust/.github/workflows/ci.yml@refs/heads/auto
Waiting for a runner to pick up this job...
Job is about to start running on the runner: windows-latest-8core-32gb_6bfad66574cb (organization)
Runner name: 'windows-latest-8core-32gb_6bfad66574cb'
Runner group name: 'Default Larger Runners'
Machine name: 'runner'
##[group]Operating System
---
Working directory is 'C:\a\rust\rust'
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.39.2.windows.1
##[endgroup]
Copying 'C:\Users\runneradmin\.gitconfig' to 'C:\a\_temp\073c870a-107f-44e3-9074-58f08b1e7978\.gitconfig'
Temporarily overriding HOME='C:\a\_temp\073c870a-107f-44e3-9074-58f08b1e7978' before making global git config changes
[command]"C:\Program Files\Git\bin\git.exe" config --global --add safe.directory C:\a\rust\rust
Deleting the contents of 'C:\a\rust\rust'
##[group]Initializing the repository
[command]"C:\Program Files\Git\bin\git.exe" init C:\a\rust\rust
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  NO_DOWNLOAD_CI_LLVM: 1
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
##[group]Run src/ci/scripts/install-mingw.sh
src/ci/scripts/install-mingw.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
##[endgroup]
:: There are 19 members in group mingw-w64-i686-toolchain:
:: Repository mingw32
   1) mingw-w64-i686-binutils  2) mingw-w64-i686-crt-git  3) mingw-w64-i686-gcc  4) mingw-w64-i686-gcc-ada  5) mingw-w64-i686-gcc-fortran  6) mingw-w64-i686-gcc-libgfortran  7) mingw-w64-i686-gcc-libs  8) mingw-w64-i686-gcc-objc  9) mingw-w64-i686-gdb  10) mingw-w64-i686-gdb-multiarch  11) mingw-w64-i686-headers-git  12) mingw-w64-i686-libgccjit  13) mingw-w64-i686-libmangle-git  14) mingw-w64-i686-libwinpthread-git  15) mingw-w64-i686-make  16) mingw-w64-i686-pkgconf  17) mingw-w64-i686-tools-git  18) mingw-w64-i686-winpthreads-git  19) mingw-w64-i686-winstorecompat-git

Enter a selection (default=all): 
looking for conflicting packages...


Packages (64) mingw-w64-i686-brotli-1.0.9-5  mingw-w64-i686-bzip2-1.0.8-2  mingw-w64-i686-c-ares-1.19.0-1  mingw-w64-i686-ca-certificates-20211016-4  mingw-w64-i686-curl-7.88.1-1  mingw-w64-i686-expat-2.5.0-1  mingw-w64-i686-gettext-0.21.1-1  mingw-w64-i686-gmp-6.2.1-5  mingw-w64-i686-isl-0.25-1  mingw-w64-i686-jsoncpp-1.9.5-2  mingw-w64-i686-libarchive-3.6.2-2  mingw-w64-i686-libb2-0.98.1-2  mingw-w64-i686-libffi-3.4.4-1  mingw-w64-i686-libiconv-1.17-3  mingw-w64-i686-libidn2-2.3.4-1  mingw-w64-i686-libpsl-0.21.2-4  mingw-w64-i686-libssh2-1.10.0-2  mingw-w64-i686-libsystre-1.0.1-4  mingw-w64-i686-libtasn1-4.19.0-1  mingw-w64-i686-libtre-git-r128.6fb7206-2  mingw-w64-i686-libunistring-1.1-1  mingw-w64-i686-libuv-1.44.2-2  mingw-w64-i686-lz4-1.9.4-1  mingw-w64-i686-mpc-1.3.1-1  mingw-w64-i686-mpdecimal-2.5.1-1  mingw-w64-i686-mpfr-4.2.0-1  mingw-w64-i686-ncurses-6.4.20230211-1  mingw-w64-i686-nghttp2-1.52.0-1  mingw-w64-i686-ninja-1.11.1-3  mingw-w64-i686-openssl-3.0.8-1  mingw-w64-i686-p11-kit-0.24.1-3  mingw-w64-i686-readline-8.2.001-6  mingw-w64-i686-rhash-1.4.3-1  mingw-w64-i686-sqlite3-3.41.0-1  mingw-w64-i686-tcl-8.6.12-1  mingw-w64-i686-termcap-1.3.1-6  mingw-w64-i686-tk-8.6.12-1  mingw-w64-i686-tzdata-2022g-1  mingw-w64-i686-windows-default-manifest-6.4-4  mingw-w64-i686-xxhash-0.8.1-2  mingw-w64-i686-xz-5.4.1-1  mingw-w64-i686-zlib-1.2.13-3  mingw-w64-i686-zstd-1.5.4-1  mingw-w64-i686-binutils-2.40-2  mingw-w64-i686-cmake-3.25.2-1  mingw-w64-i686-crt-git-10.0.0.r234.g283e5b23a-1  mingw-w64-i686-gcc-12.2.0-10  mingw-w64-i686-gcc-ada-12.2.0-10  mingw-w64-i686-gcc-fortran-12.2.0-10  mingw-w64-i686-gcc-libgfortran-12.2.0-10  mingw-w64-i686-gcc-libs-12.2.0-10  mingw-w64-i686-gcc-objc-12.2.0-10  mingw-w64-i686-gdb-13.1-3  mingw-w64-i686-gdb-multiarch-13.1-3  mingw-w64-i686-headers-git-10.0.0.r234.g283e5b23a-1  mingw-w64-i686-libgccjit-12.2.0-10  mingw-w64-i686-libmangle-git-10.0.0.r234.g283e5b23a-1  mingw-w64-i686-libwinpthread-git-10.0.0.r234.g283e5b23a-1  mingw-w64-i686-make-4.4-2  mingw-w64-i686-pkgconf-1~1.8.0-2  mingw-w64-i686-python-3.10.10-1  mingw-w64-i686-tools-git-10.0.0.r234.g283e5b23a-1  mingw-w64-i686-winpthreads-git-10.0.0.r234.g283e5b23a-1  mingw-w64-i686-winstorecompat-git-10.0.0.r234.g283e5b23a-1
Total Download Size:    187.57 MiB
Total Installed Size:  1088.07 MiB


:: Proceed with installation? [Y/n] 
:: Retrieving packages...
 mingw-w64-i686-gcc-12.2.0-10-any downloading...
 mingw-w64-i686-gcc-objc-12.2.0-10-any downloading...
 mingw-w64-i686-gcc-ada-12.2.0-10-any downloading...
 mingw-w64-i686-python-3.10.10-1-any downloading...
 mingw-w64-i686-cmake-3.25.2-1-any downloading...
 mingw-w64-i686-gcc-fortran-12.2.0-10-any downloading...
 mingw-w64-i686-libgccjit-12.2.0-10-any downloading...
 mingw-w64-i686-gdb-multiarch-13.1-3-any downloading...
 mingw-w64-i686-openssl-3.0.8-1-any downloading...
 mingw-w64-i686-binutils-2.40-2-any downloading...
 mingw-w64-i686-headers-git-10.0.0.r234.g283e5b23a-1-any downloading...
 mingw-w64-i686-gdb-13.1-3-any downloading...
 mingw-w64-i686-gettext-0.21.1-1-any downloading...
 mingw-w64-i686-crt-git-10.0.0.r234.g283e5b23a-1-any downloading...
 mingw-w64-i686-tcl-8.6.12-1-any downloading...
 mingw-w64-i686-tk-8.6.12-1-any downloading...
 mingw-w64-i686-ncurses-6.4.20230211-1-any downloading...
 mingw-w64-i686-sqlite3-3.41.0-1-any downloading...
 mingw-w64-i686-isl-0.25-1-any downloading...
 mingw-w64-i686-curl-7.88.1-1-any downloading...
 mingw-w64-i686-libarchive-3.6.2-2-any downloading...
 mingw-w64-i686-gcc-libs-12.2.0-10-any downloading...
 mingw-w64-i686-libunistring-1.1-1-any downloading...
 mingw-w64-i686-libiconv-1.17-3-any downloading...
 mingw-w64-i686-gcc-libgfortran-12.2.0-10-any downloading...
 mingw-w64-i686-zstd-1.5.4-1-any downloading...
 mingw-w64-i686-xz-5.4.1-1-any downloading...
 mingw-w64-i686-gmp-6.2.1-5-any downloading...
 mingw-w64-i686-ninja-1.11.1-3-any downloading...
 mingw-w64-i686-readline-8.2.001-6-any downloading...
 mingw-w64-i686-mpfr-4.2.0-1-any downloading...
 mingw-w64-i686-brotli-1.0.9-5-any downloading...
 mingw-w64-i686-p11-kit-0.24.1-3-any downloading...
 mingw-w64-i686-libssh2-1.10.0-2-any downloading...
 mingw-w64-i686-ca-certificates-20211016-4-any downloading...
 mingw-w64-i686-mpdecimal-2.5.1-1-any downloading...
 mingw-w64-i686-tools-git-10.0.0.r234.g283e5b23a-1-any downloading...
 mingw-w64-i686-rhash-1.4.3-1-any downloading...
 mingw-w64-i686-tzdata-2022g-1-any downloading...
 mingw-w64-i686-c-ares-1.19.0-1-any downloading...
 mingw-w64-i686-libuv-1.44.2-2-any downloading...
 mingw-w64-i686-nghttp2-1.52.0-1-any downloading...
 mingw-w64-i686-libtasn1-4.19.0-1-any downloading...
 mingw-w64-i686-jsoncpp-1.9.5-2-any downloading...
 mingw-w64-i686-libidn2-2.3.4-1-any downloading...
 mingw-w64-i686-expat-2.5.0-1-any downloading...
 mingw-w64-i686-lz4-1.9.4-1-any downloading...
 mingw-w64-i686-make-4.4-2-any downloading...
 mingw-w64-i686-xxhash-0.8.1-2-any downloading...
 mingw-w64-i686-mpc-1.3.1-1-any downloading...
 mingw-w64-i686-zlib-1.2.13-3-any downloading...
 mingw-w64-i686-libpsl-0.21.2-4-any downloading...
 mingw-w64-i686-bzip2-1.0.8-2-any downloading...
 mingw-w64-i686-libtre-git-r128.6fb7206-2-any downloading...
 mingw-w64-i686-pkgconf-1~1.8.0-2-any downloading...
 mingw-w64-i686-libffi-3.4.4-1-any downloading...
 mingw-w64-i686-winpthreads-git-10.0.0.r234.g283e5b23a-1-any downloading...
 mingw-w64-i686-libwinpthread-git-10.0.0.r234.g283e5b23a-1-any downloading...
 mingw-w64-i686-termcap-1.3.1-6-any downloading...
 mingw-w64-i686-libb2-0.98.1-2-any downloading...
 mingw-w64-i686-libsystre-1.0.1-4-any downloading...
 mingw-w64-i686-winstorecompat-git-10.0.0.r234.g283e5b23a-1-any downloading...
 mingw-w64-i686-libmangle-git-10.0.0.r234.g283e5b23a-1-any downloading...
 mingw-w64-i686-windows-default-manifest-6.4-4-any downloading...
checking package integrity...
loading package files...
checking for file conflicts...
checking available disk space...
checking available disk space...
:: Processing package changes...
installing mingw-w64-i686-libwinpthread-git...
installing mingw-w64-i686-gcc-libs...
installing mingw-w64-i686-zstd...
installing mingw-w64-i686-binutils...
installing mingw-w64-i686-crt-git...
installing mingw-w64-i686-gmp...
installing mingw-w64-i686-isl...
installing mingw-w64-i686-libiconv...
---
installing mingw-w64-i686-gcc-ada...
installing mingw-w64-i686-gcc-libgfortran...
installing mingw-w64-i686-gcc-fortran...
installing mingw-w64-i686-gcc-objc...
installing mingw-w64-i686-expat...
installing mingw-w64-i686-gettext...
installing mingw-w64-i686-libtre-git...
installing mingw-w64-i686-libsystre...
installing mingw-w64-i686-ncurses...
installing mingw-w64-i686-bzip2...
installing mingw-w64-i686-libffi...
installing mingw-w64-i686-mpdecimal...
installing mingw-w64-i686-openssl...
Optional dependencies for mingw-w64-i686-openssl
    mingw-w64-i686-ca-certificates [pending]
installing mingw-w64-i686-termcap...
installing mingw-w64-i686-readline...
installing mingw-w64-i686-tcl...
installing mingw-w64-i686-sqlite3...
installing mingw-w64-i686-tk...
installing mingw-w64-i686-xz...
installing mingw-w64-i686-tzdata...
installing mingw-w64-i686-xxhash...
installing mingw-w64-i686-gdb...
Optional dependencies for mingw-w64-i686-gdb
    mingw-w64-i686-python-pygments: for syntax highlighting
---
installing mingw-w64-i686-make...
installing mingw-w64-i686-pkgconf...
installing mingw-w64-i686-tools-git...
installing mingw-w64-i686-winstorecompat-git...
installing mingw-w64-i686-c-ares...
installing mingw-w64-i686-brotli...
installing mingw-w64-i686-libidn2...
installing mingw-w64-i686-libpsl...
installing mingw-w64-i686-libtasn1...
installing mingw-w64-i686-libtasn1...
installing mingw-w64-i686-p11-kit...
installing mingw-w64-i686-ca-certificates...
installing mingw-w64-i686-libssh2...
installing mingw-w64-i686-nghttp2...
installing mingw-w64-i686-curl...
installing mingw-w64-i686-jsoncpp...
installing mingw-w64-i686-libb2...
installing mingw-w64-i686-lz4...
installing mingw-w64-i686-libuv...
installing mingw-w64-i686-libuv...
installing mingw-w64-i686-ninja...
installing mingw-w64-i686-rhash...
Optional dependencies for mingw-w64-i686-cmake
Optional dependencies for mingw-w64-i686-cmake
    mingw-w64-i686-emacs: for cmake emacs mode
    mingw-w64-i686-qt5-winextras: CMake Qt GUI
src/ci/scripts/install-ninja.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: i686-msvc-2
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
##[group]Run src/ci/scripts/disable-git-crlf-conversion.sh
src/ci/scripts/disable-git-crlf-conversion.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
##[group]Run src/ci/scripts/verify-line-endings.sh
src/ci/scripts/verify-line-endings.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
file:C:/Program Files/Git/etc/gitconfig diff.astextplain.textconv=astextplain
file:C:/Program Files/Git/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
file:C:/Program Files/Git/etc/gitconfig filter.lfs.smudge=git-lfs smudge -- %f
file:C:/Program Files/Git/etc/gitconfig filter.lfs.process=git-lfs filter-process
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
Skipping. This is only run when merging to the beta or stable branches.
##[group]Run src/ci/scripts/verify-stable-version-number.sh
src/ci/scripts/verify-stable-version-number.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
This script only works on the stable channel. Skipping the check.
##[group]Run src/ci/scripts/run-build-from-ci.sh
src/ci/scripts/run-build-from-ci.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
  AWS_SECRET_ACCESS_KEY: ***
  TOOLSTATE_REPO_ACCESS_TOKEN: ***
##[endgroup]
info: removing rustup home
---
-- Build files have been written to: C:/a/rust/rust/build/i686-pc-windows-msvc/llvm/build
running: "cmake" "--build" "." "--target" "install" "--config" "Release" "--" "-j" "8"
[1/3113] Building C object lib\Support\BLAKE3\CMakeFiles\LLVMSupportBlake3.dir\blake3_dispatch.c.obj
[2/3113] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\Demangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/Demangle.cpp.obj 
C:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_FILE_OFFSET_BITS=64 -D_HAS_EXCEPTIONS=0 -D_LARGEFILE_SOURCE -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\lib\Demangle -IC:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\include -IC:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=i686-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Oi /Brepro /bigobj /permissive- /Gw /MT /O2 /Ob2  /EHs-c- /GR- -UNDEBUG -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\Demangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c -- C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\Demangle.cpp
In file included from C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\Demangle.cpp:13:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/Demangle.h:12:
In file included from C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\cstddef:9:
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(807,1): error: static_assert failed "Error in C++ Standard Library usage."
_EMIT_STL_ERROR(STL1000, "Unexpected compiler version, expected Clang 15.0.0 or newer.");
^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(462,5): note: expanded from macro '_EMIT_STL_ERROR'
    static_assert(false, "Error in C++ Standard Library usage.")
    ^             ~~~~~
[3/3113] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\DLangDemangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/DLangDemangle.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/DLangDemangle.cpp.obj 
C:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_FILE_OFFSET_BITS=64 -D_HAS_EXCEPTIONS=0 -D_LARGEFILE_SOURCE -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\lib\Demangle -IC:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\include -IC:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=i686-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Oi /Brepro /bigobj /permissive- /Gw /MT /O2 /Ob2  /EHs-c- /GR- -UNDEBUG -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\DLangDemangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c -- C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\DLangDemangle.cpp
In file included from C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\DLangDemangle.cpp:16:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/Demangle.h:12:
In file included from C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\cstddef:9:
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(807,1): error: static_assert failed "Error in C++ Standard Library usage."
_EMIT_STL_ERROR(STL1000, "Unexpected compiler version, expected Clang 15.0.0 or newer.");
^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(462,5): note: expanded from macro '_EMIT_STL_ERROR'
    static_assert(false, "Error in C++ Standard Library usage.")
    ^             ~~~~~
[4/3113] Building C object lib\Support\BLAKE3\CMakeFiles\LLVMSupportBlake3.dir\blake3.c.obj
[5/3113] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangleNodes.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.obj 
C:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_FILE_OFFSET_BITS=64 -D_HAS_EXCEPTIONS=0 -D_LARGEFILE_SOURCE -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\lib\Demangle -IC:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\include -IC:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=i686-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Oi /Brepro /bigobj /permissive- /Gw /MT /O2 /Ob2  /EHs-c- /GR- -UNDEBUG -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangleNodes.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c -- C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangleNodes.cpp
In file included from C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangleNodes.cpp:13:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/MicrosoftDemangleNodes.h:16:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/StringView.h:20:
In file included from C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\cassert:7:
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(807,1): error: static_assert failed "Error in C++ Standard Library usage."
_EMIT_STL_ERROR(STL1000, "Unexpected compiler version, expected Clang 15.0.0 or newer.");
^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(462,5): note: expanded from macro '_EMIT_STL_ERROR'
    static_assert(false, "Error in C++ Standard Library usage.")
    ^             ~~~~~
[6/3113] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\RustDemangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/RustDemangle.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/RustDemangle.cpp.obj 
C:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_FILE_OFFSET_BITS=64 -D_HAS_EXCEPTIONS=0 -D_LARGEFILE_SOURCE -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\lib\Demangle -IC:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\include -IC:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=i686-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Oi /Brepro /bigobj /permissive- /Gw /MT /O2 /Ob2  /EHs-c- /GR- -UNDEBUG -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\RustDemangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c -- C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\RustDemangle.cpp
In file included from C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\RustDemangle.cpp:14:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/Demangle.h:12:
In file included from C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\cstddef:9:
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(807,1): error: static_assert failed "Error in C++ Standard Library usage."
_EMIT_STL_ERROR(STL1000, "Unexpected compiler version, expected Clang 15.0.0 or newer.");
^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(462,5): note: expanded from macro '_EMIT_STL_ERROR'
    static_assert(false, "Error in C++ Standard Library usage.")
    ^             ~~~~~
[7/3113] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.obj 
C:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_FILE_OFFSET_BITS=64 -D_HAS_EXCEPTIONS=0 -D_LARGEFILE_SOURCE -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\lib\Demangle -IC:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\include -IC:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=i686-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Oi /Brepro /bigobj /permissive- /Gw /MT /O2 /Ob2  /EHs-c- /GR- -UNDEBUG -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c -- C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangle.cpp
In file included from C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\MicrosoftDemangle.cpp:16:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/MicrosoftDemangle.h:12:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/MicrosoftDemangleNodes.h:16:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/StringView.h:20:
In file included from C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\cassert:7:
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(807,1): error: static_assert failed "Error in C++ Standard Library usage."
_EMIT_STL_ERROR(STL1000, "Unexpected compiler version, expected Clang 15.0.0 or newer.");
^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(462,5): note: expanded from macro '_EMIT_STL_ERROR'
    static_assert(false, "Error in C++ Standard Library usage.")
    ^             ~~~~~
[8/3113] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\ItaniumDemangle.cpp.obj
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.obj 
FAILED: lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.obj 
C:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_FILE_OFFSET_BITS=64 -D_HAS_EXCEPTIONS=0 -D_LARGEFILE_SOURCE -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\lib\Demangle -IC:\a\rust\rust\src\llvm-project\llvm\lib\Demangle -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\include -IC:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=i686-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Oi /Brepro /bigobj /permissive- /Gw /MT /O2 /Ob2  /EHs-c- /GR- -UNDEBUG -std:c++14 /showIncludes /Folib\Demangle\CMakeFiles\LLVMDemangle.dir\ItaniumDemangle.cpp.obj /Fdlib\Demangle\CMakeFiles\LLVMDemangle.dir\LLVMDemangle.pdb -c -- C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\ItaniumDemangle.cpp
In file included from C:\a\rust\rust\src\llvm-project\llvm\lib\Demangle\ItaniumDemangle.cpp:13:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Demangle/Demangle.h:12:
In file included from C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\cstddef:9:
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(807,1): error: static_assert failed "Error in C++ Standard Library usage."
_EMIT_STL_ERROR(STL1000, "Unexpected compiler version, expected Clang 15.0.0 or newer.");
^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(462,5): note: expanded from macro '_EMIT_STL_ERROR'
    static_assert(false, "Error in C++ Standard Library usage.")
    ^             ~~~~~
[9/3113] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Process.cpp.obj
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/Process.cpp.obj 
FAILED: lib/Support/CMakeFiles/LLVMSupport.dir/Process.cpp.obj 
C:\a\rust\rust\build\bootstrap\debug\sccache-plus-cl.exe  /nologo -TP -DGTEST_HAS_RTTI=0 -DUNICODE -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_FILE_OFFSET_BITS=64 -D_HAS_EXCEPTIONS=0 -D_LARGEFILE_SOURCE -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\lib\Support -IC:\a\rust\rust\src\llvm-project\llvm\lib\Support -IC:\a\rust\rust\build\i686-pc-windows-msvc\llvm\build\include -IC:\a\rust\rust\src\llvm-project\llvm\include -nologo -MT -Brepro --target=i686-pc-windows-msvc /Zc:inline /Zc:__cplusplus /Oi /Brepro /bigobj /permissive- /Gw  /MT /O2 /Ob2 -UNDEBUG -std:c++14  /EHs-c- /GR- /showIncludes /Folib\Support\CMakeFiles\LLVMSupport.dir\Process.cpp.obj /Fdlib\Support\CMakeFiles\LLVMSupport.dir\LLVMSupport.pdb -c -- C:\a\rust\rust\src\llvm-project\llvm\lib\Support\Process.cpp
In file included from C:\a\rust\rust\src\llvm-project\llvm\lib\Support\Process.cpp:13:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Support/Process.h:27:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/Optional.h:19:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/Hashing.h:47:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm/Support/DataTypes.h:19:
In file included from C:\a\rust\rust\src\llvm-project\llvm\include\llvm-c/DataTypes.h:53:
In file included from C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\cstddef:9:
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(807,1): error: static_assert failed "Error in C++ Standard Library usage."
_EMIT_STL_ERROR(STL1000, "Unexpected compiler version, expected Clang 15.0.0 or newer.");
^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.35.32215\include\yvals_core.h(462,5): note: expanded from macro '_EMIT_STL_ERROR'
    static_assert(false, "Error in C++ Standard Library usage.")
    ^             ~~~~~
1 error generated.
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\index.crates.io-1cd66030c949c28d\cmake-0.1.48\src\lib.rs:975:5
 finished in 52.902 seconds
Build completed unsuccessfully in 0:02:03
Compile requests               105
Compile requests executed      101
---
  TOOLSTATE_PUBLISH: 1
  CACHES_AWS_ACCESS_KEY_ID: AKIA46X5W6CZI5DHEBFL
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  RUST_CONFIGURE_ARGS: --build=i686-pc-windows-msvc --set llvm.clang-cl=/c/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  NO_DOWNLOAD_CI_LLVM: 1
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /c/a/rust/rust/wix
  AWS_SECRET_ACCESS_KEY: ***
##[endgroup]
cp: cannot stat 'build/metrics.json': No such file or directory
##[error]Process completed with exit code 1.
##[error]Process completed with exit code 1.
Post job cleanup.
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.39.2.windows.1
Copying 'C:\Users\runneradmin\.gitconfig' to 'C:\a\_temp\0a74378b-4ebb-4023-90bd-8be347b93544\.gitconfig'
Temporarily overriding HOME='C:\a\_temp\0a74378b-4ebb-4023-90bd-8be347b93544' before making global git config changes
[command]"C:\Program Files\Git\bin\git.exe" config --global --add safe.directory C:\a\rust\rust
[command]"C:\Program Files\Git\bin\git.exe" config --local --name-only --get-regexp core\.sshCommand
[command]"C:\Program Files\Git\bin\git.exe" submodule foreach --recursive "sh -c \"git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :\""
Entering 'library/backtrace'
