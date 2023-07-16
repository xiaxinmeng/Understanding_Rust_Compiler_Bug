c
> #define _GNU_SOURCE
> 
> #include <err.h>
> #include <sched.h>
> #include <stdio.h>
> #include <stdlib.h>
> #include <unistd.h>
> 
> int child_func(void *ptr_param)
> {
>     unsigned long param = (unsigned long)ptr_param;
>     fprintf(stderr, "child: param=%lu; calling execv\n", param);
>     char *const argv[] = { "echo", "hello from echo executed by the child process", NULL };
>     execv("/bin/echo", argv);
>     exit(1);
> }