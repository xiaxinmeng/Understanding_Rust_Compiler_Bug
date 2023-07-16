 c
#include <unistd.h>

int main(void) {
    char *const cmd[] = { "ls", "-l", (char *)0 };
    execv("/bin/ls", cmd);
}
