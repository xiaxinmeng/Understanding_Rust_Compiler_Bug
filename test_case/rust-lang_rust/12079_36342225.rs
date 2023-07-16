
Assertion failed: (getOperand(0)->getType() == 
cast<PointerType>(getOperand(1)->getType())->getElementType() && 
"Ptr must be a pointer to Val type!"), function AssertOK, file 
/Users/daniel/Projects/rust/src/llvm/lib/IR/Instructions.cpp, line 1085.
