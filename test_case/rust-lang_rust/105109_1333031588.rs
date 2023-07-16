plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking remove_dir_all v0.5.3
    Checking winapi-util v0.1.5
The following warnings were emitted during compilation:

warning: src/smhasher/City.cpp:67:10: fatal error: byteswap.h: No such file or directory
warning:  #include <byteswap.h>
warning: compilation terminated.

error: failed to run custom build command for `fasthash-sys v0.3.2`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/fasthash-sys-fd182f8b381613cd/build-script-build` (exit status: 101)
  TARGET = Some("i686-pc-windows-gnu")
  OPT_LEVEL = Some("3")
  TARGET = Some("i686-pc-windows-gnu")
  HOST = Some("x86_64-unknown-linux-gnu")
  HOST = Some("x86_64-unknown-linux-gnu")
  TARGET = Some("i686-pc-windows-gnu")
  TARGET = Some("i686-pc-windows-gnu")
  HOST = Some("x86_64-unknown-linux-gnu")
  CC_i686-pc-windows-gnu = None
  CC_i686_pc_windows_gnu = Some("sccache i686-w64-mingw32-gcc")
  TARGET = Some("i686-pc-windows-gnu")
  HOST = Some("x86_64-unknown-linux-gnu")
  CFLAGS_i686-pc-windows-gnu = None
  CFLAGS_i686_pc_windows_gnu = Some("-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer")
  DEBUG = Some("false")
  running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-m32" "-Wno-implicit-fallthrough" "-Wno-unknown-attributes" "-msse4.2" "-maes" "-mavx" "-mavx2" "-DT1HA0_RUNTIME_SELECT=1" "-DT1HA0_AESNI_AVAILABLE=1" "-Wall" "-Wextra" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/i686-pc-windows-gnu/release/build/fasthash-sys-86a6bee7f30eb0dd/out/src/fasthash.o" "-c" "src/fasthash.cpp"
  TARGET = Some("i686-pc-windows-gnu")
  OPT_LEVEL = Some("3")
  TARGET = Some("i686-pc-windows-gnu")
  HOST = Some("x86_64-unknown-linux-gnu")
  HOST = Some("x86_64-unknown-linux-gnu")
  TARGET = Some("i686-pc-windows-gnu")
  TARGET = Some("i686-pc-windows-gnu")
  HOST = Some("x86_64-unknown-linux-gnu")
  CC_i686-pc-windows-gnu = None
  CC_i686_pc_windows_gnu = Some("sccache i686-w64-mingw32-gcc")
  TARGET = Some("i686-pc-windows-gnu")
  HOST = Some("x86_64-unknown-linux-gnu")
  CFLAGS_i686-pc-windows-gnu = None
  CFLAGS_i686_pc_windows_gnu = Some("-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer")
  DEBUG = Some("false")
  running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-m32" "-Wno-implicit-fallthrough" "-Wno-unknown-attributes" "-msse4.2" "-maes" "-mavx" "-mavx2" "-DT1HA0_RUNTIME_SELECT=1" "-DT1HA0_AESNI_AVAILABLE=1" "-Wall" "-Wextra" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/i686-pc-windows-gnu/release/build/fasthash-sys-86a6bee7f30eb0dd/out/src/smhasher/City.o" "-c" "src/smhasher/City.cpp"
  cargo:warning=src/smhasher/City.cpp:67:10: fatal error: byteswap.h: No such file or directory
  cargo:warning= #include <byteswap.h>
  cargo:warning=          ^~~~~~~~~~~~
  cargo:warning=compilation terminated.

  --- stderr
  thread 'main' panicked at '


  Internal error occurred: Command "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-m32" "-Wno-implicit-fallthrough" "-Wno-unknown-attributes" "-msse4.2" "-maes" "-mavx" "-mavx2" "-DT1HA0_RUNTIME_SELECT=1" "-DT1HA0_AESNI_AVAILABLE=1" "-Wall" "-Wextra" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/i686-pc-windows-gnu/release/build/fasthash-sys-86a6bee7f30eb0dd/out/src/smhasher/City.o" "-c" "src/smhasher/City.cpp" with args "sccache" did not execute successfully (status code exit status: 1).
  ', /cargo/registry/src/github.com-1ecc6299db9ec823/gcc-0.3.55/src/lib.rs:1672:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:42
