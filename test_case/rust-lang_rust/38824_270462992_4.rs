
$ rustc --target nvptx64-nvidia-cuda foo.rs
rustc: /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:7952: void llvm::SelectionDAGISel::LowerArguments(const llvm::Function&): Assertion `InVals.size() == Ins.size() && "LowerFormalArguments didn't emit the correct number of values!"' failed.
