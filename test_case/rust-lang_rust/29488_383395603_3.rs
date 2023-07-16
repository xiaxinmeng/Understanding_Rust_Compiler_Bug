
#include <stdio.h>

extern "C" {
	int lib_fun(int t) {
		printf("from C++: %d\r\n", t);
		fflush(stdout);
		return t;
	}
}

