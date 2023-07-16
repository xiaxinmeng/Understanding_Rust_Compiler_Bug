cpp
  // If the function exists but has the wrong type, return a bitcast to the
  // right type.
  auto *PTy = PointerType::get(Ty, F->getAddressSpace());
  if (F->getType() != PTy)
    return {Ty, ConstantExpr::getBitCast(F, PTy)};
