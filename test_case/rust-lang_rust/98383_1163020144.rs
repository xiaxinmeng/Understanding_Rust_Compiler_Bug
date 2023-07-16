plain
   Compiling object v0.26.2
   Compiling hashbrown v0.12.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
cmpxchg instructions failure argument shall be no stronger than the success argument
  %90 = cmpxchg i64* %0, i64 %1, i64 %2 acq_rel seq_cst
cmpxchg instructions failure argument shall be no stronger than the success argument
  %114 = cmpxchg i64* %0, i64 %1, i64 %2 acquire seq_cst
cmpxchg instructions failure argument shall be no stronger than the success argument
  %138 = cmpxchg i64* %0, i64 %1, i64 %2 release seq_cst
cmpxchg instructions failure argument shall be no stronger than the success argument
  %155 = cmpxchg i64* %0, i64 %1, i64 %2 monotonic acquire
cmpxchg instructions failure argument shall be no stronger than the success argument
  %162 = cmpxchg i64* %0, i64 %1, i64 %2 monotonic seq_cst
in function _ZN4core4sync6atomic23atomic_compare_exchange17h12c0bd871e66817aE
LLVM ERROR: Broken function found, compilation aborted!
Build completed unsuccessfully in 0:05:01
