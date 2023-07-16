 c
#include <stdio.h>

enum {
  x = 1,
  y,
  z = 2
};

int main() {

   printf( "%d\n", x );
   printf( "%d\n", y );
   printf( "%d\n", z );

   return 0;
}
