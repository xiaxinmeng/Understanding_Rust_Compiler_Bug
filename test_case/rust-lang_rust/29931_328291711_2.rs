c++
template <typename T, typename Allocator>
template <class... Args>
void fbvector<T, Allocator>::emplace_back_aux(Args&&... args) {
  size_type byte_sz = folly::goodMallocSize(
    computePushBackCapacity() * sizeof(T));
   // ...
}
