c++
#include <iostream>
#include <cassert>
#include <cmath>

int main() {
	assert(std::copysign(1.0, -INFINITY * 0.0) == -1.0);
	return 0;
}
