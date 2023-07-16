 c
#include <stdio.h>
#include <string.h>

int main(void) {
    printf("%zu\n", strlen("ä"));
    printf("%zu\n", strlen("\u00FF"));
    printf("%zu\n", strlen("\xFF"));
}
