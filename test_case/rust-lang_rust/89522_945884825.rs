c
#define _GNU_SOURCE

#include <err.h>
#include <sched.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int child_func(void *ptr_param)
{
    unsigned long param = (unsigned long)ptr_param;
    fprintf(stderr, "child: param=%lu; calling execv\n", param);
    char *const argv[] = { "echo", "hello from echo executed by the child process", NULL };
    execv("/bin/echo", argv);
    exit(1);
}

unsigned char stack[16384];

int main(void)
{
    int pidfd = -1;
    int pid = clone(child_func, stack + 16384 - 16, CLONE_PIDFD, (void *)42UL, &pidfd);
    if (pid == -1)
        err(1, "clone");
    printf("parent: pid=%i pidfd=%i\n", pid, pidfd);
    return 0;
}
