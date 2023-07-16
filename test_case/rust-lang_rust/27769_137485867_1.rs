 c
#include <stdio.h>

extern char *reverse(const char *);
extern void cleanup(char *);

int main() {
  char *s = reverse("Hello, world!");
  printf("%s\n", s);
  cleanup(s);
}
