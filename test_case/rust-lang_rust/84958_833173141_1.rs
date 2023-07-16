cpp
    // Check that key is an instruction, to skip the Argument mapping, which
    // points to an instruction in the original function, not the inlined one.
    if (!VMI->second || !isa<Instruction>(VMI->first))
      continue;
