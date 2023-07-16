
root@atlantis:~> cat floattest.c
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
root@atlantis:~> clang-4.0 floattest.c -I /usr/include/powerpc-linux-gnuspe/ -o floattest-llvm
root@atlantis:~> gcc floattest.c -o floattest-gcc
root@atlantis:~> ./floattest-llvm 
The result of 7.199997 + 7.000000 is 0.000000:
root@atlantis:~> ./floattest-gcc
The result of 3.400000 + 7.200000 is 10.600000:
root@atlantis:~>
