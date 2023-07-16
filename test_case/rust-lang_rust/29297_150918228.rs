
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

extern char **environ;

int main(int argc, char **argv) {
    printf("ARGV:\n");
    for(; *argv; argv++) {
        printf("%s\n", *argv);
    }
    printf("\n");
    printf("ENV:\n");
    char **env = environ;
    for(; *env; env++) {
        printf("%s\n", *env);
    }
    printf("\n");

    char *new_argv[] = {
        NULL
    };

    char *new_env[] = {
        "FOOBAR",
        NULL
    };

    if(argc != 0) {
        execve("./a.out", new_argv, new_env);
        perror("execve ./a.out");
    } else {
        char *interesting = getenv("FOOBAR");
        if(interesting) {
            printf("FOOBAR=%s\n", interesting);
        } else {
            printf("(nil)\n");
        }
    }

    return 0;
}
