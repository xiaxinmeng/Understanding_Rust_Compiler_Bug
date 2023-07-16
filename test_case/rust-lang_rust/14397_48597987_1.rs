
% ./x86_64-apple-darwin/stage2/bin/rustc --version
rustc 0.11.0-pre (2973ca6 2014-07-07 13:49:43 +1200)
host: x86_64-apple-darwin
% ./x86_64-apple-darwin/stage2/bin/rustc /tmp/g.rs && ./g
Assertion failed: (getOperand(0)->getType() == cast<PointerType>(getOperand(1)->getType())->getElementType() && "Ptr must be a pointer to Val type!"), function AssertOK, file ../../../../../src/llvm/lib/IR/Instructions.cpp, line 1085.
Abort trap: 6
% 
