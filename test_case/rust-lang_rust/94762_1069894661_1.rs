c++
unsigned LoopVectorizationCostModel::selectInterleaveCount(ElementCount VF,
                                                           unsigned LoopCost) {
//[...]

  // If we did not calculate the cost for VF (because the user selected the VF)
  // then we calculate the cost of VF here.
  if (LoopCost == 0) {
    InstructionCost C = expectedCost(VF).first;
    assert(C.isValid() && "Expected to have chosen a VF with valid cost");
    LoopCost = *C.getValue();
  }

  assert(LoopCost && "Non-zero loop cost expected");

//[...]

  if (!InterleavingRequiresRuntimePointerCheck && LoopCost < SmallLoopCost) {
    // We assume that the cost overhead is 1 and we use the cost model
    // to estimate the cost of the loop and interleave until the cost of the
    // loop overhead is about 5% of the cost of the loop.
    unsigned SmallIC =
        std::min(IC, (unsigned)PowerOf2Floor(SmallLoopCost / LoopCost));
