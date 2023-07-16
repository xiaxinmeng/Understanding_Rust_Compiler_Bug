 C
#include <stdint.h>
#include <stdio.h>
#include <inttypes.h>

int main(void) {
  int32_t x = -15;
  printf("%"PRIX32"\n", (uint32_t)x);
}
