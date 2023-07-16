c
#include <string.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
    char *array[] = {
#include "num_array.h"
        argv[0],
    };
    size_t sum = 0;
    for (int i = 0; i < sizeof(array)/sizeof(*array); i++) {
        sum += strlen(array[i]);
    }
    printf("%zu\n", sum);
    return 0;
}
