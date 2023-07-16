
static int
do_posix_spawn(pid_t *pid, const char *path,
    const posix_spawn_file_actions_t *fa,
    const posix_spawnattr_t *sa,
    char * const argv[], char * const envp[], int use_env_path)
{
        pid_t p;
        volatile int error = 0;

        p = vfork();
        switch (p) {
        case -1:
                return (errno);
        case 0:
                if (sa != NULL) {
                        error = process_spawnattr(*sa);
                        if (error)
                                _exit(127);
                }
                if (fa != NULL) {
                        error = process_file_actions(*fa);
                        if (error)
                                _exit(127);
                }
                if (use_env_path)
                        _execvpe(path, argv, envp != NULL ? envp : environ);
                else
                        _execve(path, argv, envp != NULL ? envp : environ);
                error = errno;
                _exit(127);
        default:
                if (error != 0)
                        _waitpid(p, NULL, WNOHANG);
                else if (pid != NULL)
                        *pid = p;
                return (error);
        }
}
