
$ opt -S -debug-only=loop-unroll,aarch64tti -passes=loop-unroll --disable-output fibonacci_vec-before-LoopUnrollPass.ll 
Loop Unroll: F[_ZN13fibonacci_vec13fibonacci_vec17hb6b1caf079e597beE] Loop %bb7
getProcFamily'
 isOutOfOrder0
  Loop Size = 7
  will not try to unroll loop with runtime trip count -unroll-runtime not given
