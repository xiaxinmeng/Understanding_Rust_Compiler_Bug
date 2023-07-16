plain
[00:07:23] configure: build.locked-deps    := True
[00:07:23] configure: llvm.ccache          := sccache
[00:07:23] configure: build.openssl-static := True
[00:07:23] configure: dist.missing-tools   := True
[00:07:23] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:07:23] configure: writing `config.toml` in current directory
[00:07:23] configure: 
[00:07:23] configure: run `python /checkout/x.py --help`
[00:07:23] configure: 
---
[00:09:26] --- stdout
[00:09:26] OPT_LEVEL = Some("2")
[00:09:26] TARGET = Some("i686-pc-windows-gnu")
[00:09:26] HOST = Some("x86_64-unknown-linux-gnu")
[00:09:26] CC_i686-pc-windows-gnu = Some("sccache i686-w64-mingw32-gcc")
[00:09:26] CFLAGS_i686-pc-windows-gnu = Some("-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer")
[00:09:26] DEBUG = Some("false")
[00:09:26] running: "sh" "/cargo/registry/src/github.com-1ecc6299db9ec823/jemalloc-sys-0.1.4/jemalloc/configure" "--with-jemalloc-prefix=_rjem_" "--host=i686-pc-w64-mingw32" "--build=x86_64-unknown-linux-gnu" "--prefix=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/i686-pc-windows-gnu/release/build/jemalloc-sys-dbcfc730584ebc05/out"
[00:09:26] checking for xsltproc... false
[00:09:26] checking for i686-pc-w64-mingw32-gcc... i686-w64-mingw32-gcc
[00:09:26] checking for C compiler default output file name... a.exe
[00:09:26] checking for suffix of executables... .exe
[00:09:26] checking whether we are cross compiling... yes
[00:09:26] checking for suffix of object files... o
[00:09:26] checking for suffix of object files... o
[00:09:26] checking whether we are using the GNU C compiler... yes
[00:09:26] checking whether i686-w64-mingw32-gcc accepts -g... yes
[00:09:26] checking for i686-w64-mingw32-gcc option to accept ISO C89... none needed
[00:09:26] checking whether compiler is cray... no
[00:09:26] checking whether compiler supports -std=gnu11... yes
[00:09:26] checking whether compiler supports -Wall... yes
[00:09:26] checking whether compiler supports -Werror=declaration-after-statement... yes
[00:09:26] checking whether compiler supports -Wshorten-64-to-32... no
[00:09:26] checking whether compiler supports -Wsign-compare... yes
[00:09:26] checking whether compiler supports -g3... yes
[00:09:26] checking how to run the C preprocessor... i686-w64-mingw32-gcc -E
[00:09:26] checking for grep that handles long lines and -e... /bin/grep
[00:09:26] checking for egrep... /bin/grep -E
---
[00:09:26] checking for strings.h... yes
[00:09:26] checking for inttypes.h... yes
[00:09:26] checking for stdint.h... yes
[00:09:26] checking for unistd.h... yes
[00:09:26] checking whether byte ordering is bigendian... no
[00:09:26] checking size of void *... 4
[00:09:26] checking size of int... 4
[00:09:26] checking size of long... 4
[00:09:26] checking size of long long... 8
[00:09:26] checking size of intmax_t... 8
[00:09:26] checking host system type... 
[00:09:26] --- stderr
[00:09:26] --- stderr
[00:09:26] Invalid configuration `i686-pc-w64-mingw32': machine `i686-pc-w64' not recognized
[00:09:26] configure: error: /bin/bash /cargo/registry/src/github.com-1ecc6299db9ec823/jemalloc-sys-0.1.4/jemalloc/build-aux/config.sub i686-pc-w64-mingw32 failed
[00:09:26] thread 'main' panicked at 'command did not execute successfully: "sh" "/cargo/registry/src/github.com-1ecc6299db9ec823/jemalloc-sys-0.1.4/jemalloc/configure" "--with-jemalloc-prefix=_rjem_" "--host=i686-pc-w64-mingw32" "--build=x86_64-unknown-linux-gnu" "--prefix=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/i686-pc-windows-gnu/release/build/jemalloc-sys-dbcfc730584ebc05/out"
[00:09:26] expected success, got: exit code: 1', /cargo/registry/src/github.com-1ecc6299db9ec823/jemalloc-sys-0.1.4/build.rs:105:9
[00:09:26] 
[00:09:26] warning: build failed, waiting for other jobs to finish...
[00:09:27] error: build failed
[00:09:27] error: build failed
[00:09:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:09:27] expected success, got: exit code: 101
[00:09:27] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:09:27] travis_fold:end:stage0-rustc

[00:09:27] travis_time:end:stage0-rustc:start=1540005358889866859,finish=1540005379684106728,duration=20794239869

---
travis_time:end:11800f50:start=1540005380369558572,finish=1540005380374163932,duration=4605360
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09df4640
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:018050d2
travis_time:start:018050d2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:096bd2bc
$ dmesg | grep -i kill
