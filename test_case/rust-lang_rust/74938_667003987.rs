c
#include <stddef.h>

const size_t N = 3;

int foo(size_t len) {
	size_t newlen = (len / N) * N;
	return newlen <= len;
}
