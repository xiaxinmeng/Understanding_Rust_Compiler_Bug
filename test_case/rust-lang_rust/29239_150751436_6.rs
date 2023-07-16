 bash
$ rm /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_front
$ time RUST_LOG=rustc::metadata::loader make -j4 -- VERBOSE=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z print-llvm-passes' RUST_BACKTRACE=1
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
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_front-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_front-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'librustc_front-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_front-bb943c5a.so
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_front x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_front
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_front-*.so x86_64-unknown-linux-gnu/stage1/lib
info: now are following matches for librustc_front-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/librustc_front-bb943c5a.so
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_back-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_back-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'librustc_back-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_back-bb943c5a.so
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_back x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_back
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_back-*.so x86_64-unknown-linux-gnu/stage1/lib
info: now are following matches for librustc_back-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/librustc_back-bb943c5a.so
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'librustc-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc-bb943c5a.so
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-*.so x86_64-unknown-linux-gnu/stage1/lib
info: now are following matches for librustc-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/librustc-bb943c5a.so
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_borrowck-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_borrowck-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_platform_intrinsics-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_platform_intrinsics-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'librustc_borrowck-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_borrowck-bb943c5a.so
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_borrowck x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_borrowck
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_borrowck-*.so x86_64-unknown-linux-gnu/stage1/lib
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_mir-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_mir-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'librustc_platform_intrinsics-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_platform_intrinsics-bb943c5a.so
info: now are following matches for librustc_borrowck-*.so libraries:
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_platform_intrinsics x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_platform_intrinsics
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_platform_intrinsics-*.so x86_64-unknown-linux-gnu/stage1/lib
x86_64-unknown-linux-gnu/stage1/lib/librustc_borrowck-bb943c5a.so
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_resolve-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_resolve-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'librustc_mir-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_mir-bb943c5a.so
info: now are following matches for librustc_platform_intrinsics-*.so libraries:
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_mir x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_mir
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_mir-*.so x86_64-unknown-linux-gnu/stage1/lib
x86_64-unknown-linux-gnu/stage1/lib/librustc_platform_intrinsics-bb943c5a.so
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_trans-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_trans-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'librustc_resolve-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_resolve-bb943c5a.so
info: now are following matches for librustc_mir-*.so libraries:
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_resolve x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_resolve
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_resolve-*.so x86_64-unknown-linux-gnu/stage1/lib
x86_64-unknown-linux-gnu/stage1/lib/librustc_mir-bb943c5a.so
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_privacy-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_privacy-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'librustc_trans-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_trans-bb943c5a.so
info: now are following matches for librustc_resolve-*.so libraries:
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_trans x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_trans
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_trans-*.so x86_64-unknown-linux-gnu/stage1/lib
x86_64-unknown-linux-gnu/stage1/lib/librustc_resolve-bb943c5a.so
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_lint-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_lint-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'librustc_privacy-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_privacy-bb943c5a.so
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_privacy x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_privacy
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_privacy-*.so x86_64-unknown-linux-gnu/stage1/lib
info: now are following matches for librustc_trans-*.so libraries:
warning: removing previous 'librustc_lint-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_lint-bb943c5a.so
info: now are following matches for librustc_privacy-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/librustc_trans-bb943c5a.so
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_lint x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_lint
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_typeck-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_typeck-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
x86_64-unknown-linux-gnu/stage1/lib/librustc_privacy-bb943c5a.so
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lint-*.so x86_64-unknown-linux-gnu/stage1/lib
info: now are following matches for librustc_lint-*.so libraries:
warning: removing previous 'librustc_typeck-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_typeck-bb943c5a.so
x86_64-unknown-linux-gnu/stage1/lib/librustc_lint-bb943c5a.so
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_typeck x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_typeck
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_typeck-*.so x86_64-unknown-linux-gnu/stage1/lib
info: now are following matches for librustc_typeck-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/librustc_typeck-bb943c5a.so
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-bb943c5a.so"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'librustc_driver-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'librustc_driver-*.so' libraries: x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-bb943c5a.so
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.rustc_driver x86_64-unknown-linux-gnu/stage1/lib/stamp.rustc_driver
cp -R x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_driver-*.so x86_64-unknown-linux-gnu/stage1/lib
info: now are following matches for librustc_driver-*.so libraries:
x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-bb943c5a.so
cp x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin/rustc x86_64-unknown-linux-gnu/stage1/bin/rustc
MATCHES=""; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'libcore-*.so\' "libraries:" $MATCHES; rm $MATCHES ; fi
MATCHES="x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib"; if [ -n "$MATCHES" ] ; then echo "warning: removing previous" \'libcore-*.rlib\' "libraries:" $MATCHES; rm $MATCHES ; fi
warning: removing previous 'libcore-*.rlib' libraries: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bb943c5a.rlib
CFG_LLVM_LINKAGE_FILE=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/lib:$LD_LIBRARY_PATH   x86_64-unknown-linux-gnu/stage1/bin/rustc --cfg stage1 --verbose -Z verbose -Z print-link-args -Z print-llvm-passes  -Z time-passes -Z time-llvm-passes -C prefer-dynamic --target=x86_64-unknown-linux-gnu  -D warnings -L "x86_64-unknown-linux-gnu/rt" -L native="/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release/lib"     --out-dir x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib -C extra-filename=-bb943c5a src/libcore/lib.rs
time: 3.182; rss: 51MB  parsing
time: 0.608; rss: 51MB  configuration 1
time: 0.000; rss: 51MB  recursion limit
time: 0.113; rss: 51MB  gated macro checking
time: 0.000; rss: 51MB  crate injection
time: 0.021; rss: 51MB  macro loading
time: 0.000; rss: 51MB  plugin loading
time: 0.000; rss: 51MB  plugin registration
time: 15.385; rss: 81MB expansion
time: 0.581; rss: 81MB  complete gated feature checking 1
time: 4.190; rss: 81MB  configuration 2
time: 0.000; rss: 81MB  gated configuration checking
time: 2.190; rss: 81MB  maybe building test harness
time: 0.000; rss: 81MB  prelude injection
time: 0.098; rss: 81MB  checking that all macro invocations are gone
time: 0.000; rss: 81MB  checking for inline asm in case the target doesn't support it
time: 0.582; rss: 81MB  complete gated feature checking 2
time: 2.125; rss: 81MB  assigning node ids
time: 0.483; rss: 116MB lowering ast -> hir
time: 5.242; rss: 128MB indexing hir
time: 0.001; rss: 128MB attribute checking
time: 0.554; rss: 128MB early lint checks
time: 0.083; rss: 128MB external crate/lib resolution
time: 0.177; rss: 128MB language item collection
time: 3.882; rss: 147MB resolution
time: 0.329; rss: 146MB lifetime resolution
time: 0.000; rss: 146MB looking for entry point
time: 0.085; rss: 146MB looking for plugin registrar
time: 1.263; rss: 156MB region resolution
time: 0.086; rss: 156MB loop checking
time: 0.087; rss: 156MB static item recursion checking
time: 4.253; rss: 174MB type collecting
time: 0.216; rss: 174MB variance inference
time: 54.978; rss: 176MB    coherence checking
time: 39.796; rss: 179MB    wf checking (old)
time: 46.695; rss: 183MB    item-types checking
time: 307.045; rss: 212MB   item-bodies checking
time: 0.001; rss: 212MB drop-impl checking
time: 68.719; rss: 212MB    wf checking (new)
time: 11.501; rss: 212MB    const checking
time: 1.178; rss: 212MB privacy checking
time: 0.223; rss: 212MB stability index
time: 1.929; rss: 212MB intrinsic checking
time: 0.193; rss: 212MB effect checking
time: 3.241; rss: 212MB match checking
time: 11.913; rss: 273MB    MIR dump
time: 1.087; rss: 273MB liveness checking
time: 28.726; rss: 275MB    borrow checking
time: 13.648; rss: 275MB    rvalue checking
time: 1.866; rss: 276MB reachability checking
time: 0.927; rss: 277MB death checking
time: 1.406; rss: 277MB stability checking
time: 0.000; rss: 277MB unused lib feature checking
time: 37.869; rss: 277MB    lint checking
time: 0.000; rss: 277MB resolving dependency formats
time: 48.860; rss: 346MB    translation
Pass Arguments:  -tti -targetlibinfo -verify
Target Transform Information
Target Library Information
  FunctionPass Manager
    Module Verifier
  time: 0.164; rss: 322MB   llvm function passes
