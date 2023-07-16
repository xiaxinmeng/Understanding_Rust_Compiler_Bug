
llc: /home/den/rust/src/llvm/lib/Target/NVPTX/NVPTXISelLowering.cpp:1729: virtual llvm::SDValue llvm::NVPTXTargetLowering::LowerCall(llvm::TargetLowering::CallLoweringInfo&, llvm::SmallVectorImpl<llvm::SDValue>&) const: Assertion `VTs.size() == Ins.size() && "Bad value decomposition"' failed.
Stack dump:
0.      Program arguments: /home/den/rust/build/x86_64-unknown-linux-gnu/llvm/build/bin/llc core.ll
1.      Running pass 'Function Pass Manager' on module 'core.ll'.
2.      Running pass 'NVPTX DAG->DAG Pattern Instruction Selection' on function '@_ZN4core3num14from_str_radix17hb496b766c386ac7aE'
