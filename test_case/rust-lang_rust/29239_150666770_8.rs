 bash
$ time make -j1 -- VERBOSE=1 NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose -C debug-assertions=y -Ccodegen-units=4' RUST_BACKTRACE=1
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
/usr/bin/python2 /home/zazdxscf/build/1nonpkgs/rust/rust/src/etc/get-snapshot.py x86_64-unknown-linux-gnu 
determined most recent snapshot: rust-stage0-2015-08-11-1af31d4-linux-x86_64-7df8ba9dec63ec77b857066109d4b6250f3d222f.tar.bz2
got download with ok hash
...
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage0/bin/rustc --cfg stage0 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes -Z verbose -C debug-assertions=y -Ccodegen-units=4 -O --cfg rtopt -C debug-assertions=on -Z time-passes -Z time-llvm-passes -C prefer-dynamic -C no-stack-check --target=x86_64-unknown-linux-gnu  -W warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib"     --out-dir x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/libcore/lib.rs
time: 0.228; rss: 43MB  parsing
time: 0.075; rss: 45MB  configuration 1
time: 0.000; rss: 45MB  recursion limit
time: 0.006; rss: 45MB  gated macro checking
time: 0.000; rss: 45MB  crate injection
time: 0.003; rss: 45MB  macro loading
time: 0.000; rss: 45MB  plugin loading
time: 0.000; rss: 45MB  plugin registration
time: 1.413; rss: 72MB  expansion
time: 0.026; rss: 72MB  complete gated feature checking 1
time: 0.294; rss: 73MB  configuration 2
time: 0.149; rss: 73MB  maybe building test harness
time: 0.000; rss: 73MB  prelude injection
time: 0.013; rss: 73MB  checking that all macro invocations are gone
time: 0.026; rss: 73MB  complete gated feature checking 2
time: 0.168; rss: 78MB  assigning node ids and indexing ast
time: 0.014; rss: 78MB  external crate/lib resolution
time: 0.027; rss: 78MB  language item collection
time: 0.261; rss: 96MB  resolution
time: 0.024; rss: 93MB  lifetime resolution
time: 0.000; rss: 93MB  looking for entry point
time: 0.013; rss: 93MB  looking for plugin registrar
time: 0.101; rss: 105MB region resolution
time: 0.013; rss: 105MB loop checking
time: 0.014; rss: 105MB static item recursion checking
time: 0.210; rss: 111MB type collecting
time: 0.027; rss: 111MB variance inference
time: 2.126; rss: 122MB coherence checking
time: 16.711; rss: 169MB    type checking
time: 1.228; rss: 170MB const checking
time: 0.132; rss: 170MB privacy checking
time: 0.015; rss: 170MB stability index
time: 0.095; rss: 170MB intrinsic checking
time: 0.024; rss: 170MB effect checking
time: 0.351; rss: 170MB match checking
time: 0.057; rss: 170MB liveness checking
time: 2.406; rss: 170MB borrow checking
time: 1.211; rss: 170MB rvalue checking
time: 0.082; rss: 171MB reachability checking
time: 0.089; rss: 172MB death checking
time: 0.073; rss: 172MB stability checking
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
time: 1.489; rss: 172MB lint checking
time: 0.000; rss: 172MB resolving dependency formats
time: 6.426; rss: 301MB translation
Pass Arguments: Pass Arguments: Pass Arguments:  -tti -targetlibinfo -no-aa -tbaa -scoped-noalias -assumption-cache-tracker -basicaa -verify -simplifycfg -domtree -sroa -tti - -targetlibinfotti - -no-aa -early-csetargetlibinfo -lower-expect
 - -no-aaTarget Transform Informationtbaa - -scoped-noalias -tbaa -assumption-cache-tracker -
Target Library Informationscoped-noalias
No Alias Analysis (always returns 'may' alias) -assumption-cache-trackerbasicaa - -basicaa
Type-Based Alias Analysisverify -verify -simplifycfg
Scoped NoAlias Alias Analysis -simplifycfg
Assumption Cache Tracker - -domtreedomtree - -sroa
Basic Alias Analysis (stateless AA impl)sroa -early-cse -early-cse
   -FunctionPass Manager
lower-expect -lower-expect

Target Transform Information
Module VerifierTarget Transform Information

Target Library Information    
Target Library InformationNo Alias Analysis (always returns 'may' alias)
Simplify the CFG

    Dominator Tree ConstructionNo Alias Analysis (always returns 'may' alias)

SROAType-Based Alias AnalysisType-Based Alias Analysis
Scoped NoAlias Alias Analysis

Assumption Cache TrackerScoped NoAlias Alias Analysis


Early CSE
Basic Alias Analysis (stateless AA impl)    Lower 'expect' Intrinsics
Assumption Cache Tracker

FunctionPass Manager
Basic Alias Analysis (stateless AA impl)
    Module Verifier  
FunctionPass Manager
    Simplify the CFG    
Module Verifier
        Dominator Tree Construction
Simplify the CFG
    SROA    
    Dominator Tree Construction
Early CSE
    SROA    Lower 'expect' Intrinsics

    Early CSE
    Lower 'expect' Intrinsics
Pass Arguments:  -tti -targetlibinfo -no-aa -tbaa -/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.core' failed
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.core] Killed (core dumped)

real    0m56.018s
user    0m47.537s
sys 0m6.397s