Pass Arguments:  -tti -assumption-cache-tracker -no-aa -basiccg -inline-cost -always-inline
Target Transform Information
Assumption Cache Tracker
No Alias Analysis (always returns 'may' alias)
  ModulePass Manager
    CallGraph Construction
    Call Graph SCC Pass Manager
      Inline Cost Analysis
      Inliner for always_inline functions
  time: 0.175; rss: 322MB   llvm module passes
Pass Arguments:  -tti -targetlibinfo -targetpassconfig -no-aa -tbaa -scoped-noalias -assumption-cache-tracker -basicaa -collector-metadata -machinemoduleinfo -atomic-expand -verify -gc-lowering -shadow-stack-gc-lowering -unreachableblockelim -rewrite-symbols -domtree -dwarfehprepare -safe-stack -stack-protector -verify -expand-isel-pseudos -localstackalloc -phi-node-elimination -twoaddressinstruction -edge-bundles -prologepilog -postrapseudos -gc-analysis -stackmap-liveness
Target Transform Information
Target Library Information
Target Pass Configuration
No Alias Analysis (always returns 'may' alias)
Type-Based Alias Analysis
Scoped NoAlias Alias Analysis
Assumption Cache Tracker
Basic Alias Analysis (stateless AA impl)
Create Garbage Collector Module Metadata
Machine Module Information
  ModulePass Manager
    FunctionPass Manager
      Expand Atomic calls in terms of either load-linked & store-conditional or cmpxchg
      Module Verifier
      Lower Garbage Collection Instructions
      Shadow Stack GC Lowering
      Remove unreachable blocks from the CFG
    Rewrite Symbols
    FunctionPass Manager
      Dominator Tree Construction
      Exception handling preparation
      Safe Stack instrumentation pass
      Insert stack protectors
      Module Verifier
      Machine Function Analysis
      X86 DAG->DAG Instruction Selection
      X86 PIC Global Base Reg Initialization
      Expand ISel Pseudo-instructions
      Local Stack Slot Allocation
      X86 Optimize Call Frame
      Eliminate PHI nodes for register allocation
      Two-Address instruction pass
      Fast Register Allocator
      Bundle Machine CFG Edges
      X86 FP Stackifier
      Prologue/Epilogue Insertion & Frame Finalization
      Post-RA pseudo instruction expansion pass
      X86 pseudo instruction expansion pass
      Analyze Machine Code For Garbage Collection
      X86 vzeroupper inserter
      StackMap Liveness Analysis
      X86 Assembly / Object Emitter
  time: 4.017; rss: 326MB   codegen passes
