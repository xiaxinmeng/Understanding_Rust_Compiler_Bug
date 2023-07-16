c
root@86646b35b199:~# cat >test.c
#include <stdio.h>
#include <errno.h>
#include <unistd.h>
#include <sys/syscall.h>

#define SYS_statx 332 // x86_64
#define STATX_ALL 0xFFF

int main (void) {
    char buf[0x100] = {};
    int ret = syscall(SYS_statx, 0, "/", 0, STATX_ALL, buf);
    if (ret == 0)
        puts("ok");
    else {
        int e = errno;
        perror("err");
        printf("errno = %d\n", e);
    }
    return 0;
}
root@86646b35b199:~# gcc test.c
root@86646b35b199:~# ./a.out
ok
