
#include<stdlib.h>
void wowzers(int y) {
    void *z = malloc(y);
    if(z != NULL) free(z);
}
