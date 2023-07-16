 c
#include <stdio.h>

int
main(void)
{
        for(size_t i = 0; i < 100; ++i){
                printf("Hello\n");
                fflush(stdout);
        }
        return 0;
}
