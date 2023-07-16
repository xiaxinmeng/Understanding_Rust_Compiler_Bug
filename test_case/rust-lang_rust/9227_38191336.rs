 c
#include<stdio.h>
int main() {
    static int X = 10;
    {
        static bool X = false;
        printf("...");
    }
    printf("...");
}
