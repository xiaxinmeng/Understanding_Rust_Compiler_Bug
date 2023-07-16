cpp
constexpr int test() {
  int n = 113383;
  while (n != 1) {
    if (n % 2 == 0) {
      n /= 2;
    } else {
      n = n * 3 + 1;
    }
  }
  return n;
}

int test_1() {
  return test();
}
