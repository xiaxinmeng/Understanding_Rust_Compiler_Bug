
     The pthread_rwlock_rdlock(), pthread_rwlock_timedrdlock(), and
     pthread_rwlock_tryrdlock() functions may fail if:

     [EAGAIN]           The lock could not be acquired because the maximum
                        number of read locks against lock has been exceeded.
