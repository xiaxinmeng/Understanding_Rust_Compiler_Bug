 bash
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 'RUSTFLAGS=--verbose' RUST_BACKTRACE=1
cfg: version 1.5.0-dev (9a855668f 2015-10-23)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: disabling rustc optimization (CFG_DISABLE_OPTIMIZE)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: disabling C++ optimization (CFG_DISABLE_OPTIMIZE_CXX)
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose  -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libstd-bb943c5a.rlib" => Duration { secs: 0, nanos: 308489 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcore-bb943c5a.rlib" => Duration { secs: 0, nanos: 193600 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcollections-bb943c5a.rlib" => Duration { secs: 0, nanos: 209245 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_unicode-bb943c5a.rlib" => Duration { secs: 0, nanos: 192133 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc-bb943c5a.rlib" => Duration { secs: 0, nanos: 231245 }
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_system-bb943c5a.rlib" => Duration { secs: 0, nanos: 204356 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 323156 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librand-bb943c5a.rlib" => Duration { secs: 0, nanos: 201912 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblibc-bb943c5a.rlib" => Duration { secs: 0, nanos: 184800 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 314356 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblog-bb943c5a.rlib" => Duration { secs: 0, nanos: 257644 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libsyntax-bb943c5a.rlib" => Duration { secs: 0, nanos: 294800 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libfmt_macros-bb943c5a.rlib" => Duration { secs: 0, nanos: 235645 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libserialize-bb943c5a.rlib" => Duration { secs: 0, nanos: 200445 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libterm-bb943c5a.rlib" => Duration { secs: 0, nanos: 239555 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_bitflags-bb943c5a.rlib" => Duration { secs: 0, nanos: 230756 }
info: now are following matches for librustc_front-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-bb943c5a.so
info: now are following matches for librustc_front-*.rlib libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-bb943c5a.rlib
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_back-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_back-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose  -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_back/lib.rs
cp x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front x86_64-unknown-linux-gnu/stage2/lib/stamp.rustc_front
cp -R x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-*.so x86_64-unknown-linux-gnu/stage2/lib
info: now are following matches for librustc_front-*.so libraries:
x86_64-unknown-linux-gnu/stage2/lib/librustc_front-bb943c5a.so
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libstd-bb943c5a.rlib" => Duration { secs: 0, nanos: 311422 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcore-bb943c5a.rlib" => Duration { secs: 0, nanos: 191644 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcollections-bb943c5a.rlib" => Duration { secs: 0, nanos: 198489 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_unicode-bb943c5a.rlib" => Duration { secs: 0, nanos: 181867 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 337823 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_system-bb943c5a.rlib" => Duration { secs: 0, nanos: 186755 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc-bb943c5a.rlib" => Duration { secs: 0, nanos: 202400 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librand-bb943c5a.rlib" => Duration { secs: 0, nanos: 197511 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblibc-bb943c5a.rlib" => Duration { secs: 0, nanos: 196044 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 336844 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblog-bb943c5a.rlib" => Duration { secs: 0, nanos: 240045 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libsyntax-bb943c5a.rlib" => Duration { secs: 0, nanos: 278666 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libfmt_macros-bb943c5a.rlib" => Duration { secs: 0, nanos: 229778 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libserialize-bb943c5a.rlib" => Duration { secs: 0, nanos: 195556 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libterm-bb943c5a.rlib" => Duration { secs: 0, nanos: 247866 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_llvm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_llvm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_llvm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_llvm-bb943c5a.rlib" => Duration { secs: 0, nanos: 4669379 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_front-bb943c5a.rlib" => Duration { secs: 0, nanos: 284534 }
info: now are following matches for librustc_back-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_back-bb943c5a.so
info: now are following matches for librustc_back-*.rlib libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_back-bb943c5a.rlib
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_back-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose  -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc/lib.rs
cp x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_back x86_64-unknown-linux-gnu/stage2/lib/stamp.rustc_back
cp -R x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_back-*.so x86_64-unknown-linux-gnu/stage2/lib
info: now are following matches for librustc_back-*.so libraries:
x86_64-unknown-linux-gnu/stage2/lib/librustc_back-bb943c5a.so
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libstd-bb943c5a.rlib" => Duration { secs: 0, nanos: 309467 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcore-bb943c5a.rlib" => Duration { secs: 0, nanos: 191644 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libcollections-bb943c5a.rlib" => Duration { secs: 0, nanos: 225378 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_unicode-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_unicode-bb943c5a.rlib" => Duration { secs: 0, nanos: 235155 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_system-bb943c5a.rlib" => Duration { secs: 0, nanos: 225867 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 331466 }
INFO:rustc::metadata::loader: Rejecting via crate name
INFO:rustc::metadata::loader: metadata mismatch
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc-bb943c5a.rlib" => Duration { secs: 0, nanos: 164756 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librand-bb943c5a.rlib" => Duration { secs: 0, nanos: 202400 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblibc-bb943c5a.rlib" => Duration { secs: 0, nanos: 208755 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liballoc_jemalloc-bb943c5a.rlib" => Duration { secs: 0, nanos: 306044 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblog-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "liblog-bb943c5a.rlib" => Duration { secs: 0, nanos: 265466 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libsyntax-bb943c5a.rlib" => Duration { secs: 0, nanos: 248845 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libfmt_macros-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libfmt_macros-bb943c5a.rlib" => Duration { secs: 0, nanos: 195067 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserialize-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libserialize-bb943c5a.rlib" => Duration { secs: 0, nanos: 228800 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libterm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libterm-bb943c5a.rlib" => Duration { secs: 0, nanos: 185777 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_bitflags-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_bitflags-bb943c5a.rlib" => Duration { secs: 0, nanos: 250311 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libarena-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libarena-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libarena-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libarena-bb943c5a.rlib" => Duration { secs: 0, nanos: 293822 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libflate-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libflate-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libflate-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libflate-bb943c5a.rlib" => Duration { secs: 0, nanos: 213644 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgetopts-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libgetopts-bb943c5a.rlib" => Duration { secs: 0, nanos: 196533 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgraphviz-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgraphviz-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgraphviz-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "libgraphviz-bb943c5a.rlib" => Duration { secs: 0, nanos: 212666 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_llvm-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_llvm-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_llvm-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_llvm-bb943c5a.rlib" => Duration { secs: 0, nanos: 4839023 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_back-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_back-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_back-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_back-bb943c5a.rlib" => Duration { secs: 0, nanos: 288444 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_front-bb943c5a.rlib" => Duration { secs: 0, nanos: 193111 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_data_structures-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_data_structures-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_data_structures-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librustc_data_structures-bb943c5a.rlib" => Duration { secs: 0, nanos: 287467 }
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librbml-bb943c5a.rlib
INFO:rustc::metadata::loader: lib candidate: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librbml-bb943c5a.so
INFO:rustc::metadata::loader: rlib reading metadata from: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librbml-bb943c5a.rlib
INFO:rustc::metadata::loader: reading "librbml-bb943c5a.rlib" => Duration { secs: 0, nanos: 263511 }
...
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose  -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_resolve/lib.rs
...
//press C-c after a while
real    32m46.757s
user    36m57.460s
sys 0m18.123s

