 C
#include "stdio.h"
const char *hello_rust(void);
int main(void) {
    printf("%.32s\n", hello_rust());
}
