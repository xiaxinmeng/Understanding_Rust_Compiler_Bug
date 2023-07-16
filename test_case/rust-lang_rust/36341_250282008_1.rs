 c
// freebsd/lib/libthr/thread/thr_equal.c
int
_pthread_equal(pthread_t t1, pthread_t t2)
{
    /* Compare the two thread pointers: */
    return (t1 == t2);
}
