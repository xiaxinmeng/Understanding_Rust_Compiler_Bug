
$ clang test.c -fprofile-instr-generate -fcoverage-mapping -S -o ctest.ll -emit-llvm -O3 -mllvm -debug-pass=Structure
Pass Arguments:  -tti -targetlibinfo -tbaa -scoped-noalias -assumption-cache-tracker -ee-instrument -simplifycfg -domtree -sroa -early-cse -lower-expect
Target Transform Information
Target Library Information
Type-Based Alias Analysis
Scoped NoAlias Alias Analysis
Assumption Cache Tracker
  FunctionPass Manager
    Instrument function entry/exit with calls to e.g. mcount() (pre inlining)
    Simplify the CFG
    Dominator Tree Construction
    SROA
    Early CSE
    Lower 'expect' Intrinsics
Pass Arguments:  -tti -targetlibinfo -tbaa -scoped-noalias -assumption-cache-tracker -profile-summary-info -instrprof -forceattrs -inferattrs -domtree -callsite-splitting -ipsccp -called-value-propagation -globalopt -domtree -mem2reg -deadargelim -domtree -basic-aa -aa -loops -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -instcombine -simplifycfg -basiccg -globals-aa -prune-eh -inline -openmpopt -functionattrs -argpromotion -domtree -sroa -basic-aa -aa -memoryssa -early-cse-memssa -aa -lazy-value-info -jump-threading -correlated-propagation -simplifycfg -domtree -aggressive-instcombine -basic-aa -aa -loops -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -instcombine -libcalls-shrinkwrap -loops -postdomtree -branch-prob -block-freq -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -pgo-memop-opt -basic-aa -aa -loops -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -tailcallelim -simplifycfg -reassociate -domtree -loops -loop-simplify -lcssa-verification -lcssa -basic-aa -aa -scalar-evolution -loop-rotate -memoryssa -licm -loop-unswitch -simplifycfg -domtree -basic-aa -aa -loops -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -instcombine -loop-simplify -lcssa-verification -lcssa -scalar-evolution -indvars -loop-idiom -loop-deletion -loop-unroll -mldst-motion -phi-values -aa -memdep -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -gvn -phi-values -basic-aa -aa -memdep -memcpyopt -sccp -demanded-bits -bdce -aa -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -instcombine -lazy-value-info -jump-threading -correlated-propagation -basic-aa -aa -phi-values -memdep -dse -aa -memoryssa -loops -loop-simplify -lcssa-verification -lcssa -scalar-evolution -licm -postdomtree -adce -simplifycfg -domtree -basic-aa -aa -loops -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -instcombine -barrier -elim-avail-extern -basiccg -rpo-functionattrs -globalopt -globaldce -basiccg -globals-aa -domtree -float2int -lower-constant-intrinsics -domtree -loops -loop-simplify -lcssa-verification -lcssa -basic-aa -aa -scalar-evolution -loop-rotate -loop-accesses -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -loop-distribute -postdomtree -branch-prob -block-freq -scalar-evolution -basic-aa -aa -loop-accesses -demanded-bits -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -inject-tli-mappings -loop-vectorize -loop-simplify -scalar-evolution -aa -loop-accesses -lazy-branch-prob -lazy-block-freq -loop-load-elim -basic-aa -aa -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -instcombine -simplifycfg -domtree -loops -scalar-evolution -basic-aa -aa -demanded-bits -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -inject-tli-mappings -slp-vectorizer -vector-combine -opt-remark-emitter -instcombine -loop-simplify -lcssa-verification -lcssa -scalar-evolution -loop-unroll -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -instcombine -memoryssa -loop-simplify -lcssa-verification -lcssa -scalar-evolution -licm -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -transform-warning -alignment-from-assumptions -strip-dead-prototypes -globaldce -constmerge -cg-profile -domtree -loops -postdomtree -branch-prob -block-freq -loop-simplify -lcssa-verification -lcssa -basic-aa -aa -scalar-evolution -block-freq -loop-sink -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -instsimplify -div-rem-pairs -simplifycfg
Target Transform Information
Target Library Information
Type-Based Alias Analysis
Scoped NoAlias Alias Analysis
Assumption Cache Tracker
Profile summary info
  ModulePass Manager
    Frontend instrumentation-based coverage lowering
    Force set function attributes
    Infer set function attributes
    FunctionPass Manager
      Dominator Tree Construction
      Call-site splitting
    Interprocedural Sparse Conditional Constant Propagation
      FunctionPass Manager
        Dominator Tree Construction
    Called Value Propagation
    Global Variable Optimizer
      FunctionPass Manager
        Dominator Tree Construction
        Natural Loop Information
        Post-Dominator Tree Construction
        Branch Probability Analysis
        Block Frequency Analysis
    FunctionPass Manager
      Dominator Tree Construction
      Promote Memory to Register
    Dead Argument Elimination
    FunctionPass Manager
      Dominator Tree Construction
      Basic Alias Analysis (stateless AA impl)
      Function Alias Analysis Results
      Natural Loop Information
      Lazy Branch Probability Analysis
      Lazy Block Frequency Analysis
      Optimization Remark Emitter
      Combine redundant instructions
      Simplify the CFG
    CallGraph Construction
    Globals Alias Analysis
    Call Graph SCC Pass Manager
      Remove unused exception handling info
      Function Integration/Inlining
      OpenMP specific optimizations
      Deduce function attributes
      Promote 'by reference' arguments to scalars
      FunctionPass Manager
        Dominator Tree Construction
        SROA
        Basic Alias Analysis (stateless AA impl)
        Function Alias Analysis Results
        Memory SSA
        Early CSE w/ MemorySSA
        Speculatively execute instructions if target has divergent branches
        Function Alias Analysis Results
        Lazy Value Information Analysis
        Jump Threading
        Value Propagation
        Simplify the CFG
        Dominator Tree Construction
        Combine pattern based expressions
        Basic Alias Analysis (stateless AA impl)
        Function Alias Analysis Results
        Natural Loop Information
        Lazy Branch Probability Analysis
        Lazy Block Frequency Analysis
        Optimization Remark Emitter
        Combine redundant instructions
        Conditionally eliminate dead library calls
        Natural Loop Information
        Post-Dominator Tree Construction
        Branch Probability Analysis
        Block Frequency Analysis
        Lazy Branch Probability Analysis
        Lazy Block Frequency Analysis
        Optimization Remark Emitter
        PGOMemOPSize
        Basic Alias Analysis (stateless AA impl)
        Function Alias Analysis Results
        Natural Loop Information
        Lazy Branch Probability Analysis
        Lazy Block Frequency Analysis
        Optimization Remark Emitter
        Tail Call Elimination
        Simplify the CFG
        Reassociate expressions
        Dominator Tree Construction
        Natural Loop Information
        Canonicalize natural loops
        LCSSA Verifier
        Loop-Closed SSA Form Pass
        Basic Alias Analysis (stateless AA impl)
        Function Alias Analysis Results
        Scalar Evolution Analysis
        Loop Pass Manager
          Rotate Loops
        Memory SSA
        Loop Pass Manager
          Loop Invariant Code Motion
          Unswitch loops
        Simplify the CFG
        Dominator Tree Construction
        Basic Alias Analysis (stateless AA impl)
        Function Alias Analysis Results
        Natural Loop Information
        Lazy Branch Probability Analysis
        Lazy Block Frequency Analysis
        Optimization Remark Emitter
        Combine redundant instructions
        Canonicalize natural loops
        LCSSA Verifier
        Loop-Closed SSA Form Pass
        Scalar Evolution Analysis
        Loop Pass Manager
          Induction Variable Simplification
          Recognize loop idioms
          Delete dead loops
          Unroll loops
        MergedLoadStoreMotion
        Phi Values Analysis
        Function Alias Analysis Results
        Memory Dependence Analysis
        Lazy Branch Probability Analysis
        Lazy Block Frequency Analysis
        Optimization Remark Emitter
        Global Value Numbering
        Phi Values Analysis
        Basic Alias Analysis (stateless AA impl)
        Function Alias Analysis Results
        Memory Dependence Analysis
        MemCpy Optimization
        Sparse Conditional Constant Propagation
        Demanded bits analysis
        Bit-Tracking Dead Code Elimination
        Function Alias Analysis Results
        Lazy Branch Probability Analysis
        Lazy Block Frequency Analysis
        Optimization Remark Emitter
        Combine redundant instructions
        Lazy Value Information Analysis
        Jump Threading
        Value Propagation
        Basic Alias Analysis (stateless AA impl)
        Function Alias Analysis Results
        Phi Values Analysis
        Memory Dependence Analysis
        Dead Store Elimination
        Function Alias Analysis Results
        Memory SSA
        Natural Loop Information
        Canonicalize natural loops
        LCSSA Verifier
        Loop-Closed SSA Form Pass
        Scalar Evolution Analysis
        Loop Pass Manager
          Loop Invariant Code Motion
        Post-Dominator Tree Construction
        Aggressive Dead Code Elimination
        Simplify the CFG
        Dominator Tree Construction
        Basic Alias Analysis (stateless AA impl)
        Function Alias Analysis Results
        Natural Loop Information
        Lazy Branch Probability Analysis
        Lazy Block Frequency Analysis
        Optimization Remark Emitter
        Combine redundant instructions
    A No-Op Barrier Pass
    Eliminate Available Externally Globals
    CallGraph Construction
    Deduce function attributes in RPO
    Global Variable Optimizer
      FunctionPass Manager
        Dominator Tree Construction
        Natural Loop Information
        Post-Dominator Tree Construction
        Branch Probability Analysis
        Block Frequency Analysis
    Dead Global Elimination
    CallGraph Construction
    Globals Alias Analysis
    FunctionPass Manager
      Dominator Tree Construction
      Float to int
      Lower constant intrinsics
      Dominator Tree Construction
      Natural Loop Information
      Canonicalize natural loops
      LCSSA Verifier
      Loop-Closed SSA Form Pass
      Basic Alias Analysis (stateless AA impl)
      Function Alias Analysis Results
      Scalar Evolution Analysis
      Loop Pass Manager
        Rotate Loops
      Loop Access Analysis
      Lazy Branch Probability Analysis
      Lazy Block Frequency Analysis
      Optimization Remark Emitter
      Loop Distribution
      Post-Dominator Tree Construction
      Branch Probability Analysis
      Block Frequency Analysis
      Scalar Evolution Analysis
      Basic Alias Analysis (stateless AA impl)
      Function Alias Analysis Results
      Loop Access Analysis
      Demanded bits analysis
      Lazy Branch Probability Analysis
      Lazy Block Frequency Analysis
      Optimization Remark Emitter
      Inject TLI Mappings
      Loop Vectorization
      Canonicalize natural loops
      Scalar Evolution Analysis
      Function Alias Analysis Results
      Loop Access Analysis
      Lazy Branch Probability Analysis
      Lazy Block Frequency Analysis
      Loop Load Elimination
      Basic Alias Analysis (stateless AA impl)
      Function Alias Analysis Results
      Lazy Branch Probability Analysis
      Lazy Block Frequency Analysis
      Optimization Remark Emitter
      Combine redundant instructions
      Simplify the CFG
      Dominator Tree Construction
      Natural Loop Information
      Scalar Evolution Analysis
      Basic Alias Analysis (stateless AA impl)
      Function Alias Analysis Results
      Demanded bits analysis
      Lazy Branch Probability Analysis
      Lazy Block Frequency Analysis
      Optimization Remark Emitter
      Inject TLI Mappings
      SLP Vectorizer
      Optimize scalar/vector ops
      Optimization Remark Emitter
      Combine redundant instructions
      Canonicalize natural loops
      LCSSA Verifier
      Loop-Closed SSA Form Pass
      Scalar Evolution Analysis
      Loop Pass Manager
        Unroll loops
      Lazy Branch Probability Analysis
      Lazy Block Frequency Analysis
      Optimization Remark Emitter
      Combine redundant instructions
      Memory SSA
      Canonicalize natural loops
      LCSSA Verifier
      Loop-Closed SSA Form Pass
      Scalar Evolution Analysis
      Loop Pass Manager
        Loop Invariant Code Motion
      Lazy Branch Probability Analysis
      Lazy Block Frequency Analysis
      Optimization Remark Emitter
      Warn about non-applied transformations
      Alignment from assumptions
    Strip Unused Function Prototypes
    Dead Global Elimination
    Merge Duplicate Global Constants
    Call Graph Profile
      FunctionPass Manager
        Dominator Tree Construction
        Natural Loop Information
        Lazy Branch Probability Analysis
        Lazy Block Frequency Analysis
    FunctionPass Manager
      Dominator Tree Construction
      Natural Loop Information
      Post-Dominator Tree Construction
      Branch Probability Analysis
      Block Frequency Analysis
      Canonicalize natural loops
      LCSSA Verifier
      Loop-Closed SSA Form Pass
      Basic Alias Analysis (stateless AA impl)
      Function Alias Analysis Results
      Scalar Evolution Analysis
      Block Frequency Analysis
      Loop Pass Manager
        Loop Sink
      Lazy Branch Probability Analysis
      Lazy Block Frequency Analysis
      Optimization Remark Emitter
      Remove redundant instructions
      Hoist/decompose integer division and remainder
      Simplify the CFG
    Print Module IR
Pass Arguments:  -domtree
  FunctionPass Manager
    Dominator Tree Construction
Pass Arguments:  -targetlibinfo -domtree -loops -postdomtree -branch-prob -block-freq
Target Library Information
  FunctionPass Manager
    Dominator Tree Construction
    Natural Loop Information
    Post-Dominator Tree Construction
    Branch Probability Analysis
    Block Frequency Analysis
Pass Arguments:  -targetlibinfo -domtree -loops -postdomtree -branch-prob -block-freq
Target Library Information
  FunctionPass Manager
    Dominator Tree Construction
    Natural Loop Information
    Post-Dominator Tree Construction
    Branch Probability Analysis
    Block Frequency Analysis
Pass Arguments:  -targetlibinfo -domtree -loops -lazy-branch-prob -lazy-block-freq
Target Library Information
  FunctionPass Manager
    Dominator Tree Construction
    Natural Loop Information
    Lazy Branch Probability Analysis
    Lazy Block Frequency Analysis
Pass Arguments:  -tti
Target Transform Information
  ModulePass Manager
