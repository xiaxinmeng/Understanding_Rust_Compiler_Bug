c
#include <stdio.h>
__attribute__((weak)) extern void *posix_spawn_file_actions_addchdir_np;

int main() {
    printf("%p\n", &posix_spawn_file_actions_addchdir_np);
}
