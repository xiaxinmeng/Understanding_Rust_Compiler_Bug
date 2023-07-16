cc
  // If the CallSite is to malloc or calloc, we can assume that it doesn't
  // modify any IR visible value.  This is only valid because we assume these
  // routines do not read values visible in the IR.  TODO: Consider special
  // casing realloc and strdup routines which access only their arguments as
  // well.  Or alternatively, replace all of this with inaccessiblememonly once
  // that's implemented fully.
  auto *Inst = CS.getInstruction();
  if (isMallocOrCallocLikeFn(Inst, &TLI)) {
    // Be conservative if the accessed pointer may alias the allocation -
    // fallback to the generic handling below.
    if (getBestAAResults().alias(MemoryLocation(Inst), Loc) == NoAlias)
      return ModRefInfo::NoModRef;
  }
