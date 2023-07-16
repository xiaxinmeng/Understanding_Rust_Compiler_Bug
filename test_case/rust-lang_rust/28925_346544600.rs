C
#include <stdio.h>

enum E {
  L = 0,
  H = 0x100000000
};

int main() {
	printf("%zu > %zu\n", sizeof(enum E), sizeof(void*));
	return 0;
}
