 C
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned char byte;

// 16 bytes
struct RawData {
    byte data[16];
};

// 16 bytes
struct StructuredData {
    short data;
    long _align;
};

struct StructuredData addr_to_sockaddr() {
    struct StructuredData storage = { 0 };

    // Try to set the fields by directly setting memory
    struct RawData *raw = (struct RawData *) &storage;
    memset(raw, 99, 16);

    // Then return it as a struct which is largely padding
    return storage;
}

int main(void) {
    // Get it back
    struct StructuredData addr = addr_to_sockaddr();

    // Convert to raw data
    struct RawData *addrp = (struct RawData *) &addr;

    int i;
    for(i = 0; i < 16; ++i) {
      // The padding is now zeroes!
      printf("%d, ", addrp->data[i]);
    }
    puts("");
}
