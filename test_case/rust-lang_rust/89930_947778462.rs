c
pid_t pid = fork();
if (pid)
         int pidfd = pidfd_open(pid, 0);