Pass Arguments:  -tti -targetlibinfo -targetpassconfig -no-aa -tbaa -scoped-noalias -assumption-cache-tracker -basicaa -collector-metadata -machinemoduleinfo -atomic-expand -verify -gc-lowering -shadow-stack-gc-lowering -unreachableblockelim -rewrite-symbols -domtree -dwarfehprepare -safe-stack -stack-protector -verify -expand-isel-pseudos -localstackalloc -phi-node-elimination -twoaddressinstruction -edge-bundles -prologepilog -postrapseudos -gc-analysis -stackmap-liveness
Target Transform Information
Target Library Information
Target Pass Configuration
No Alias Analysis (always returns 'may' alias)
Type-Based Alias Analysis
Scoped NoAlias Alias Analysis
Assumption Cache Tracker
Basic Alias Analysis (stateless AA impl)
Create Garbage Collector Module Metadata
Machine Module Information
  ModulePass Manager
    FunctionPass Manager
      Expand Atomic calls in terms of either load-linked & store-conditional or cmpxchg
      Module Verifier
      Lower Garbage Collection Instructions
      Shadow Stack GC Lowering
      Remove unreachable blocks from the CFG
    Rewrite Symbols
    FunctionPass Manager
      Dominator Tree Construction
      Exception handling preparation
      Safe Stack instrumentation pass
      Insert stack protectors
      Module Verifier
      Machine Function Analysis
      X86 DAG->DAG Instruction Selection
      X86 PIC Global Base Reg Initialization
      Expand ISel Pseudo-instructions
      Local Stack Slot Allocation
      X86 Optimize Call Frame
      Eliminate PHI nodes for register allocation
      Two-Address instruction pass
      Fast Register Allocator
      Bundle Machine CFG Edges
      X86 FP Stackifier
      Prologue/Epilogue Insertion & Frame Finalization
      Post-RA pseudo instruction expansion pass
      X86 pseudo instruction expansion pass
      Analyze Machine Code For Garbage Collection
      X86 vzeroupper inserter
      StackMap Liveness Analysis
      X86 Assembly / Object Emitter
  time: 0.008; rss: 326MB   codegen passes
===-------------------------------------------------------------------------===
                      Instruction Selection and Scheduling
===-------------------------------------------------------------------------===
  Total Execution Time: 0.7300 seconds (0.6431 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.1567 ( 22.9%)   0.0200 ( 42.9%)   0.1767 ( 24.2%)   0.1488 ( 23.1%)  Instruction Selection
   0.1200 ( 17.6%)   0.0067 ( 14.3%)   0.1267 ( 17.4%)   0.1115 ( 17.3%)  DAG Combining 1
   0.1767 ( 25.9%)   0.0033 (  7.1%)   0.1800 ( 24.7%)   0.1046 ( 16.3%)  Instruction Scheduling
   0.0433 (  6.3%)   0.0000 (  0.0%)   0.0433 (  5.9%)   0.0674 ( 10.5%)  DAG Combining 2
   0.0533 (  7.8%)   0.0067 ( 14.3%)   0.0600 (  8.2%)   0.0582 (  9.0%)  Instruction Creation
   0.0400 (  5.9%)   0.0033 (  7.1%)   0.0433 (  5.9%)   0.0516 (  8.0%)  DAG Legalization
...
real    13m47.960s
user    14m38.980s
sys 0m4.890s

