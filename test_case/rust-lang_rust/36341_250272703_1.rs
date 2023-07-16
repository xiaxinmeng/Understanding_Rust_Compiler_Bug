 c
/* XXX This implementation is considered (to quote Tim Peters) "inherently
   hosed" because:
     - It does not guarantee the promise that a non-zero integer is returned.
     - The cast to long is inherently unsafe.
     - It is not clear that the 'volatile' (for AIX?) are any longer necessary.
*/
long
PyThread_get_thread_ident(void)
{
    volatile pthread_t threadid;
    if (!initialized)
        PyThread_init_thread();
    threadid = pthread_self();
    return (long) threadid;
}
