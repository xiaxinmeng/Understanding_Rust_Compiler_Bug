c
#include <stdio.h>

int main() {
    char a[10];

    printf("%ld\n", &a[5] - a); // 5
}
