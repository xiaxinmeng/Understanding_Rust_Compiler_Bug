c++
Constant *ConstantExpr::getTrunc(Constant *C, Type *Ty, bool OnlyIfReduced) {
  ...
  assert(C->getType()->getScalarSizeInBits() > Ty->getScalarSizeInBits()&&
"SrcTy must be larger than DestTy for Trunc!");
