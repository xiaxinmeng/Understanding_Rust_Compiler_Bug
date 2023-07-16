cpp
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>

#include <vector>

void foo1(const uint32_t *y, size_t y_len) {
  const uint32_t *y_end = y + y_len;
  size_t c = 0;
  for (const uint32_t *y_iter = y; y_iter != y_end; y_iter++, c++) {
    assert(c < y_len);
  }
  assert(c == y_len);
}

void foo2(const std::vector<uint32_t>& y) {
    size_t c = 0;
    for (auto y_iter: y) {
        assert(c < y.size());
        c++;
    }
    assert(c == y.size());
}

void foo3(const std::vector<uint32_t>& y) {
    size_t c = 0;
    for (auto y_iter = y.cbegin(); y_iter != y.cend(); y_iter++, c++) {
        assert(c < y.size());
    }
    assert(c == y.size());
}
