cpp
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>

uint32_t foo1(const uint32_t *x, size_t x_len, const uint32_t *y, size_t y_len) {
  uint32_t sum = 0;
  size_t chunk_size = y_len;

  const uint32_t *y_end = y + y_len;
  size_t c = 0;
  for (const uint32_t *y_iter = y; y_iter != y_end; y_iter++, c++) {
    size_t chunks_len = x_len - (x_len % chunk_size);

    for (const uint32_t *chunks_iter = x; chunks_len >= chunk_size; chunks_iter += chunks_len, chunks_len -= chunk_size) {
      uint32_t val;

      assert(c < chunk_size);
      val = chunks_iter[c];

      sum += val + *y_iter;
    }
  }

  return sum;
}

uint32_t foo2(const uint32_t *x, size_t x_len, const uint32_t *y, size_t y_len) {
  uint32_t sum = 0;
  size_t chunk_size = y_len;

  for (size_t c = 0; c < chunk_size; c++) {
    uint32_t y_iter;
    size_t chunks_len;

    assert(c < y_len);
    y_iter = y[c];
    
    chunks_len  = x_len - (x_len % chunk_size);
    for (const uint32_t *chunks_iter = x; chunks_len >= chunk_size; chunks_iter += chunks_len, chunks_len -= chunk_size) {
      uint32_t val;

      assert(c < chunk_size);
      val = chunks_iter[c];

      sum += val + y_iter;
    }
  }

  return sum;
}
