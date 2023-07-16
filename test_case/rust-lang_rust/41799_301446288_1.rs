llvm
  %1 = icmp ugt i128 %0, -10141204801825835211973625643009
  %2 = uitofp i128 %0 to float
  %_0.0 = select i1 %1, float 0x7FF0000000000000, float %2
