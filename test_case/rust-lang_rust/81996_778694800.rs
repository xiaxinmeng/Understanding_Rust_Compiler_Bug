
#include <stdio.h>

struct EmptyStruct {
};

int EmptyArray[0];

int main() {
    printf("size of empty struct = %d\n", (int)sizeof(struct EmptyStruct));
    printf("size of empty array = %d\n", (int)sizeof(EmptyArray));
}
