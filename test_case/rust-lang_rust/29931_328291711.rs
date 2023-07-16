c++
  size_type computePushBackCapacity() const {
    if (capacity() == 0) {
      return std::max(64 / sizeof(T), size_type(1));
    }
    if (capacity() < folly::jemallocMinInPlaceExpandable / sizeof(T)) {
      return capacity() * 2;
    }
    if (capacity() > 4096 * 32 / sizeof(T)) {
      return capacity() * 2;
    }
    return (capacity() * 3 + 1) / 2;
  }
