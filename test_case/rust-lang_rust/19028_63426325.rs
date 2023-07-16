 c
#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

int main(void) {
    int fd;

    fd = open("/", 0);
    printf("%d\n", fd); /* => 3 */
    close(fd);

    fclose(stdin);

    fd = open("/", 0);
    printf("%d\n", fd); /* => 0 */
    close(fd);

    return 0;
}
