
// Don't need to zero the memory, but do so anyway
// to appease Valgrind.
if (valgrind_) {
  for (int i = 0; i < max_size; i++) {
    dense_[i] = 0xababababU;
    sparse_to_dense_[i] = 0xababababU;
  }
}
