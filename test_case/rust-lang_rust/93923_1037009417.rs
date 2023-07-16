cpp
template <int W>
void writePatchableLEB(raw_pwrite_stream &Stream, uint64_t X, uint64_t Offset) {
  uint8_t Buffer[W];
  unsigned SizeLen = encodeULEB128(X, Buffer, W);
  assert(SizeLen == W);
  Stream.pwrite((char *)Buffer, SizeLen, Offset);
}
