c
#include <stdio.h>

struct foo {
  long double bar;
};

int main() {
  printf("%ld\n", _Alignof(struct foo)); // 16
}
