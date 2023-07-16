c++
void InstrProfiling::lowerIncrement(InstrProfIncrementInst *Inc) {
  auto *Addr = getCounterAddress(Inc);

  IRBuilder<> Builder(Inc);
  if (Options.Atomic || AtomicCounterUpdateAll ||
      (Inc->getIndex()->isZeroValue() && AtomicFirstCounter)) {
    Builder.CreateAtomicRMW(AtomicRMWInst::Add, Addr, Inc->getStep(),
                            MaybeAlign(), AtomicOrdering::Monotonic);
  } else {
    Value *IncStep = Inc->getStep();
    Value *Load = Builder.CreateLoad(IncStep->getType(), Addr, "pgocount");
    auto *Count = Builder.CreateAdd(Load, Inc->getStep());
    auto *Store = Builder.CreateStore(Count, Addr);
    if (isCounterPromotionEnabled())
      PromotionCandidates.emplace_back(cast<Instruction>(Load), Store);
  }
  Inc->eraseFromParent();
}
