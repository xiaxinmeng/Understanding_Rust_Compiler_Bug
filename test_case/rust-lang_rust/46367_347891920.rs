
rustc: /home/alex/code/rust2/src/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:8133: std::pair<llvm::SDValue, llvm::SDValue> llvm::TargetLowering::LowerCallTo(llvm::TargetLowering::CallLoweringInfo&) const: Assertion `CLI.RetTy == Args[i].Ty && RetTys.size() == NumValues && "unexpected use of 'returned'"' failed.
zsh: abort (core dumped)  ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc foo.rs --target   cdylib
