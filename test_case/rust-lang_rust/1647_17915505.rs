 c
#include <stdio.h>

enum foo { A, B };
enum bar { C = 256, D, E };
enum baz { F = 8796093022208, G };

int main() {
  printf("%zu\n", sizeof(enum foo));
  printf("%zu\n", sizeof(enum bar));
  printf("%zu\n", sizeof(enum baz));
  return 0;
}
