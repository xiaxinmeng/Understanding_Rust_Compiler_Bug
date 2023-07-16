
#include <stdio.h>

class Inner {
  public:
  ~Inner() { printf("Inner\n"); }
};

class Outer {
  public:
  Inner c;
  ~Outer() { printf("Outer\n"); }
};

int main() {
  Outer outer;
  return 0;
}
