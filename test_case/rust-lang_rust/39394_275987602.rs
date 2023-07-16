
#include <stdio.h>
struct foo {
        int c;
};
struct foo fcdr(struct foo);
int main(void) {
        printf("call fcdr with 0\n");
        struct foo x = { 0 };
        fcdr(x);
        return 0;
}
