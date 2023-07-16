 c
#define _POSIX_C_SOURCE 200112L
#include <stdio.h>
#include <unistd.h>

int main(void) {
    int fds[2];
    if (pipe(fds)) perror("pipe(2)");
    if (fsync(fds[0])) perror("fsync(2)");
    if (fsync(fds[1])) perror("fsync(2)");
    if (fdatasync(fds[0])) perror("fdatasync(2)");
    if (fdatasync(fds[1])) perror("fdatasync(2)");
    return 0;
}
