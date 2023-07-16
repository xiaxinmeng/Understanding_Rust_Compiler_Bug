
void *
__tls_get_addr (tls_index* ti)
{
    dtv_t *dtv = THREAD_DTV ();

    if (__builtin_expect (dtv[0].counter != _rtld_global.dl_tls_generation, 0))
        return update_get_addr (ti);

    void *p = dtv[ti->ti_module].pointer.val;

    if (__builtin_expect (p == TLS_DTV_UNALLOCATED, 0))
        return tls_get_addr_tail (ti, dtv, NULL);

    return (char *) p + ti->ti_offset;
}
