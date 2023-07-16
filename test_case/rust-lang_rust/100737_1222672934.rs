c
sigset_t ss;
pthread_sigmask(SIG_BLOCK, NULL, &ss); // read current thread's signal mask into `ss`
sigaddset(&ss, SIGINT); // for instance
posix_spawnattr_setsigmask(&attr, &ss);
