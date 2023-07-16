
#include <assert.h>
#include <stdlib.h>
#include <errno.h>
#include <spawn.h>

extern char **environ;

int
main(void)
{
    pid_t pid;
    char *pargv[] = {"/nonexistent", NULL};
    int status = posix_spawn(&pid, "/nonexistent", NULL, NULL, pargv, environ);
    assert(status == ENOENT);
    return 0;
}
