c
int gettimeofday(struct timeval *tv, struct timezone *tz);

struct timezone {
    int tz_minuteswest;     /* minutes west of Greenwich */
    int tz_dsttime;         /* type of DST correction */
};
