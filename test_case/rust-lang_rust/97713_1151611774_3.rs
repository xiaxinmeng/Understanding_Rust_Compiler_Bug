
  std::tie(CtorFunc, std::ignore) = createSanitizerCtorAndInitFunctions(                                                                                                                                          
      M, CtorName, InitFunctionName, {PtrTy, PtrTy}, {SecStart, SecEnd});                                                                                                                                         
  assert(CtorFunc->getName() == CtorName);                                                                                                                                                                        
