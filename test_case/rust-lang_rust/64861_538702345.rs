
#include <stdio.h>
#include <unistd.h>

int main() {
	printf("a\n");
	char *a[] = {"/bin/ls", NULL};
	execv(a[0], a);
	return 0;
}
