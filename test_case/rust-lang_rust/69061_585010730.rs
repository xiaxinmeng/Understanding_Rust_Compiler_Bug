c
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <string.h>
#include <stddef.h>

int main(int argc, char *argv[])
{
    int ret = 0;

    int fd = socket(AF_UNIX, SOCK_DGRAM, 0);
    if (fd < 0)
    {
        perror("socket");
        return EXIT_FAILURE;
    }

    struct sockaddr_un baddr;
    memset(&baddr, 0, sizeof(baddr));

    const char *BIND_PATH = "/tmp/shadowsocks-manager-c.sock";
    baddr.sun_family = AF_UNIX;
    strcpy(baddr.sun_path, BIND_PATH);

    unlink(BIND_PATH);

    if ((ret = bind(fd, (struct sockaddr *)&baddr, sizeof(baddr))) < 0)
    {
        perror("bind");
        close(fd);
        return EXIT_FAILURE;
    }

    const char buf[] = "ping";

    struct sockaddr_un caddr;
    memset(&caddr, 0, sizeof(caddr));
    caddr.sun_family = AF_UNIX;

    const char *SVR_PATH = "/tmp/shadowsocks-manager.sock";
    strcpy(caddr.sun_path, SVR_PATH);

    caddr.sun_len = SUN_LEN(&caddr);

    socklen_t caddr_len = offsetof(struct sockaddr_un, sun_path) + strlen(caddr.sun_path) + 1;

    ssize_t rcount = sendto(fd, buf, sizeof(buf), 0, (struct sockaddr *)&caddr, caddr_len);
    if (rcount < 0)
    {
        perror("sendto");
        close(fd);
        return EXIT_FAILURE;
    }

    return 0;
}
