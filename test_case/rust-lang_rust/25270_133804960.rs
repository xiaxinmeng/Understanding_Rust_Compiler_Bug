
$ make VERBOSE=1
cfg: version 1.4.0-dev (5e5b99f47 2015-08-22)
cfg: build triple x86_64-pc-windows-gnu
cfg: host triples x86_64-pc-windows-gnu
cfg: target triples x86_64-pc-windows-gnu
cfg: enabling debug assertions (CFG_ENABLE_DEBUG_ASSERTIONS)
cfg: enabling debuginfo (CFG_ENABLE_DEBUGINFO)
cfg: host for x86_64-pc-windows-gnu is x86_64
cfg: os for x86_64-pc-windows-gnu is pc-windows-gnu
cfg: good valgrind for x86_64-pc-windows-gnu is
cfg: using CC=gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'std-*.dll\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'libstd-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/rust/x86_64-pc-windows-gnu/rt/llvmdeps.rs PATH=/home/rust/x86_64-pc-windows-gnu/stage1/bin:$PATH   x86_64-pc-windows-gnu/stage1/bin/rustc.exe --cfg stage1  -O --cfg rtopt -C debug-assertions=on -g -C prefer-dynamic --target=x86_64-pc-windows-gnu  -D warnings -L "x86_64-pc-windows-gnu/rt" -L "C:\msys64\home\rust\x86_64-pc-windows-gnu\llvm\Release+Asserts/lib"    --out-dir x86_64-pc-windows-gnu/stage1/bin/rustlib/x86_64-pc-windows-gnu/lib -C extra-filename=-a5fc0d6c src/libstd/lib.rs
/home/rust/mk/target.mk:163: ошибка выполнения рецепта для цели «x86_64-pc-windows-gnu/stage1/bin/rustlib/x86_64-pc-windows-gnu/lib/stamp.std»
make: *** [x86_64-pc-windows-gnu/stage1/bin/rustlib/x86_64-pc-windows-gnu/lib/stamp.std] Segmentation fault
