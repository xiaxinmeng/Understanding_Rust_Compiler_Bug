c++
  size_type computeInsertCapacity(size_type n) {
    size_type nc = std::max(computePushBackCapacity(), size() + n);
    size_type ac = folly::goodMallocSize(nc * sizeof(T)) / sizeof(T);
    return ac;
  }
