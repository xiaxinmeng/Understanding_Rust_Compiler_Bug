 c
#include <stdio.h>

int twice(int (*f)(int), int x) {
    int ret, arg;
    printf("-- C twice calling f(%d)\n", x);
    ret = f(f(x));
    printf("-- C twice done w/ f(%d) => %d\n", x, ret);
    return ret;
}

int incr3(int x) {
    int ret;
    printf("------ C Enter incr3(f, %d)\n", x);
    ret = x + 3;
    printf("------ C Finis incr3(f, %d) => %d\n", x, ret);
    return ret;
}

int callback(int (*rtwice)(int (*f)(int), int x), int x) {
    int ret;
    printf("-- C callback calling rtwice(incr3, %d)\n", x);
    ret = rtwice(incr3, x);
    printf("-- C callback done w/ rtwice(incr3, %d) => %d\n", x, ret);
    return ret;
}
