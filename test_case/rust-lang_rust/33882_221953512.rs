 C++
#include <string.h>
#include <stdio.h>

int main(void)
{
    char input[] = "abc";
    char delim[] = "";
    char *token = strtok(input, delim);
    printf("[");
    while (token) {
        printf("\"%s\"", token);
        token = strtok(NULL, delim);
        if (token) {
            printf(", ");
        }
    }
    printf("]\n");
}
