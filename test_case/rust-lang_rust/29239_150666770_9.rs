 bash
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --disable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust --release-channel=dev --host=x86_64-unknown-linux-gnu --target=x86_64-unknown-linux-gnu --build=x86_64-unknown-linux-gnu
...
$ time make -j1 -- VERBOSE=1 NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose -C debug-assertions=y -Ccodegen-units=4' RUST_BACKTRACE=1
...
cfg: version 1.5.0-dev (9a855668f 2015-10-23)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: enabling debug assertions (CFG_ENABLE_DEBUG_ASSERTIONS)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
touch /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/llvm-auto-clean-stamp.start_time
make clean-llvmx86_64-unknown-linux-gnu
make[1]: Entering directory '/home/zazdxscf/build/1nonpkgs/rust/rust'
...
cp x86_64-unknown-linux-gnu/rt/libcompiler-rt.a x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler-rt.a
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'libcore-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'libcore-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage0/bin/rustc --cfg stage0 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose -C debug-assertions=y -Ccodegen-units=4 -O --cfg rtopt -C debug-assertions=on -Z time-passes -Z time-llvm-passes -C prefer-dynamic -C no-stack-check --target=x86_64-unknown-linux-gnu  -W warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib"     --out-dir x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/libcore/lib.rs
time: 0.229; rss: 44MB  parsing
time: 0.077; rss: 45MB  configuration 1
time: 0.000; rss: 45MB  recursion limit
time: 0.006; rss: 45MB  gated macro checking
time: 0.000; rss: 45MB  crate injection
time: 0.003; rss: 45MB  macro loading
time: 0.000; rss: 45MB  plugin loading
time: 0.000; rss: 45MB  plugin registration
time: 1.428; rss: 73MB  expansion
time: 0.026; rss: 73MB  complete gated feature checking 1
time: 0.296; rss: 73MB  configuration 2
time: 0.149; rss: 73MB  maybe building test harness
time: 0.000; rss: 73MB  prelude injection
time: 0.013; rss: 73MB  checking that all macro invocations are gone
time: 0.027; rss: 73MB  complete gated feature checking 2
time: 0.167; rss: 79MB  assigning node ids and indexing ast
time: 0.013; rss: 79MB  external crate/lib resolution
time: 0.029; rss: 79MB  language item collection
time: 0.260; rss: 97MB  resolution
time: 0.024; rss: 94MB  lifetime resolution
time: 0.000; rss: 94MB  looking for entry point
time: 0.014; rss: 94MB  looking for plugin registrar
time: 0.101; rss: 105MB region resolution
time: 0.013; rss: 105MB loop checking
time: 0.013; rss: 105MB static item recursion checking
time: 0.213; rss: 112MB type collecting
time: 0.028; rss: 112MB variance inference
time: 2.148; rss: 123MB coherence checking
time: 16.765; rss: 169MB    type checking
time: 1.222; rss: 170MB const checking
time: 0.132; rss: 170MB privacy checking
time: 0.015; rss: 170MB stability index
time: 0.096; rss: 170MB intrinsic checking
time: 0.024; rss: 170MB effect checking
time: 0.349; rss: 170MB match checking
time: 0.056; rss: 170MB liveness checking
time: 2.390; rss: 170MB borrow checking
time: 1.210; rss: 171MB rvalue checking
time: 0.083; rss: 171MB reachability checking
time: 0.091; rss: 172MB death checking
time: 0.077; rss: 172MB stability checking
time: 0.000; rss: 172MB unused lib feature checking
src/libcore/lib.rs:82:12: 82:29 warning: unused or unknown feature, #[warn(unused_features)] on by default
src/libcore/lib.rs:82 #![feature(unwind_attributes)]
                                 ^~~~~~~~~~~~~~~~~
src/libcore/ptr.rs:21:11: 21:24 warning: unused import, #[warn(unused_imports)] on by default
src/libcore/ptr.rs:21 use ops::{CoerceUnsized, Deref};
                                ^~~~~~~~~~~~~
