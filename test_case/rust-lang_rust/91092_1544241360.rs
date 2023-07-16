
void InstrProfiling::lowerIncrement(InstrProfIncrementInst *Inc) {
  auto *Addr = getCounterAddress(Inc);

  IRBuilder<> Builder(Inc);
  if (Options.Atomic || AtomicCounterUpdateAll ||
