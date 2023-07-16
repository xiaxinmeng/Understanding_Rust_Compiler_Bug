 bash
$ time make -j4 RUST_BACKTRACE=1 VERBOSE=1
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
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1   -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_front/lib.rs
info: now are following matches for librustc_front-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-bb943c5a.so
info: now are following matches for librustc_front-*.rlib libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-bb943c5a.rlib
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_back-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_back-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1   -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_back/lib.rs
cp x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front x86_64-unknown-linux-gnu/stage2/lib/stamp.rustc_front
cp -R x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-*.so x86_64-unknown-linux-gnu/stage2/lib
info: now are following matches for librustc_front-*.so libraries:
x86_64-unknown-linux-gnu/stage2/lib/librustc_front-bb943c5a.so
info: now are following matches for librustc_back-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_back-bb943c5a.so
info: now are following matches for librustc_back-*.rlib libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_back-bb943c5a.rlib
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_back-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1   -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc/lib.rs
cp x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_back x86_64-unknown-linux-gnu/stage2/lib/stamp.rustc_back
cp -R x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_back-*.so x86_64-unknown-linux-gnu/stage2/lib
info: now are following matches for librustc_back-*.so libraries:
x86_64-unknown-linux-gnu/stage2/lib/librustc_back-bb943c5a.so
info: now are following matches for librustc-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-bb943c5a.so
info: now are following matches for librustc-*.rlib libraries:
x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-bb943c5a.rlib
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_borrowck-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_borrowck-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1   -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_borrowck/lib.rs
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_platform_intrinsics-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_platform_intrinsics-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1   -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_platform_intrinsics/lib.rs
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_mir-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_mir-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1   -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_mir/lib.rs
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_resolve-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_resolve-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1   -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/librustc_resolve/lib.rs
...
//I C-c ed  it after a while
real    25m48.675s
user    27m8.817s
sys 0m17.803s
