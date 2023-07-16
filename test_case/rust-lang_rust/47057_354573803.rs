
    int fd = open("/dev/urandom", O_RDONLY);
    if (fd >= 0) {
        // Use strong PRNG based on data from the random device.
    } else {
        // Use weak PRNG based on getpid() and gettimeofday().
    }
    