c
#define _GNU_SOURCE
#include <unistd.h>
#include <limits.h>
#include <stdio.h>

size_t max = ((size_t)INT_MAX);

int main() {
  FILE *fa = fopen("a.txt", "rb");
  FILE *fb = fopen("b.txt", "wb");
  printf("calling copy_file_range with len %p\n", max);
  if (copy_file_range(fileno(fa), NULL, fileno(fb), NULL, max, 0) != 0) {
          perror(NULL);
  }
}
