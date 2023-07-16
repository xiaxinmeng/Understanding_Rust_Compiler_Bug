
#include "stdio.h"

void copy(int * restrict to, int * restrict from) {
	*to = *from;
}

void test(int *a, int *b) {
	for (int i = 0; i < 4; i++) {
		copy(&b[i & 1], &a[i & 1]);
	}
}

int main() {
	int ary[] = {0, 1, 2};
	test(&ary[1], &ary[0]);
	printf("%d %d %d\n", ary[0], ary[1], ary[2]);
	return 1;
}
