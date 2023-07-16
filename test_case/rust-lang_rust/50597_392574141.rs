c
#include <stdio.h>
#include <stdbool.h>

int main(void) {
	bool a = 256; // the conversion occurring here stores the value 1 in memory.
	printf("%d\n", a);
	return 0;
}
// actually prints `1`.
