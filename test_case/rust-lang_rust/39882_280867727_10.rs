 c
#include <stdio.h>

typedef struct {} Nil;

int main() {
  Nil nil = {};
  int x = *((int *)&nil);

  printf("%p\n", (void *)&nil);
  printf("%p\n", (void *)&x);
  printf("%d\n", x);
}
