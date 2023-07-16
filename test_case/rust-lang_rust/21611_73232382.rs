 cpp
#include<cstdio>

struct foo { char x; };
int main() {
    printf("%lu\n", alignof(struct foo));
}
