
glaubitz@pbox-ppc:~$ cat floattest.c 
#include <stdio.h>
#include <math.h>

double add_double (double a, double b) {

        return a + b;
}

int main() {

        double x = 3.4;
        double y = 7.2;

        double z = add_double(x, y);

        printf ("The result of %lf + %lf is %lf:\n", x, y, z);

        return 0;
}
glaubitz@pbox-ppc:~$ clang-4.0 floattest.c -o floatest-llvm
glaubitz@pbox-ppc:~$ ./floatest-llvm 
The result of 3.400000 + 7.200000 is 10.600000:
glaubitz@pbox-ppc:~$
