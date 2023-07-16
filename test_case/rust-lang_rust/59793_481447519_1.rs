cc
void Lint::visitUnreachableInst(UnreachableInst &I) {
  // This isn't undefined behavior, it's merely suspicious.
  Assert(&I == &I.getParent()->front() ||
             std::prev(I.getIterator())->mayHaveSideEffects(),
         "Unusual: unreachable immediately preceded by instruction without "
         "side effects",
         &I);
}
