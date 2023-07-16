c
#define _GNU_SOURCE
#include <unistd.h>
#include <limits.h>
#include <stdio.h>

size_t max = INT_MAX + 1;

int main() {
  FILE *fa = fopen("a.txt", "rb");
  FILE *fb = fopen("b.txt", "wb");
  printf("calling copy_file_range with len %d\n", max);
  if (copy_file_range(fileno(fa), NULL, fileno(fb), NULL, max, 0) != 0) {
          perror(NULL);
  }
}
