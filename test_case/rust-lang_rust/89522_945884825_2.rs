c
#define _GNU_SOURCE

#include <err.h>
#include <sched.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <syscall.h>
#include <unistd.h>

struct clone_args {
    uint64_t flags;
    uint64_t pidfd;
    uint64_t child_tid;
    uint64_t parent_tid;
    uint64_t exit_signal;
    uint64_t stack;
    uint64_t stack_size;
    uint64_t tls;
    uint64_t set_tid;
    uint64_t set_tid_size;
    uint64_t cgroup;
};

int main(void)
{
    int pidfd = -1;
    struct clone_args clone_args = {
        .flags = CLONE_PIDFD,
        .pidfd = (uint64_t)&pidfd,
    };
    long ret = syscall(SYS_clone3, &clone_args, sizeof(clone_args));
    if (ret == -1)
        err(1, "clone3");
    else if (ret == 0) {
        fprintf(stderr, "child: calling execv\n");
        char *const argv[] = { "echo", "hello from echo executed by the child process", NULL };
        execv("/bin/echo", argv);
        exit(1);
    }
    printf("parent: pid=%ld pidfd=%i\n", ret, pidfd);
    return 0;
}
