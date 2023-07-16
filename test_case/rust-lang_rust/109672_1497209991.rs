cpp
#include <cstdlib>
#include <iostream>

struct Abort {
  ~Abort() {
    abort();
  }
};

__attribute__((noinline))
static void abort_in_dtor() {
  Abort abort;
  throw "test";
}

int main() {
  try {
    abort_in_dtor();
  } catch (...) {
    std::cout << "caught" << std::endl;
  }
}
