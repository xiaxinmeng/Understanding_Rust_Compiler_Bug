
       As  an  extension  to the POSIX.1-2001 standard, glibc's getcwd() allocates
       the buffer dynamically using malloc(3) if buf is NULL.  In this  case,  the
       allocated buffer has the length size unless size is zero, when buf is allo‐
       cated as big as necessary.  The caller should free(3) the returned buffer.