src/libcore/ptr.rs:25:52: 25:58 warning: unused import, #[warn(unused_imports)] on by default
src/libcore/ptr.rs:25 use marker::{Copy, PhantomData, Send, Sized, Sync, Unsize};
                                                                         ^~~~~~
src/libcore/panicking.rs:65:9: 65:18 warning: unused attribute, #[warn(unused_attributes)] on by default
src/libcore/panicking.rs:65         #[unwind]
                                    ^~~~~~~~~
time: 1.525; rss: 172MB lint checking
time: 0.000; rss: 172MB resolving dependency formats
time: 6.468; rss: 301MB translation
Pass Arguments:  -tti -targetlibinfo -no-aa -tbaa -scoped-noalias -assumption-cache-tracker -basicaa -verify -simplifycfg -domtree -sroa -early-cse -lower-expect
Target Transform Information
Target Library Information
No Alias Analysis (always returns 'may' alias)
Type-Based Alias Analysis
Scoped NoAlias Alias AnalysisPass Arguments:  -tti -targetlibinfo -no-aa -tbaa -
scoped-noalias -Assumption Cache Tracker
assumption-cache-trackerBasic Alias Analysis (stateless AA impl)Pass Arguments:  -tti -targetlibinfo
 -  FunctionPass Manager
basicaa -    Module Verifierno-aa -verify - -simplifycfgtbaa -domtree -scoped-noalias -sroa -assumption-cache-tracker - -early-csebasicaa - -lower-expectverify
 -simplifycfgTarget Transform Information

 -domtree -Target Library Informationsroa -
early-cse -lower-expect
Target Transform Information
Target Library InformationPass Arguments:  -ttiNo Alias Analysis (always returns 'may' alias)
    Simplify the CFG
No Alias Analysis (always returns 'may' alias)
Type-Based Alias Analysis
Scoped NoAlias Alias Analysis
Assumption Cache Tracker
Basic Alias Analysis (stateless AA impl)
  FunctionPass Manager
    Module Verifier
    Simplify the CFG
    Dominator Tree Construction
    SROA
    Early CSE
    Lower 'expect' Intrinsics
 -
    Dominator Tree Construction
    SROA
    Early CSE
    Lower 'expect' Intrinsics
targetlibinfo -Type-Based Alias Analysisno-aa -tbaa -
Scoped NoAlias Alias Analysis
Assumption Cache Tracker
Basic Alias Analysis (stateless AA impl)
  FunctionPass Manager
    Module Verifier
    Simplify the CFG
    Dominator Tree Construction
    SROA
    Early CSE
    Lower 'expect' Intrinsics
scoped-noalias -assumption-cache-tracker -basicaa -verify -simplifycfg -domtree -sroa -early-cse -lower-expect
Target Transform Information
Target Library Information
No Alias Analysis (always returns 'may' alias)
Type-Based Alias Analysis
Scoped NoAlias Alias Analysis
Assumption Cache Tracker
Basic Alias Analysis (stateless AA impl)
  FunctionPass Manager
    Module Verifier
    Simplify the CFG
    Dominator Tree Construction
    SROA
    Early CSE
    Lower 'expect' Intrinsics
rustc: malloc.c:2921: __libc_malloc: Assertion `!victim || ((((mchunkptr)((char*)(victim) - 2*(sizeof(size_t)))))->size & 0x2) || ar_ptr == (((((mchunkptr)((char*)(victim) - 2*(sizeof(size_t)))))->size & 0x4) ? ((heap_info *) ((unsigned long) (((mchunkptr)((char*)(victim) - 2*(sizeof(size_t))))) & ~((2 * (4 * 1024 * 1024 * sizeof(long))) - 1)))->ar_ptr : &main_arena)' failed.
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.core' failed
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.core] Aborted (core dumped)

real    5m4.755s
user    8m9.393s
sys 4m20.027s

