c++
  unsigned getNumBuckets() const {
    return Small ? InlineBuckets : getLargeRep()->NumBuckets;
  }
