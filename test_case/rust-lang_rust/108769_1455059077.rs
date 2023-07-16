c++
#include <cmath>
#include <cstdio>

int main() {
  printf("%f\n", std::atan(std::tan(float(M_PI / 2))));
  printf("%f\n", std::atan(std::tan(double(M_PI / 2))));
}
