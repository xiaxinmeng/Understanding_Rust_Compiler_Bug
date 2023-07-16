cpp
struct processed_utf_bytes {
  __m128i rawbytes;
  __m128i high_nibbles;
  __m128i carried_continuations;
};
