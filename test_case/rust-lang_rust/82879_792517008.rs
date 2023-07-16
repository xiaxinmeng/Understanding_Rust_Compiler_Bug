
$ man signal-safety | rg -F exec
       trant or because it is atomic with respect to signals (i.e., its execu‐
       execl(3)               Added in POSIX.1-2008; see notes below
       execle(3)              See notes below
       execv(3)               Added in POSIX.1-2008
       execve(2)
       fexecve(3)             Added in POSIX.1-2008
       *  If a signal handler interrupts the execution of an unsafe  function,
       *  Before glibc 2.24, execl(3) and execle(3) employed realloc(3) inter
