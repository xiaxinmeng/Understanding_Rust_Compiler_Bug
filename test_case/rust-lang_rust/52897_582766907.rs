
$ cat example.c 
#include <math.h>
#include <stdio.h>

int main() {
    double a = 1;
    double b = NAN;
    printf("fmin(%f, %f) = %f\n", a, b, fmin(a, b));
    return 0;
}
$ clang -Wall -Werror -std=c99 -lm -fno-builtin -o example example.c 
$ ./example
fmin(1.000000, nan) = 1.000000
$
