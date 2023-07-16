 cpp
DIE *DwarfUnit::getOrCreateTypeDIE(const MDNode *TyNode) {
  if (!TyNode)
    return NULL;

  DIType Ty(TyNode);
  assert(Ty.isType());
  assert(Ty == resolve(Ty.getRef()) &&
         "type was not uniqued, possible ODR violation.");
