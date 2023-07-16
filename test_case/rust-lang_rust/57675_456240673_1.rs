cpp
TypeIndex CodeViewDebug::lowerTypeMemberFunction(const DISubroutineType *Ty,
                                                 const DIType *ClassTy,
                                                 int ThisAdjustment,
                                                 bool IsStaticMethod,
                                                 FunctionOptions FO) {
  // Lower the containing class type.
  TypeIndex ClassType = getTypeIndex(ClassTy);

  DITypeRefArray ReturnAndArgs = Ty->getTypeArray();

  unsigned Index = 0;
  SmallVector<TypeIndex, 8> ArgTypeIndices;
  TypeIndex ReturnTypeIndex = getTypeIndex(ReturnAndArgs[Index++]);
