 cpp
  if (const auto *Class = dyn_cast_or_null<DICompositeType>(Scope)) {
    // If the scope is a DICompositeType, then this must be a method. Member
    // function types take some special handling, and require access to the
    // subprogram.
