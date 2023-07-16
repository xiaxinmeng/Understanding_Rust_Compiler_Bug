 c++
#include <limits>
#include <iostream>
#include <string>

int main()
{
  std::cout << std::stof(std::to_string(std::numeric_limits<float>::max())) << std::endl;
}
