c

#include <stdlib.h>
#include <unistd.h>

int main() {
    int fd = open("myfile.txt", O_RDONLY);
    void* mem = malloc(300);
    if (fork()) {
        close(fd); // first "drop" of fd
        free(mem); // first "drop" of mem
    } else {
        close(fd); // second "drop" of fd
        free(mem); // second "drop" of mem
    }
}
